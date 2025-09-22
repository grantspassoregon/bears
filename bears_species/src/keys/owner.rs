use crate::{
    BeaErr, Integer, MneDoi, ParameterName, ParameterValueTable, ParameterValueTableVariant,
};

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
    serde::Deserialize,
    serde::Serialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum OwnershipKind {
    #[default]
    AllAffiliates,
    MajorityOwnedAffiliates,
}

impl TryFrom<&MneDoi> for OwnershipKind {
    type Error = OwnershipInvalid;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match value.key().as_str() {
            "0" => Ok(Self::MajorityOwnedAffiliates),
            "1" => Ok(Self::AllAffiliates),
            other => {
                let error = OwnershipInvalid::new(other.into(), line!(), file!().to_string());
                Err(error)
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
    derive_new::new,
    derive_getters::Getters,
)]
pub struct OwnershipLevel {
    key: Integer,
    kind: OwnershipKind,
}

impl OwnershipLevel {
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::OwnershipLevel.to_string();
        let value = self.key().to_string();
        (key, value)
    }
}

impl TryFrom<&MneDoi> for OwnershipLevel {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = Integer::try_from(value)?;
        let kind = OwnershipKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for OwnershipLevel {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("MneDoi needed, found {other:#?}"),
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
    Copy,
    Clone,
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
pub enum AffiliateKind {
    #[default]
    All,
    NonBank,
}

impl TryFrom<&MneDoi> for AffiliateKind {
    type Error = OwnershipInvalid;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match value.key().as_str() {
            "0" => Ok(Self::All),
            "1" => Ok(Self::NonBank),
            other => {
                let error = OwnershipInvalid::new(other.into(), line!(), file!().to_string());
                Err(error)
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
    derive_new::new,
    derive_getters::Getters,
)]
pub struct AffiliateLevel {
    key: Integer,
    kind: AffiliateKind,
}

impl AffiliateLevel {
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::NonbankAffiliatesOnly.to_string();
        let value = self.key().to_string();
        (key, value)
    }
}

impl TryFrom<&MneDoi> for AffiliateLevel {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = Integer::try_from(value)?;
        let kind = AffiliateKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for AffiliateLevel {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("MneDoi needed, found {other:#?}"),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid Ownership at line {line} in file {file}")]
pub struct OwnershipInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for OwnershipInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
