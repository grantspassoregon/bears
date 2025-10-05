use crate::{
    BeaErr, BeaResponse, Currency, Data, Dataset, DatasetMissing, Frequencies, Frequency, Integer,
    IoError, KeyMissing, Measure, Naics, NotArray, NotObject, ParameterName, ParameterValueTable,
    Scale, SerdeJson, Set, VariantMissing, Year, data::result_to_data, map_to_float, map_to_int,
    map_to_string, parse_year, roman_numeral_quarter,
};
use strum::IntoEnumIterator;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
    derive_new::new,
)]
pub struct GdpByIndustry {
    dataset: Dataset,
    frequency: Frequencies,
    // BTreeMap of table ids to industry names
    industry: std::collections::BTreeMap<Integer, Vec<Naics>>,
    table_id: Vec<Integer>,
    // BTreeMap of table ids to years
    year: std::collections::BTreeMap<Integer, Vec<Year>>,
}

impl GdpByIndustry {
    /// Hardcoded vector of variants, skipping the need to read from file.
    #[tracing::instrument]
    pub fn frequencies(dataset: Dataset) -> Frequencies {
        let freqs = match dataset {
            Dataset::GDPbyIndustry => vec![Frequency::Annual, Frequency::Quarterly],
            Dataset::UnderlyingGDPbyIndustry => vec![Frequency::Annual],
            _ => Frequency::iter().collect::<Vec<Frequency>>(),
        };
        Frequencies::from(freqs)
    }

    /// Returns the set of all [`Naics`] variants contained in the *industry* field.
    /// Takes the union of all variants under various table ids.
    #[tracing::instrument(skip_all)]
    pub fn industries(&self) -> std::collections::BTreeSet<Naics> {
        let mut set = std::collections::BTreeSet::new();
        for items in self.industry.values() {
            let mut add = items
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<Naics>>();
            set.append(&mut add);
        }
        set
    }

    /// Returns the set of all i32 values contained in the *table_id* field.
    #[tracing::instrument(skip_all)]
    pub fn table_ids(&self) -> std::collections::BTreeSet<i64> {
        self.table_id()
            .iter()
            .map(|v| *v.value())
            .collect::<std::collections::BTreeSet<i64>>()
    }

    /// Returns the set of all [`Year`] values contained in the *year* field.
    /// Takes the union of all variants under various table ids and converts them to
    /// [`jiff::civil::Date`].
    #[tracing::instrument(skip_all)]
    pub fn years(&self) -> std::collections::BTreeSet<jiff::civil::Date> {
        let mut set = std::collections::BTreeSet::new();
        for items in self.year.values() {
            let mut add = items
                .iter()
                .map(|v| *v.date())
                .collect::<std::collections::BTreeSet<jiff::civil::Date>>();
            set.append(&mut add);
        }
        set
    }

    #[tracing::instrument(skip_all)]
    pub fn iter_tables(&self) -> GdpTables<'_> {
        GdpTables::new(self)
    }

    #[tracing::instrument]
    pub fn from_file<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
        dataset: Dataset,
    ) -> Result<Self, BeaErr> {
        let frequency = Self::frequencies(dataset);
        let industry = Self::read_industry(&path, dataset)?;
        let table_id = Self::read_table_id(&path, dataset)?;
        let year = Self::read_year(&path, dataset)?;
        Ok(Self::new(dataset, frequency, industry, table_id, year))
    }

    #[tracing::instrument]
    pub fn read_industry<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
        dataset: Dataset,
    ) -> Result<std::collections::BTreeMap<Integer, Vec<Naics>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path, dataset)?;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Industry;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut industries = std::collections::BTreeMap::new();
        for id in table_id {
            // open the file at the expected storage location, error if missing
            let path = path.join(format!(
                "{dataset}_{name}_byTableId_{}_values.json",
                id.value()
            ));
            let file = std::fs::File::open(&path)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            let mut industry = Vec::new();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                for table in pv.iter() {
                    match table {
                        ParameterValueTable::ParameterFields(pf) => {
                            if let Some(naics) = Naics::from_code(pf.key()) {
                                industry.push(naics);
                            } else {
                                let error = VariantMissing::new(
                                    "Naics".to_string(),
                                    pf.key().to_owned(),
                                    line!(),
                                    file!().to_owned(),
                                );
                                return Err(error.into());
                            }
                            // industry.push(pf.clone());
                        }
                        _ => {
                            return Err(Set::ParameterFieldsMissing.into());
                        }
                    }
                }
                tracing::trace!("{dataset} contains {} {name} values.", industry.len());
                industries.insert(id, industry);
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        Ok(industries)
    }

    #[tracing::instrument]
    pub fn read_table_id<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
        dataset: Dataset,
    ) -> Result<Vec<Integer>, BeaErr> {
        let path = path.as_ref();
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::TableID;
        // open the file at the expected storage location, error if missing
        let path = path.join(format!(
            "parameter_values/{dataset}_{name}_parameter_values.json"
        ));
        let file = std::fs::File::open(&path)
            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
        // read the file to json
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        // parse to internal bea response format
        let data = BeaResponse::try_from(&res)?;
        let results = data.results();

        let mut table_id = Vec::new();
        // access parameter values from response
        if let Some(pv) = results.into_parameter_values() {
            for table in pv.iter() {
                table_id.push(Integer::try_from(table)?);
            }
            tracing::trace!("{dataset} contains {} {name} values.", table_id.len());
            Ok(table_id)
        } else {
            tracing::warn!("Results must be of type ParameterValues");
            Err(Set::ParameterValuesMissing.into())
        }
    }

    #[tracing::instrument]
    pub fn read_year<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
        dataset: Dataset,
    ) -> Result<std::collections::BTreeMap<Integer, Vec<Year>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path, dataset)?;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Year;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut years = std::collections::BTreeMap::new();
        for id in table_id {
            // open the file at the expected storage location, error if missing
            let path = path.join(format!(
                "{dataset}_{name}_byTableId_{}_values.json",
                id.value()
            ));
            let file = std::fs::File::open(&path)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            let mut year = Vec::new();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                for table in pv.iter() {
                    year.push(Year::try_from(table)?);
                }
                tracing::trace!("{dataset} contains {} {name} values.", year.len());
                years.insert(id, year);
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        Ok(years)
    }
}

