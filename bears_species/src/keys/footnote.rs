use crate::{
    BeaErr, DeriveFromStr, MneDoi, ParameterFields, ParameterName, ParameterValueTable,
    ParameterValueTableVariant,
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
    serde::Deserialize,
    serde::Serialize,
    derive_more::FromStr,
    derive_more::Display,
    strum::EnumIter,
)]
pub enum Footnotes {
    #[default]
    Yes,
    No,
}

impl Footnotes {
    #[tracing::instrument(skip_all)]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Yes => "footnotes are included",
            Self::No => "footnotes are not included",
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn key(&self) -> &'static str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::GetFootnotes.to_string();
        let value = self.key().to_owned();
        (key, value)
    }
}

use std::str::FromStr;
impl TryFrom<&ParameterFields> for Footnotes {
    type Error = DeriveFromStr;

    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        Self::from_str(value.key()).map_err(|e| {
            DeriveFromStr::new(value.key().to_owned(), e, line!(), file!().to_string())
        })
    }
}

impl TryFrom<&MneDoi> for Footnotes {
    type Error = DeriveFromStr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        Self::from_str(value.key()).map_err(|e| {
            DeriveFromStr::new(value.key().to_owned(), e, line!(), file!().to_string())
        })
    }
}

impl TryFrom<&ParameterValueTable> for Footnotes {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("ParameterFields or MneDoi needed, found {other:#?}"),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}
