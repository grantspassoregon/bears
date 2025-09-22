use crate::{
    BeaErr, BeaResponse, Data, Dataset, DatasetMissing, Frequencies, Frequency, Integer, IoError,
    JsonParseError, KeyMissing, Naics, NotArray, NotObject, ParameterFields, ParameterName,
    ParameterValueTable, SelectionKind, SerdeJson, Set, VariantMissing, Year, data::result_to_data,
    map_to_float, map_to_int, map_to_string, parse_year, roman_numeral_quarter,
};

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
    frequency: Frequencies,
    // BTreeMap of table ids to industry names
    industry: std::collections::BTreeMap<Integer, Vec<ParameterFields>>,
    table_id: Vec<Integer>,
    // BTreeMap of table ids to years
    year: std::collections::BTreeMap<Integer, Vec<Year>>,
}

impl GdpByIndustry {
    #[tracing::instrument]
    pub fn from_file<P: AsRef<std::path::Path> + std::fmt::Debug>(path: P) -> Result<Self, BeaErr> {
        let frequency = Self::frequencies();
        let industry = Self::read_industry(&path)?;
        let table_id = Self::read_table_id(&path)?;
        let year = Self::read_year(&path)?;
        Ok(Self::new(frequency, industry, table_id, year))
    }

    #[tracing::instrument]
    pub fn frequencies() -> Frequencies {
        vec![Frequency::Annual, Frequency::Quarterly].into()
    }

    #[tracing::instrument]
    pub fn iter(&self) -> GdpByIndustryIterator<'_> {
        GdpByIndustryIterator::new(self)
    }

    #[tracing::instrument]
    pub fn iter_tables(&self) -> GdpTables<'_> {
        GdpTables::new(self)
    }

    // pub fn queue() -> Result<Queue, BeaErr> {
    //     let req = Request::Data;
    //     let mut app = req.init()?;
    //     let dataset = Dataset::GDPbyIndustry;
    //     app.with_dataset(dataset);
    //     dotenvy::dotenv().ok();
    //     let path = bea_data()?;
    //     let data = GdpByIndustry::try_from(&path)?;
    //     let mut queue = Vec::new();
    //     for params in data.iter() {
    //         tracing::trace!("{params:#?}");
    //         app.with_params(params.clone());
    //         queue.push(app.clone());
    //     }
    //     Ok(Queue::new(queue))
    // }

    #[tracing::instrument]
    pub fn read_industry<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<std::collections::BTreeMap<Integer, Vec<ParameterFields>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::GDPbyIndustry;
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
                            industry.push(pf.clone());
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
    ) -> Result<Vec<Integer>, BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::GDPbyIndustry;
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
    ) -> Result<std::collections::BTreeMap<Integer, Vec<Year>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::GDPbyIndustry;
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

impl TryFrom<&std::path::PathBuf> for GdpByIndustry {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::from_file(value)
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, derive_setters::Setters)]
#[setters(prefix = "with_", borrow_self, into)]
pub struct GdpByIndustryIterator<'a> {
    #[setters(skip)]
    data: &'a GdpByIndustry,
    frequency_options: SelectionKind,
    industry_options: SelectionKind,
    year_selection: SelectionKind,
    // index into data.table_name
    #[setters(skip)]
    table_index: usize,
    // index into data.frequency
    #[setters(skip)]
    frequency_index: usize,
    #[setters(skip)]
    frequency_end: bool,
    #[setters(skip)]
    industry_index: usize,
    #[setters(skip)]
    industry_end: bool,
    #[setters(skip)]
    years: Option<Vec<String>>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> GdpByIndustryIterator<'a> {
    #[tracing::instrument]
    pub fn new(data: &'a GdpByIndustry) -> Self {
        let frequency_options = SelectionKind::default();
        let industry_options = SelectionKind::default();
        let year_selection = SelectionKind::default();
        let table_index = 0;
        let frequency_index = 0;
        let industry_index = 0;
        let industry_end = false;
        Self {
            data,
            frequency_options,
            industry_options,
            year_selection,
            table_index,
            frequency_index,
            frequency_end: false,
            industry_index,
            industry_end,
            years: None,
            year_index: 0,
            year_end: false,
        }
    }
}