impl<P: AsRef<std::path::Path>> TryFrom<(P, Dataset)> for GdpByIndustry {
    type Error = BeaErr;
    fn try_from(value: (P, Dataset)) -> Result<Self, Self::Error> {
        let (path, dataset) = value;
        let path = path.as_ref();
        Self::from_file(path, dataset)
    }
}

/// Returns an iterator over table ids in the `GdpByIndustry` struct.
/// Used to create API calls with Industry, Frequency and Years set to "ALL".
#[derive(Debug, Clone)]
pub struct GdpTables<'a> {
    tables: std::slice::Iter<'a, Integer>,
}

impl<'a> GdpTables<'a> {
    /// Creates an iterator over table ids in the provided `GdpByIndustry` struct.
    pub fn new(data: &'a GdpByIndustry) -> Self {
        let tables = data.table_id().iter();
        Self { tables }
    }
}

impl Iterator for GdpTables<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // set type of investment
        let tables = self.tables.next()?;
        let key = ParameterName::TableID.to_string();
        let value = tables.value().to_string();
        params.insert(key, value);

        // set industry
        let key = ParameterName::Industry.to_string();
        params.insert(key, "ALL".to_string());

        // set frequency
        let key = ParameterName::Frequency.to_string();
        params.insert(key, "A,Q".to_string());

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        Some(params)
    }
}

/// Return value format associated with the
/// [`Dataset::GDPbyIndustry`](crate::Dataset::GDPbyIndustry) variant.
#[derive(
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
)]
pub struct GdpDatum {
    data_value: Currency,
    frequency: Frequency,
    industry_description: String,
    industry: Naics,
    note_ref: String,
    quarter: Option<jiff::civil::Date>,
    table_id: i64,
    year: jiff::civil::Date,
}

