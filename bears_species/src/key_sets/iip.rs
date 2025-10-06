use crate::{
    BeaErr, BeaResponse, Component, Currency, Data, Dataset, DatasetMissing, DeriveFromStr,
    Investment, IoError, ItaFrequencies, ItaFrequency, Measure, NotArray, NotObject, Note, Notes,
    ParameterName, ParameterValueTable, Scale, SerdeJson, Set, VariantMissing, Year,
    date_by_period, map_to_int, map_to_string, parse_year,
};
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
)]
pub struct Iip {
    component: Vec<Component>,
    frequency: ItaFrequencies,
    type_of_investment: Vec<Investment>,
    year: Vec<Year>,
}

impl Iip {
    /// Uses an iterator over investments in the `Iip` data to produce a series of API calls with
    /// Component, Frequency and Year set to "ALL", intended to download the complete dataset.
    pub fn iter_investments(&self) -> IipInvestments<'_> {
        IipInvestments::new(self)
    }

    #[tracing::instrument(skip_all)]
    pub fn components(&self) -> std::collections::BTreeSet<Component> {
        self.component()
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<Component>>()
    }

    #[tracing::instrument(skip_all)]
    pub fn frequencies(&self) -> std::collections::BTreeSet<ItaFrequency> {
        self.frequency()
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<ItaFrequency>>()
    }

    #[tracing::instrument(skip_all)]
    pub fn investments(&self) -> std::collections::BTreeSet<Investment> {
        self.type_of_investment()
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<Investment>>()
    }

    #[tracing::instrument(skip_all)]
    pub fn years(&self) -> std::collections::BTreeSet<jiff::civil::Date> {
        self.year()
            .iter()
            .map(|v| v.date())
            .cloned()
            .collect::<std::collections::BTreeSet<jiff::civil::Date>>()
    }
}

impl TryFrom<&std::path::PathBuf> for Iip {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::Iip;
        let names = dataset.names();
        // empty vectors to store values
        let mut component = Vec::new();
        let mut frequency = Vec::new();
        let mut type_of_investment = Vec::new();
        let mut year = Vec::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!("parameter_values/{dataset}_{name}_values.json"));
            let file = std::fs::File::open(&path)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            // access parameter values from response
            if let Some(pf) = results.into_parameter_values() {
                // type of vector varies by parameter name
                match name {
                    ParameterName::Component => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    let c = Component::from_str(pf.key()).map_err(|e| {
                                        DeriveFromStr::new(
                                            pf.key().to_owned(),
                                            e,
                                            line!(),
                                            file!().to_owned(),
                                        )
                                    })?;
                                    component.push(c);
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::Frequency => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(_) => {
                                    frequency.push(ItaFrequency::try_from(table)?);
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::TypeOfInvestment => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    let i = Investment::from_str(pf.key()).map_err(|e| {
                                        DeriveFromStr::new(
                                            pf.key().to_owned(),
                                            e,
                                            line!(),
                                            file!().to_owned(),
                                        )
                                    })?;
                                    type_of_investment.push(i);
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::Year => {
                        for table in pf.iter() {
                            year.push(Year::try_from(table)?);
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if component.is_empty()
            || frequency.is_empty()
            || type_of_investment.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let frequency = ItaFrequencies::new(frequency);
            let table = Self {
                component,
                frequency,
                type_of_investment,
                year,
            };
            Ok(table)
        }
    }
}

/// Returns an iterator over investments in the `Iip` struct.
/// Used to create API calls with Component, Frequency and Years set to "ALL".
#[derive(Debug, Clone)]
pub struct IipInvestments<'a> {
    investments: std::slice::Iter<'a, Investment>,
}

impl<'a> IipInvestments<'a> {
    /// Creates an iterator over investments in the provided `Iip` struct.
    pub fn new(data: &'a Iip) -> Self {
        let investments = data.type_of_investment().iter();
        Self { investments }
    }
}

impl Iterator for IipInvestments<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // set type of investment
        let investments = self.investments.next()?;
        let (key, value) = investments.params();
        params.insert(key, value);

        // set component
        let key = ParameterName::Component.to_string();
        params.insert(key, "ALL".to_string());

        // set frequency
        let key = ParameterName::Frequency.to_string();
        params.insert(key, "ALL".to_string());

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        Some(params)
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
)]
pub struct IipDatum {
    cl_unit: Measure,
    component: Component,
    data_value: Option<Currency>,
    frequency: ItaFrequency,
    note_ref: Option<String>,
    time_period: jiff::civil::Date,
    time_series_description: String,
    time_series_id: String,
    type_of_investment: Investment,
    unit_mult: Scale,
    year: jiff::civil::Date,
}