impl Iterator for GdpByIndustryIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // advance state
        if self.year_end {
            // no more year values in self.years
            // only needs to reset for individual, but doesn't hurt to just reset
            self.year_index = 0;
            self.year_end = false;
            match self.frequency_options {
                SelectionKind::All => self.frequency_end = true,
                SelectionKind::Individual => {
                    // more frequencies remain
                    if self.frequency_index < self.data.frequency.len() - 1 {
                        // increment the frequency index
                        self.frequency_index += 1;
                    } else {
                        // no more frequencies, move to next table
                        self.frequency_end = true;
                    }
                }
                // TODO: unimplemented
                SelectionKind::Multiple => {}
            }
        }

        // no more years for this table, move to next table
        if self.frequency_end {
            self.frequency_index = 0;
            self.frequency_end = false;
            let table_id = &self.data.table_id[self.table_index];
            if let Some(industries) = self.data.industry.get(table_id) {
                if self.industry_index < industries.len() - 1 {
                    // increment the index
                    self.industry_index += 1;
                } else {
                    self.industry_end = true;
                }
            } else {
                tracing::error!("Industries should not be None.");
                self.industry_end = true;
            }
        }

        // no more years for this table, move to next table
        if self.industry_end {
            self.industry_index = 0;
            self.industry_end = false;
            if self.table_index < self.data.table_id.len() - 1 {
                // increment the table index
                self.table_index += 1;
            } else {
                // no more tables, end iterator
                return None;
            }
        }

        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();
        // set table id
        let key = ParameterName::TableID.to_string();
        let table_id = self.data.table_id[self.table_index].clone();
        params.insert(key, table_id.to_string());

        // set frequency
        match self.frequency_options {
            // only a single key value pair needed
            SelectionKind::All => {
                let (key, value) = self.data.frequency.params();
                params.insert(key, value);
            }
            // step through the available frequencies
            SelectionKind::Individual => {
                let (key, value) = self.data.frequency[self.frequency_index].params();
                params.insert(key, value);
            }
            // TODO: unimplemented
            SelectionKind::Multiple => {}
        }

        // set industry
        let key = ParameterName::Industry.to_string();
        match self.industry_options {
            SelectionKind::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
                // move to next set
                self.industry_end = true;
            }
            SelectionKind::Individual => {
                if let Some(industries) = self.data.industry.get(&table_id) {
                    params.insert(key, industries[self.industry_index].key().to_owned());
                } else {
                    tracing::warn!("Industry should not be None.");
                    // move to next set
                    self.industry_end = true;
                }
            }
            // TODO: unimplemented
            SelectionKind::Multiple => {}
        }

        // set year values
        let key = ParameterName::Year.to_string();
        match self.year_selection {
            // single key value pair is sufficient
            SelectionKind::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
                // move to next year range or table
                self.year_end = true;
            }
            // pull next year from years
            SelectionKind::Individual => {
                if let Some(years) = self.data.year.get(&table_id) {
                    params.insert(key, years[self.year_index].to_string());
                    // advance state
                    if self.year_index < years.len() - 1 {
                        // increment year index
                        self.year_index += 1;
                    } else {
                        // move to next year range or table
                        self.year_end = true;
                    }
                } else {
                    tracing::warn!("Years should not be None.");
                    // move to next year range or table
                    self.year_end = true;
                }
            }
            // TODO: unimplemented
            SelectionKind::Multiple => {}
        }
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
    data_value: f64,
    frequency: Frequency,
    industry_description: String,
    industry: Naics,
    note_ref: String,
    quarter: jiff::civil::Date,
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
        let quarter = map_to_string("Quarter", m)?;
        let quarter = if let Ok(date) = parse_year(&quarter) {
            date
        } else if let Some(date) = roman_numeral_quarter(&quarter, year) {
            date
        } else {
            let error = KeyMissing::new("Quarter".to_owned(), line!(), file!().to_owned());
            let error = JsonParseError::from(error);
            return Err(error.into());
        };
        tracing::trace!("Quarter: {quarter}.");
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
    #[tracing::instrument]
    pub fn frequencies(&self) -> std::collections::BTreeSet<Frequency> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.frequency().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn industries(&self) -> std::collections::BTreeSet<Naics> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.industry().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
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

    #[tracing::instrument]
    pub fn table_ids(&self) -> std::collections::BTreeSet<i64> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(*v.table_id()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
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
pub struct UnderlyingGdpByIndustry {
    frequency: Frequencies,
    industry: std::collections::HashMap<Integer, Vec<ParameterFields>>,
    table_id: Vec<Integer>,
    year: std::collections::HashMap<Integer, Vec<Year>>,
}

impl UnderlyingGdpByIndustry {
    #[tracing::instrument]
    pub fn iter(&self) -> UnderlyingGDPbyIndustryIterator<'_> {
        UnderlyingGDPbyIndustryIterator::new(self)
    }

    /// Primary creation method, called `from_file` instead of `new` because it requires reference to
    /// a path.
    #[tracing::instrument]
    pub fn from_file<P: AsRef<std::path::Path> + std::fmt::Debug>(path: P) -> Result<Self, BeaErr> {
        let frequency = Self::frequencies();
        let industry = Self::read_industry(&path)?;
        tracing::info!("Industries read at {}.", path.as_ref().display());
        let table_id = Self::read_table_id(&path)?;
        tracing::info!("Table IDs read at {}.", path.as_ref().display());
        let year = Self::read_year(&path)?;
        tracing::info!("Years read at {}.", path.as_ref().display());
        Ok(Self::new(frequency, industry, table_id, year))
    }

    #[tracing::instrument]
    pub fn frequencies() -> Frequencies {
        vec![Frequency::Annual].into()
    }

    #[tracing::instrument]
    pub fn read_industry<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<std::collections::HashMap<Integer, Vec<ParameterFields>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::UnderlyingGDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Industry;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut industries = std::collections::HashMap::new();
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
                            industry.push(pf.clone());
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
    ) -> Result<Vec<Integer>, BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::UnderlyingGDPbyIndustry;
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

    // TODO: fix the redundant call to read_table_id
    #[tracing::instrument]
    pub fn read_year<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<std::collections::HashMap<Integer, Vec<Year>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::UnderlyingGDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Year;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut years = std::collections::HashMap::new();
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

