use crate::{
    FromStrError, JsonParseError, JsonParseErrorKind, KeyMissing, NotArray, NotObject, json_str,
    map_to_bool, map_to_string,
};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_value: Option<String>,
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    multiple_accepted_flag: bool,
    parameter_data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_default_value: Option<String>,
    parameter_description: String,
    parameter_name: ParameterName,
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    parameter_is_required_flag: bool,
}

impl Parameter {
    /// Given a [`serde_json::Map`] object, attempts to parse `Self` from anticipated keys.  Can
    /// fail if the expected key is missing or if coercion to expected type (such as bool) fails.
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        let all_value = map_to_string("AllValue", m).ok();
        let multiple_accepted_flag = map_to_bool("MultipleAcceptedFlag", m)?;
        let parameter_data_type = map_to_string("ParameterDataType", m)?;
        let parameter_default_value = if let Some(value) = m.get("ParameterDefaultValue") {
            let v = json_str(value)?;
            Some(v)
        } else {
            None
        };
        let parameter_description = map_to_string("ParameterDescription", m)?;
        let parameter_name = ParameterName::from_map("ParameterName", m)?;
        let parameter_is_required_flag = map_to_bool("ParameterIsRequiredFlag", m)?;
        Ok(Self {
            all_value,
            multiple_accepted_flag,
            parameter_data_type,
            parameter_default_value,
            parameter_description,
            parameter_name,
            parameter_is_required_flag,
        })
    }
}

impl TryFrom<serde_json::Value> for Parameter {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading Parameter.");
        match value {
            serde_json::Value::Object(m) => {
                let param = Self::read_json(&m)?;
                Ok(param)
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

/// Thin wrapper around a vector of type [`Parameter`].
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
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "PascalCase")]
pub struct Parameters {
    /// The explicit field facilitates idempotent conversion back to JSON.
    parameter: Vec<Parameter>,
}

impl TryFrom<&serde_json::Value> for Parameters {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading Parameters");
        match value {
            serde_json::Value::Object(m) => {
                let key = "Parameter".to_string();
                if let Some(data) = m.get(&key) {
                    match data {
                        serde_json::Value::Array(v) => {
                            tracing::trace!("Array found.");
                            let mut parameter = Vec::new();
                            for val in v {
                                let param = Parameter::try_from(val.clone())?;
                                parameter.push(param);
                            }
                            tracing::trace!("Parameters found.");
                            let parameters = Self::new(parameter);
                            Ok(parameters)
                        }
                        serde_json::Value::Object(mp) => {
                            tracing::trace!("Object found.");
                            let param = Parameter::read_json(mp)?;
                            let params = Parameters::new(vec![param]);
                            Ok(params)
                        }
                        _ => {
                            tracing::trace!("Unexpected content: {m:#?}");
                            let error = NotArray::new(line!(), file!().to_string());
                            Err(error.into())
                        }
                    }
                } else {
                    tracing::trace!("Parameter missing.");
                    let error = KeyMissing::new(key, line!(), file!().to_string());
                    Err(error.into())
                }
            }
            _ => {
                tracing::trace!("Wrong Value type: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

/// Custom handling of bool deserialization to match BEA.
#[tracing::instrument(skip_all)]
pub fn deserialize_bool<'de, D: Deserializer<'de>>(de: D) -> Result<bool, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    match intermediate {
        "1" => Ok(true),
        "true" => Ok(true),
        "false" => Ok(false),
        "0" => Ok(false),
        _ => Ok(false),
    }
}

/// Custom handling of bool serialization to match BEA.
#[tracing::instrument(skip(serializer))]
pub fn serialize_bool<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    match *value {
        true => serializer.serialize_u8(1),
        false => serializer.serialize_u8(0),
    }
}

/// Each variant of the `ParameterName` enum represents a valid input for the `PARAMETERNAME`
/// parameter of the BEA REST API.
///
/// A single enum is not an ideal data structure because parameter names are only valid within a
/// context of a [`Dataset`](crate::Dataset) variant, a [`Method`](crate::Method) variant, and
/// potentially other parameters.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum ParameterName {
    Affiliation,
    AreaOrCountry,
    Channel,
    Classification,
    ClUnit,
    ColumnCode,
    ColumnType,
    Component,
    Country,
    Dataset,
    Destination,
    DirectionOfInvestment,
    Frequency,
    GeoFips,
    GetFootnotes,
    Indicator,
    Industry,
    IndustryCode,
    Investment,
    LineCode,
    LineDescription,
    LineNumber,
    MetricName,
    NonbankAffiliatesOnly,
    NoteRef,
    Notes,
    NoteText,
    OwnershipLevel,
    ParentInvestment,
    Quarter,
    RowCode,
    RowType,
    SeriesCode,
    SeriesID,
    ShowMillions,
    State,
    TableID,
    #[default]
    TableName,
    TimePeriod,
    TimeSeriesCode,
    TradeDirection,
    TypeOfInvestment,
    TypeOfService,
    UnitMult,
    Year,
}

impl ParameterName {
    /// Invoked when parsing from JSON, used in the `TryFrom` impl.
    #[tracing::instrument]
    pub fn from_json(json: &str) -> Result<Self, JsonParseError> {
        let mut name = json.to_string();
        // May include quotes added around a String, strip the quotes.
        if json.starts_with('"') {
            match serde_json::from_str(json) {
                Ok(v) => name = v,
                Err(e) => {
                    let error = FromStrError::new(name, e);
                    return Err(error.into());
                }
            }
        }
        name = name.trim().to_owned();
        if let Ok(p) = ParameterName::from_str(&name) {
            tracing::trace!("Name found: {p}");
            Ok(p)
        } else {
            tracing::trace!("Paramater Variant missing.");
            let error = NotParameterName::new(name, line!(), file!().to_owned());
            Err(error.into())
        }
    }

    /// Invoked by [`Parameter::read_json`].  A wrapper around the `TryFrom` impl that accepts a
    /// [`serde_json::Map`].
    ///
    /// Produces an error if the String value under the `PARAMETERNAME` key does not parse to a
    /// valid variant.
    #[tracing::instrument(skip_all)]
    pub fn from_map(
        key: &str,
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        if let Some(value) = m.get(key) {
            let name = ParameterName::try_from(value)?;
            Ok(name)
        } else {
            let error = KeyMissing::new(key.to_string(), line!(), file!().to_string());
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for ParameterName {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::String(s) => {
                let name = ParameterName::from_json(s)?;
                Ok(name)
            }
            _ => {
                tracing::trace!("Unexpected Value variant.");
                let error = JsonParseErrorKind::NotString;
                Err(error.into())
            }
        }
    }
}

#[derive(Debug, derive_new::new, derive_more::Display)]
#[display("{value} is not a ParameterName at line {line} of {file}")]
pub struct NotParameterName {
    value: String,
    line: u32,
    file: String,
}

impl std::error::Error for NotParameterName {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