impl IipDatum {
    #[tracing::instrument]
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let cl_unit = map_to_string("CL_UNIT", m)?;
        let cl_unit = Measure::from_str(&cl_unit)
            .map_err(|e| DeriveFromStr::new(cl_unit, e, line!(), file!().to_owned()))?;
        tracing::trace!("cl_unit is {cl_unit}.");
        let component = map_to_string("Component", m)?;
        let component = Component::from_str(&component)
            .map_err(|e| DeriveFromStr::new(component, e, line!(), file!().to_string()))?;
        tracing::trace!("component is {component}.");
        let frequency = map_to_string("Frequency", m)?;
        let frequency = ItaFrequency::from_value(&frequency)?;
        tracing::trace!("frequency is {frequency}.");
        let note_ref = map_to_string("NoteRef", m)?;
        let note_ref = if note_ref.is_empty() {
            None
        } else {
            Some(note_ref)
        };
        tracing::trace!("note_ref is {note_ref:?}.");
        let time_period = map_to_string("TimePeriod", m)?;
        let time_period = date_by_period(&time_period)?;
        tracing::trace!("time_period is {time_period}.");
        let time_series_description = map_to_string("TimeSeriesDescription", m)?;
        tracing::trace!("time_series_description is {time_series_description}.");
        let time_series_id = map_to_string("TimeSeriesId", m)?;
        tracing::trace!("time_series_id is {time_series_id}.");
        let type_of_investment = map_to_string("TypeOfInvestment", m)?;
        let type_of_investment = Investment::from_str(&type_of_investment)
            .map_err(|e| DeriveFromStr::new(type_of_investment, e, line!(), file!().to_string()))?;
        tracing::trace!("type_of_investment is {type_of_investment}.");
        let unit_mult = map_to_int("UNIT_MULT", m)?;
        let unit_mult = Scale::from_key(unit_mult)?;
        tracing::trace!("unit_mult is {unit_mult:?}.");
        let year = map_to_string("Year", m)?;
        let year = parse_year(&year)?;
        tracing::trace!("year is {year}.");
        let data_value = map_to_string("DataValue", m)?;
        let data_value = if data_value.is_empty() {
            None
        } else {
            let raw = map_to_int("DataValue", m)?;
            let currency = Currency::from((raw, unit_mult, cl_unit));
            Some(currency)
        };
        tracing::trace!("data_value is {data_value:?}.");
        Ok(Self {
            cl_unit,
            component,
            data_value,
            frequency,
            note_ref,
            time_period,
            time_series_description,
            time_series_id,
            type_of_investment,
            unit_mult,
            year,
        })
    }

    /// Returns the time series id and description as a tuple.
    #[tracing::instrument]
    pub fn time_series_code(&self) -> (String, String) {
        (
            self.time_series_id().to_owned(),
            self.time_series_description().to_owned(),
        )
    }
}

impl TryFrom<serde_json::Value> for IipDatum {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading IipDatum.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct IipData {
    #[deref]
    #[deref_mut]
    data: Vec<IipDatum>,
    notes: Option<Notes>,
}

impl IipData {
    #[tracing::instrument]
    pub fn cl_units(&self) -> std::collections::BTreeSet<Measure> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.cl_unit().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn components(&self) -> std::collections::BTreeSet<Component> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.component().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn frequencies(&self) -> std::collections::BTreeSet<ItaFrequency> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.frequency().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument(skip_all)]
    pub fn notes(&self) -> Option<std::collections::BTreeSet<Note>> {
        self.notes.as_ref().map(|v| v.set())
    }

    #[tracing::instrument]
    pub fn time_periods(&self) -> std::collections::BTreeSet<jiff::civil::Date> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.time_period().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn time_series_codes(&self) -> std::collections::BTreeMap<String, String> {
        let mut codes = std::collections::BTreeMap::new();
        self.iter()
            .map(|v| {
                codes.insert(
                    v.time_series_id().to_owned(),
                    v.time_series_description().to_owned(),
                )
            })
            .for_each(drop);
        codes
    }

    #[tracing::instrument]
    pub fn investments(&self) -> std::collections::BTreeSet<Investment> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.type_of_investment().to_owned()))
            .for_each(drop);
        set
    }

    #[tracing::instrument]
    pub fn unit_multipliers(&self) -> std::collections::BTreeSet<Scale> {
        let mut set = std::collections::BTreeSet::new();
        self.iter()
            .map(|v| set.insert(v.unit_mult().to_owned()))
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

impl TryFrom<&std::path::PathBuf> for IipData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::trace!("Response read.");
        tracing::trace!("Response: {data:#?}");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::Iip(iip) => {
                    tracing::trace!("{} IIP records read.", iip.len());
                    Ok(iip)
                }
                _ => {
                    tracing::warn!("Not IIP variant.");
                    let error = DatasetMissing::new(
                        "IIP variant needed".to_string(),
                        line!(),
                        file!().to_string(),
                    );
                    Err(error.into())
                }
            }
        } else {
            let clue = "Data variant missing, expected IIP results".to_string();
            tracing::warn!("{clue}");
            let error =
                VariantMissing::new(clue, "Results".to_string(), line!(), file!().to_string());
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for IipData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading ItaData");
        match crate::data::result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = IipDatum::read_json(m)?;
                            data.push(datum);
                        }
                        _ => {
                            let error = NotObject::new(line!(), file!().to_string());
                            return Err(error.into());
                        }
                    }
                }
                let notes = Notes::try_from(value).ok();
                tracing::trace!("Data found: {} records.", data.len());
                Ok(Self::new(data, notes))
            }
            _ => {
                let error = NotArray::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}