impl TryFrom<&std::path::PathBuf> for UnderlyingGdpByIndustry {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::from_file(value)
    }
}

pub struct UnderlyingGDPbyIndustryIterator<'a> {
    table_id: std::slice::Iter<'a, Integer>,
}

impl<'a> UnderlyingGDPbyIndustryIterator<'a> {
    #[tracing::instrument]
    pub fn new(data: &'a UnderlyingGdpByIndustry) -> Self {
        let table_id = data.table_id().iter();
        Self { table_id }
    }
}

impl Iterator for UnderlyingGDPbyIndustryIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // advance state
        // let current_id = if let Some(id) = &self.current_table {
        //     id.value().to_string()
        // } else {
        //     let current_id = self.table_id.next()?;
        //     self.current_table = Some(current_id.clone());
        //     if let Some(fields) = self.data.industry.get(current_id) {
        //         self.industries = Some(fields.iter());
        //     }
        //     current_id.value().to_string()
        // };

        // set industry
        // let industry = if let Some(industries) = &mut self.industries {
        //     match industries.next() {
        //         Some(value) => value.key(),
        //         None => {
        //             self.current_table = None;
        //             return self.next();
        //         }
        //     }
        // } else {
        //     tracing::error!("Industry should not start as None");
        //     return None;
        // };
        let key = ParameterName::Industry.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        // set table_id
        let current_id = self.table_id.next()?.value().to_string();
        let key = ParameterName::TableID.to_string();
        params.insert(key, current_id);

        // set Frequency to all
        // UnderlyingGDPbyIndustry only has Annual "A" data available
        let key = ParameterName::Frequency.to_string();
        let value = "A".to_owned();
        params.insert(key, value);

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        Some(params)
    }
}

/// Return value format associated with the
/// [`Dataset::UnderlyingGDPbyIndustry`](crate::Dataset::UnderlyingGDPbyIndustry) variant.
#[derive(
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
)]
pub struct UnderlyingGdpDatum {
    data_value: f64,
    frequency: Frequency,
    industry_description: String,
    industry: Naics,
    note_ref: String,
    table_id: i64,
    year: jiff::civil::Date,
}

impl UnderlyingGdpDatum {
    /// Attempts to map a [`serde_json::Map`] `m` to an instance of `UnderlyingGpdDatum`.
    ///
    /// Encapsulates the logic of retrieving the `UnderlyingGpdDatum` when converting the JSON representation
    /// into internal data types.  Used to implement the [`TryFrom`] trait from [`serde_json::Value`] to `self`.
    #[tracing::instrument]
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        tracing::trace!("Reading MneDiDatum.");
        let data_value = map_to_float("DataValue", m)?;
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
        Ok(Self {
            data_value,
            frequency,
            industry_description,
            industry,
            note_ref,
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

/// `UnderlyingGdpData` represents the `Data` portion of the `Results` from a BEA response.
/// These portions of the response have corresponding internal library representations, [`Data`],
/// [`Results`](crate::Results) and [`BeaResponse`] respectively.  This is the data type contained
/// within the [`Data::UnderlyingGdpData`] variant.
///
/// Functionally, `UnderlyingGdpData` is a thin wrapper around a vector of type [`UnderlyingGdpDatum`].
/// This type implements [`TryFrom`] for `&PathBuf`, which I suppose is what Path is for, but I'm
/// still trying to figure out the idiom here.
/// This type also implements [`TryFrom`] for references to a [`serde_json::Value`].  Given a
/// reference to a path, the impl for try_from will then call the impl associated with the
/// [`serde_json::Value`], which calls [`UnderlyingGdpDatum::read_json`], bubbling up any errors.
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
#[from(Vec<UnderlyingGdpDatum>)]
pub struct UnderlyingGdpData(Vec<UnderlyingGdpDatum>);

impl UnderlyingGdpData {
    #[tracing::instrument]
    pub fn frequencies(&self) -> std::collections::BTreeSet<Frequency> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.frequency().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn industries(&self) -> std::collections::BTreeSet<Naics> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.industry().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn table_ids(&self) -> std::collections::BTreeSet<i64> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(*v.table_id()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn years(&self) -> std::collections::BTreeSet<jiff::civil::Date> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.year().to_owned()))
            .for_each(drop);
        set
    }
}

impl TryFrom<&std::path::PathBuf> for UnderlyingGdpData {
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
                Data::UnderlyingGdp(value) => {
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

impl TryFrom<&serde_json::Value> for UnderlyingGdpData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading GdpData");
        match result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = UnderlyingGdpDatum::read_json(m)?;
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

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
    derive_new::new,
)]
pub struct GdpCodes {
    frequencies: std::collections::BTreeSet<Frequency>,
    industries: std::collections::BTreeSet<Naics>,
    table_ids: std::collections::BTreeSet<i64>,
    years: std::collections::BTreeSet<jiff::civil::Date>,
}