impl GdpDatum {
    /// Attempts to map a [`serde_json::Map`] `m` to an instance of `GpdDatum`.
    ///
    /// Encapsulates the logic of retrieving the `GpdDatum` when converting the JSON representation
    /// into internal data types.  Used to implement the [`TryFrom`] trait from [`serde_json::Value`] to `self`.
    #[tracing::instrument]
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        tracing::trace!("Reading MneDiDatum.");
        let data_value = map_to_float("DataValue", m)?;
        let measure = Measure::Usd;
        let scale = Scale::Million;
        let data_value = Currency::from((data_value, scale, measure));
        tracing::trace!("Data Value: {data_value}.");
        let frequency = map_to_string("Frequency", m)?;
        let frequency = Frequency::from_value(&frequency)?;
        tracing::trace!("Frequency: {}.", frequency.value());
        let industry_description = map_to_string("IndustrYDescription", m)?;
        tracing::trace!("Industry Description: {industry_description}.");
        let industry = map_to_string("Industry", m)?;
        let industry = if let Some(naics) = Naics::from_code(&industry) {
            naics
        } else {
            let error =
                VariantMissing::new(industry.clone(), industry, line!(), file!().to_string());
            return Err(error.into());
        };
        tracing::trace!("Industry: {industry:?}.");
        let note_ref = map_to_string("NoteRef", m)?;
        tracing::trace!("Note Ref: {note_ref}.");
        let table_id = map_to_int("TableID", m)?;
        // let table_id = RowCode::from_value(m, &row, naics)?;
        tracing::trace!("Note Ref: {note_ref}.");
        let year = map_to_string("Year", m)?;
        let year = parse_year(&year)?;
        tracing::trace!("Year: {year}.");
        let quarter_str = map_to_string("Quarter", m).ok();
        let quarter = if let Some(value) = quarter_str {
            if let Ok(date) = parse_year(&value) {
                Some(date)
            } else if let Some(date) = roman_numeral_quarter(&value, year) {
                Some(date)
            } else {
                let error = KeyMissing::new("Quarter".to_owned(), line!(), file!().to_owned());
                return Err(error.into());
            }
        } else {
            None
        };
        tracing::trace!("Quarter: {quarter:?}.");
        Ok(Self {
            data_value,
            frequency,
            industry_description,
            industry,
            note_ref,
            quarter,
            table_id,
            year,
        })
    }

    #[tracing::instrument]
    pub fn to_industry(&self) -> (String, String) {
        (
            self.industry().code(),
            self.industry_description().to_owned(),
        )
    }
}

/// `GdpData` represents the `Data` portion of the `Results` from a BEA response.
/// These portions of the response have corresponding internal library representations, [`Data`],
/// [`Results`](crate::Results) and [`BeaResponse`] respectively.  This is the data type contained
/// within the [`Data::GdpData`] variant.
///
/// Functionally, `GdpData` is a thin wrapper around a vector of type [`GdpDatum`].
/// This type implements [`TryFrom`] for `&PathBuf`, which I suppose is what Path is for, but I'm
/// still trying to figure out the idiom here.
/// This type also implements [`TryFrom`] for references to a [`serde_json::Value`].  Given a
/// reference to a path, the impl for try_from will then call the impl associated with the
/// [`serde_json::Value`], which calls [`GdpDatum::read_json`], bubbling up any errors.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::AsRef,
    derive_more::AsMut,
)]
#[from(Vec<GdpDatum>)]
pub struct GdpData(Vec<GdpDatum>);

impl GdpData {
    #[tracing::instrument(skip_all)]
    pub fn frequencies(&self) -> std::collections::BTreeSet<Frequency> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.frequency().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument(skip_all)]
    pub fn industries(&self) -> std::collections::BTreeSet<Naics> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.industry().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument(skip_all)]
    pub fn industry_codes(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        self.iter()
            .map(|v| {
                let (key, value) = v.to_industry();
                params.insert(key, value);
            })
            .for_each(drop);
        params
    }

    #[tracing::instrument(skip_all)]
    pub fn quarters(&self) -> Option<std::collections::BTreeSet<jiff::civil::Date>> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| {
                if let Some(quarter) = v.quarter() {
                    set.insert(quarter.to_owned());
                }
            })
            .for_each(drop);
        if set.is_empty() { None } else { Some(set) }
    }

    #[tracing::instrument(skip_all)]
    pub fn table_ids(&self) -> std::collections::BTreeSet<i64> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(*v.table_id()))
            .for_each(drop);
        set
    }

    #[tracing::instrument(skip_all)]
    pub fn years(&self) -> std::collections::BTreeSet<jiff::civil::Date> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.year().to_owned()))
            .for_each(drop);
        set
    }
}

impl TryFrom<&std::path::PathBuf> for GdpData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::trace!("Response read.");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::Gdp(value) => {
                    tracing::trace!("{} GdpData records read.", value.len());
                    Ok(value)
                }
                _ => {
                    let error =
                        DatasetMissing::new("GdpData".to_string(), line!(), file!().to_string());
                    Err(error.into())
                }
            }
        } else {
            tracing::warn!("Data variant missing.");
            let error = VariantMissing::new(
                "Data variant missing".to_string(),
                "Results".to_string(),
                line!(),
                file!().to_string(),
            );
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for GdpData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading GdpData");
        match result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = GdpDatum::read_json(m)?;
                            data.push(datum);
                        }
                        _ => {
                            let error = NotObject::new(line!(), file!().to_string());
                            return Err(error.into());
                        }
                    }
                }
                tracing::trace!("Data found: {} records.", data.len());
                Ok(Self(data))
            }
            _ => {
                let error = NotArray::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}
