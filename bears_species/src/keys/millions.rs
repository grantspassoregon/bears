use crate::{
    BeaErr, JsonParseError, KeyMissing, NipaShowMillions, ParameterName, ParameterValueTable,
    ParameterValueTableVariant, VariantMissing,
};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}", self.show)]
pub struct Millions {
    description: String,
    show: bool,
}

impl TryFrom<&NipaShowMillions> for Millions {
    type Error = VariantMissing;
    fn try_from(value: &NipaShowMillions) -> Result<Self, Self::Error> {
        let description = value.description().to_string();
        let show = match value.show_millions_id().as_str() {
            "Y" => true,
            "N" => false,
            other => {
                tracing::warn!("Unexpected NipaShowMillions value: {other}");
                let clue = "value of 'Y' or 'N' expected".to_string();
                let input = other;
                let line = line!();
                let file = file!();
                let error = VariantMissing::new(clue, input.into(), line, file.into());
                return Err(error);
            }
        };
        Ok(Self::new(description, show))
    }
}

impl TryFrom<&ParameterValueTable> for Millions {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::NipaShowMillions(mil) => Ok(Self::try_from(mil)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "NipaShowMillions needed".to_string(),
                    line!(),
                    file!().to_string(),
                );
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
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
    derive_more::From,
)]
pub enum MillionsOptions {
    Yes,
    #[default]
    No,
}

impl MillionsOptions {
    pub fn value(&self) -> String {
        match self {
            Self::Yes => "Y".to_string(),
            Self::No => "N".to_string(),
        }
    }

    pub fn from_value(value: &str) -> Result<Self, JsonParseError> {
        match value {
            "Y" => Ok(Self::Yes),
            "N" => Ok(Self::No),
            _ => {
                let error = KeyMissing::new(value.into(), line!(), file!().into());
                Err(error.into())
            }
        }
    }

    pub fn params(&self) -> (String, String) {
        let key = ParameterName::ShowMillions.to_string();
        let value = self.value();
        (key, value)
    }
}
