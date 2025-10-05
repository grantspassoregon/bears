use crate::{
    BeaErr, Dataset, DeriveFromStr, Jiff, JsonParseError, KeyMissing, NotArray, NotObject,
    map_to_string,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum ParameterValueKind {
    Dataset,
    DatasetDescription,
    Desc,
    Description,
    FirstAnnualYear,
    FirstMonthlyYear,
    FirstQuarterlyYear,
    FrequencyID,
    JsonUpdateDate,
    Key,
    LastAnnualYear,
    LastMonthlyYear,
    LastQuarterlyYear,
    ShowMillionsID,
    TableName,
    TableNumber,
    TargetValue,
    XmlUpdateDate,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct NipaFrequency {
    description: String,
    frequency_id: String,
}

impl NipaFrequency {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        use ParameterValueKind as pvk;
        let description = map_to_string(&pvk::Description.to_string(), m)?;
        let frequency_id = map_to_string(&pvk::FrequencyID.to_string(), m)?;
        Ok(Self {
            description,
            frequency_id,
        })
    }
}

impl TryFrom<serde_json::Value> for NipaFrequency {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading NipaFrequency.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
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
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct NipaShowMillions {
    description: String,
    #[serde(rename = "ShowMillionsID")]
    show_millions_id: String,
}

impl NipaShowMillions {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        use ParameterValueKind as pvk;
        let description = map_to_string(&pvk::Description.to_string(), m)?;
        let show_millions_id = map_to_string(&pvk::ShowMillionsID.to_string(), m)?;
        Ok(Self {
            description,
            show_millions_id,
        })
    }
}

impl TryFrom<serde_json::Value> for NipaShowMillions {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading NipaShowMillions.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
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
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct NipaTable {
    description: String,
    table_name: String,
}

impl NipaTable {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        use ParameterValueKind as pvk;
        let description = map_to_string(&pvk::Description.to_string(), m)?;
        let table_name = map_to_string(&pvk::TableName.to_string(), m)?;
        Ok(Self {
            description,
            table_name,
        })
    }
}

impl TryFrom<serde_json::Value> for NipaTable {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading NipaTable.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
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
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "PascalCase")]
pub struct NipaTableNumber {
    description: String,
    table_number: String,
}

impl NipaTableNumber {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        use ParameterValueKind as pvk;
        let description = map_to_string(&pvk::Description.to_string(), m)?;
        let table_number = map_to_string(&pvk::TableNumber.to_string(), m)?;
        Ok(Self {
            description,
            table_number,
        })
    }
}

impl TryFrom<serde_json::Value> for NipaTableNumber {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading NipaTableNumber.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

/// The `NipaYear` struct contains fields associated with the BEA response for Year parameter
/// values in the NIPA dataset.  The fields define ranges of years for which table data is
/// available for a given frequency (annual, monthly or quartelry), using the pattern
/// `first_{frequency}_year` and `last_{frequency}_year`.
///
/// * first_annual_year - First year of annual data available for `table_name`.
/// * first_monthly_year - First year of monthly data available for `table_name`.
/// * first_quarterly_year - First year of quarterly data available for `table_name`.
/// * last_annual_year - Last year of annual data available for `table_name`.
/// * last_monthly_year - Last year of monthly data available for `table_name`.
/// * last_quarterly_year - Last year of quarterly data available for `table_name`.
/// * table_name - The table name parameter value associated with the target data.
///
/// This data structure is the "raw" version of [`NipaRange`](crate::NipaRange), where we use the
/// table name as keys in a HashMap to look up the appropriate range value for a given
/// frequency.  Zero values indicate no data is available for the specified table at the given
/// frequency.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct NipaYear {
    first_annual_year: String,
    first_monthly_year: String,
    first_quarterly_year: String,
    last_annual_year: String,
    last_monthly_year: String,
    last_quarterly_year: String,
    table_name: String,
}

impl NipaYear {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        use ParameterValueKind as pvk;
        let first_annual_year = map_to_string(&pvk::FirstAnnualYear.to_string(), m)?;
        let first_monthly_year = map_to_string(&pvk::FirstMonthlyYear.to_string(), m)?;
        let first_quarterly_year = map_to_string(&pvk::FirstQuarterlyYear.to_string(), m)?;
        let last_annual_year = map_to_string(&pvk::LastAnnualYear.to_string(), m)?;
        let last_monthly_year = map_to_string(&pvk::LastMonthlyYear.to_string(), m)?;
        let last_quarterly_year = map_to_string(&pvk::LastQuarterlyYear.to_string(), m)?;
        let table_name = map_to_string(&pvk::TableName.to_string(), m)?;
        Ok(Self {
            first_annual_year,
            first_monthly_year,
            first_quarterly_year,
            last_annual_year,
            last_monthly_year,
            last_quarterly_year,
            table_name,
        })
    }
}

impl TryFrom<serde_json::Value> for NipaYear {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading NipaYear.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
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
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_new::new,
    derive_getters::Getters,
)]
pub struct MneDoi {
    desc: String,
    key: String,
}

impl MneDoi {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        let desc = map_to_string("desc", m)?;
        let key = map_to_string("key", m)?;
        let result = Self::new(desc, key);
        Ok(result)
    }
}

impl TryFrom<serde_json::Value> for MneDoi {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading MneDoi.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
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
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[serde(rename_all = "PascalCase")]
#[setters(prefix = "with_", strip_option, borrow_self)]
pub struct Metadata {
    dataset: Dataset,
    dataset_description: String,
    #[serde(rename = "JSONUpdateDate")]
    json_update_date: Option<jiff::civil::Date>,
    #[serde(rename = "XMLUpdateDate")]
    xml_update_date: Option<jiff::civil::Date>,
}

impl Metadata {
    #[tracing::instrument(skip_all)]
    pub fn new(dataset: Dataset, dataset_description: String) -> Self {
        Self {
            dataset,
            dataset_description,
            json_update_date: None,
            xml_update_date: None,
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        use ParameterValueKind as pvk;
        tracing::trace!("Converting {} to Metadata.", &pvk::Dataset);
        let dataset = map_to_string(&pvk::Dataset.to_string(), m)?;
        let dataset = match Dataset::from_str(&dataset) {
            Ok(value) => value,
            Err(source) => {
                let error = DeriveFromStr::new(dataset, source, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        tracing::trace!("Dataset identified: {dataset}");
        let dataset_description = map_to_string(&pvk::DatasetDescription.to_string(), m)?;
        tracing::trace!("Description: {dataset_description}");
        let mut param = Self::new(dataset, dataset_description);
        if let Ok(date) = map_to_string("JSONUpdateDate", m) {
            let date = date
                .parse::<jiff::civil::Date>()
                .map_err(|e| Jiff::new(date, e))?;
            tracing::trace!("Json Update Date: {date}");
            param.json_update_date = Some(date);
        }
        if let Ok(date) = map_to_string("XMLUpdateDate", m) {
            let date = date
                .parse::<jiff::civil::Date>()
                .map_err(|e| Jiff::new(date, e))?;
            tracing::trace!("Xml Update Date: {date}");
            param.xml_update_date = Some(date);
        }
        Ok(param)
    }
}

impl TryFrom<serde_json::Value> for Metadata {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading Metadata.");
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
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_new::new,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterFields {
    desc: String,
    key: String,
}

impl ParameterFields {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        let desc = map_to_string("Desc", m)?;
        match map_to_string("Key", m) {
            Ok(key) => {
                let param = Self::new(desc, key);
                Ok(param)
            }
            Err(_) => {
                // by now we know desc exists,
                // so if key is missing we need to record it or dump it
                let key = String::new();
                let param = Self::new(desc, key);
                Ok(param)
            }
        }
    }
}

impl TryFrom<serde_json::Value> for ParameterFields {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading ParameterFields.");
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
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
    derive_more::From,
    strum::EnumIter,
)]
pub enum ParameterValueTable {
    #[from(Metadata)]
    Metadata(Metadata),
    #[from(MneDoi)]
    MneDoi(MneDoi),
    #[from(NipaFrequency)]
    NipaFrequency(NipaFrequency),
    #[from(NipaShowMillions)]
    NipaShowMillions(NipaShowMillions),
    #[from(NipaTable)]
    NipaTable(NipaTable),
    #[from(NipaTableNumber)]
    NipaTableNumber(NipaTableNumber),
    #[from(NipaYear)]
    NipaYear(NipaYear),
    #[from(ParameterFields)]
    ParameterFields(ParameterFields),
}

impl ParameterValueTable {
    /// This method tries to convert the input to each variant in sequence, returning the first
    /// successful conversion.  Do we need individual conversion methods that we can call into
    /// based on the expected output?
    pub fn read_json(value: &serde_json::Value) -> Result<Self, JsonParseError> {
        let tables = Self::iter().collect::<Vec<Self>>();
        for table in tables {
            match table {
                Self::Metadata(_) => {
                    if let Ok(t) = Metadata::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
                Self::ParameterFields(_) => {
                    if let Ok(t) = ParameterFields::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
                Self::MneDoi(_) => {
                    if let Ok(t) = MneDoi::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
                Self::NipaFrequency(_) => {
                    if let Ok(t) = NipaFrequency::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
                Self::NipaShowMillions(_) => {
                    if let Ok(t) = NipaShowMillions::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
                Self::NipaTable(_) => {
                    if let Ok(t) = NipaTable::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
                Self::NipaTableNumber(_) => {
                    if let Ok(t) = NipaTableNumber::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
                Self::NipaYear(_) => {
                    if let Ok(t) = NipaYear::try_from(value.clone()) {
                        return Ok(Self::from(t));
                    }
                }
            }
        }
        let error = KeyMissing::new(
            "not Parameter Value Table".to_string(),
            line!(),
            file!().to_string(),
        );
        Err(error.into())
    }

    pub fn parameter_fields(&self) -> Option<&ParameterFields> {
        match self {
            Self::ParameterFields(values) => Some(values),
            _ => None,
        }
    }
}

/// Thin wrapper around a vector of type [`ParameterValueTable`].
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterValues(Vec<ParameterValueTable>);

impl TryFrom<&serde_json::Value> for ParameterValues {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading ParameterValues");
        match value {
            serde_json::Value::Object(m) => {
                let key = "ParamValue".to_string();
                if let Some(data) = m.get(&key) {
                    match data {
                        serde_json::Value::Array(v) => {
                            tracing::trace!("Array found.");
                            let mut parameter = Vec::new();
                            for val in v {
                                let param = ParameterValueTable::read_json(val)?;
                                parameter.push(param);
                            }
                            tracing::trace!("Parameter Value Table found.");
                            let parameters = Self::new(parameter);
                            Ok(parameters)
                        }
                        serde_json::Value::Object(_) => {
                            tracing::trace!("Object found.");
                            let param = ParameterValueTable::read_json(data)?;
                            let parameters = Self::new(vec![param]);
                            Ok(parameters)
                        }
                        _ => {
                            tracing::trace!("Unexpected content: {m:#?}");
                            let error = NotArray::new(line!(), file!().to_string());
                            Err(error.into())
                        }
                    }
                } else {
                    tracing::trace!("Parameter Value Table missing.");
                    let error = KeyMissing::new(key, line!(), file!().to_string());
                    Err(error.into())
                }
            }
            _ => {
                // tracing::trace!("Wrong Value type: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}
