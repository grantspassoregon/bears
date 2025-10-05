use crate::{
    BeaErr, DeriveFromStr, MneKind, ParameterName, ParameterValueTable, ParameterValueTableVariant,
    parameter_value::MneDoi,
};
use convert_case::Casing;
use std::str::FromStr;
use strum::IntoEnumIterator;

/// DI can be `Inward` or `Outward` variants, while AMNE can take all values.
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
    strum::EnumIter,
    derive_more::FromStr,
    derive_more::Display,
)]
pub enum DirectionKind {
    #[default]
    Inward,
    Outward,
    Parent,
    State,
}

impl DirectionKind {
    /// Returns the BEA description of the parameter variant.
    #[tracing::instrument(skip_all)]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Inward => {
                "Provides data for U.S. affiliates of foreign multinational companies.  Note that for state-level data on U.S. affiliates, directionOfInvestment should be ‘state’."
            }
            Self::Outward => {
                "Provides data for foreign affiliates of U.S. multinational companies."
            }
            Self::Parent => "Provides data on U.S. parent companies.",
            Self::State => {
                "Provides data for U.S. affiliates of foreign multinational companies at the state level.  Note that only data on employment and (prior to 2007) gross property, plant, and equipment are available at the state level."
            }
        }
    }

    /// Returns the BEA key of the parameter variant.
    #[tracing::instrument(skip_all)]
    pub fn key(&self) -> &'static str {
        match self {
            Self::Inward => "inward",
            Self::Outward => "outward",
            Self::Parent => "parent",
            Self::State => "state",
        }
    }

    /// Returns a vector containing source datasets in which the variant appears.
    #[tracing::instrument(skip_all)]
    pub fn source(&self) -> Vec<MneKind> {
        match self {
            Self::Inward => MneKind::iter().collect(),
            Self::Outward => MneKind::iter().collect(),
            Self::Parent => vec![MneKind::Amne],
            Self::State => vec![MneKind::Amne],
        }
    }

    /// Returns a vector containing the variants of source `kind`.
    #[tracing::instrument]
    pub fn mne(kind: MneKind) -> Vec<Self> {
        Self::iter()
            .filter(|v| v.source().contains(&kind))
            .collect()
    }
}

impl TryFrom<&MneDoi> for DirectionKind {
    type Error = DeriveFromStr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = value.key().to_case(convert_case::Case::Title);
        Self::from_str(&key).map_err(|e| DeriveFromStr::new(key, e, line!(), file!().to_owned()))
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
pub struct DirectionOfInvestment {
    description: String,
    key: String,
    kind: DirectionKind,
}

impl TryFrom<&MneDoi> for DirectionOfInvestment {
    type Error = DeriveFromStr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let description = value.desc().to_string();
        let key = value.key().to_string();
        let kind = DirectionKind::try_from(value)?;
        Ok(Self::new(description, key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for DirectionOfInvestment {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "MneDoi needed".to_string(),
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
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::FromStr,
    derive_more::Display,
)]
pub enum Investment {
    /// U.S. assets; other investment; currency and deposits
    #[default]
    CurrAndDepAssets,
    /// U.S. liabilities; other investment; currency and deposits
    CurrAndDepLiabs,
    /// U.S. liabilities to foreign official agencies; other investment; currency and deposits
    CurrAndDepLiabsFoa,
    /// U.S. assets; other reserve assets; currency and deposits
    CurrAndDepReserveAssets,
    /// U.S. debt assets except reserve assets
    DebtAssetsExclReserve,
    /// U.S. debt assets except reserve assets; central bank
    DebtAssetsExclReserveCenBank,
    /// U.S. debt assets except reserve assets; central bank; euro
    DebtAssetsExclReserveCenBankEuro,
    /// U.S. debt assets except reserve assets; central bank; euro; long term
    DebtAssetsExclReserveCenBankEuroLt,
    /// U.S. debt assets except reserve assets; central bank; euro; short term
    DebtAssetsExclReserveCenBankEuroSt,
    /// U.S. debt assets except reserve assets; central bank; foreign currency
    DebtAssetsExclReserveCenBankFc,
    /// U.S. debt assets except reserve assets; central bank; foreign currency; long term
    DebtAssetsExclReserveCenBankFcLt,
    /// U.S. debt assets except reserve assets; central bank; foreign currency; short term
    DebtAssetsExclReserveCenBankFcSt,
    /// U.S. debt assets except reserve assets; central bank; long term
    DebtAssetsExclReserveCenBankLt,
    /// U.S. debt assets except reserve assets; central bank; other foreign currency
    DebtAssetsExclReserveCenBankOthFc,
    /// U.S. debt assets except reserve assets; central bank; other foreign currency; long term
    DebtAssetsExclReserveCenBankOthFcLt,
    /// U.S. debt assets except reserve assets; central bank; other foreign currency; short term
    DebtAssetsExclReserveCenBankOthFcSt,
    /// U.S. debt assets except reserve assets; central bank; short term
    DebtAssetsExclReserveCenBankSt,
    /// U.S. debt assets except reserve assets; central bank; U.S. dollar
    DebtAssetsExclReserveCenBankUsd,
    /// U.S. debt assets except reserve assets; central bank; U.S. dollar; long term
    DebtAssetsExclReserveCenBankUsdLt,
    /// U.S. debt assets except reserve assets; central bank; U.S. dollar; short term
    DebtAssetsExclReserveCenBankUsdSt,
    /// U.S. debt assets except reserve assets; central bank; yen
    DebtAssetsExclReserveCenBankYen,
    /// U.S. debt assets except reserve assets; central bank; yen; long term
    DebtAssetsExclReserveCenBankYenLt,
    /// U.S. debt assets except reserve assets; central bank; yen; short term
    DebtAssetsExclReserveCenBankYenSt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank
    DebtAssetsExclReserveDepExclCenBank,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro
    DebtAssetsExclReserveDepExclCenBankEuro,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; long term
    DebtAssetsExclReserveDepExclCenBankEuroLt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; short term
    DebtAssetsExclReserveDepExclCenBankEuroSt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency
    DebtAssetsExclReserveDepExclCenBankFc,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; long term
    DebtAssetsExclReserveDepExclCenBankFcLt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; short term
    DebtAssetsExclReserveDepExclCenBankFcSt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; long term
    DebtAssetsExclReserveDepExclCenBankLt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency
    DebtAssetsExclReserveDepExclCenBankOthFc,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; long term
    DebtAssetsExclReserveDepExclCenBankOthFcLt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; short term
    DebtAssetsExclReserveDepExclCenBankOthFcSt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; short term
    DebtAssetsExclReserveDepExclCenBankSt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar
    DebtAssetsExclReserveDepExclCenBankUsd,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; long term
    DebtAssetsExclReserveDepExclCenBankUsdLt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; short term
    DebtAssetsExclReserveDepExclCenBankUsdSt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen
    DebtAssetsExclReserveDepExclCenBankYen,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; long term
    DebtAssetsExclReserveDepExclCenBankYenLt,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; short term
    DebtAssetsExclReserveDepExclCenBankYenSt,
    /// U.S. debt assets except reserve assets; euro
    DebtAssetsExclReserveEuro,
    /// U.S. debt assets except reserve assets; euro; long term
    DebtAssetsExclReserveEuroLt,
    /// U.S. debt assets except reserve assets; euro; short term
    DebtAssetsExclReserveEuroSt,
    /// U.S. debt assets except reserve assets; foreign currency
    DebtAssetsExclReserveFc,
    /// U.S. debt assets except reserve assets; foreign currency; long term
    DebtAssetsExclReserveFcLt,
    /// U.S. debt assets except reserve assets; foreign currency; short term
    DebtAssetsExclReserveFcSt,
    /// U.S. debt assets except reserve assets; general government
    DebtAssetsExclReserveGenGovt,
    /// U.S. debt assets except reserve assets; general government; euro
    DebtAssetsExclReserveGenGovtEuro,
    /// U.S. debt assets except reserve assets; general government; euro; long term
    DebtAssetsExclReserveGenGovtEuroLt,
    /// U.S. debt assets except reserve assets; general government; euro; short term
    DebtAssetsExclReserveGenGovtEuroSt,
    /// U.S. debt assets except reserve assets; general government; foreign currency
    DebtAssetsExclReserveGenGovtFc,
    /// U.S. debt assets except reserve assets; general government; foreign currency; long term
    DebtAssetsExclReserveGenGovtFcLt,
    /// U.S. debt assets except reserve assets; general government; foreign currency; short term
    DebtAssetsExclReserveGenGovtFcSt,
    /// U.S. debt assets except reserve assets; general government; long term
    DebtAssetsExclReserveGenGovtLt,
    /// U.S. debt assets except reserve assets; general government; other foreign currency
    DebtAssetsExclReserveGenGovtOthFc,
    /// U.S. debt assets except reserve assets; general government; other foreign currency; long term
    DebtAssetsExclReserveGenGovtOthFcLt,
    /// U.S. debt assets except reserve assets; general government; other foreign currency; short term
    DebtAssetsExclReserveGenGovtOthFcSt,
    /// U.S. debt assets except reserve assets; general government; short term
    DebtAssetsExclReserveGenGovtSt,
    /// U.S. debt assets except reserve assets; general government; U.S. dollar
    DebtAssetsExclReserveGenGovtUsd,
    /// U.S. debt assets except reserve assets; general government; U.S. dollar; long term
    DebtAssetsExclReserveGenGovtUsdLt,
    /// U.S. debt assets except reserve assets; general government; U.S. dollar; short term
    DebtAssetsExclReserveGenGovtUsdSt,
    /// U.S. debt assets except reserve assets; general government; yen
    DebtAssetsExclReserveGenGovtYen,
    /// U.S. debt assets except reserve assets; general government; yen; long term
    DebtAssetsExclReserveGenGovtYenLt,
    /// U.S. debt assets except reserve assets; general government; yen; short term
    DebtAssetsExclReserveGenGovtYenSt,
    /// U.S. debt assets except reserve assets; long term
    DebtAssetsExclReserveLt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government
    DebtAssetsExclReserveNonFinExclGenGovt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro
    DebtAssetsExclReserveNonFinExclGenGovtEuro,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; long term
    DebtAssetsExclReserveNonFinExclGenGovtEuroLt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; short term
    DebtAssetsExclReserveNonFinExclGenGovtEuroSt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency
    DebtAssetsExclReserveNonFinExclGenGovtFc,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; long term
    DebtAssetsExclReserveNonFinExclGenGovtFcLt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; short term
    DebtAssetsExclReserveNonFinExclGenGovtFcSt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; long term
    DebtAssetsExclReserveNonFinExclGenGovtLt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency
    DebtAssetsExclReserveNonFinExclGenGovtOthFc,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; long term
    DebtAssetsExclReserveNonFinExclGenGovtOthFcLt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; short term
    DebtAssetsExclReserveNonFinExclGenGovtOthFcSt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; short term
    DebtAssetsExclReserveNonFinExclGenGovtSt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar
    DebtAssetsExclReserveNonFinExclGenGovtUsd,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; long term
    DebtAssetsExclReserveNonFinExclGenGovtUsdLt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; short term
    DebtAssetsExclReserveNonFinExclGenGovtUsdSt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen
    DebtAssetsExclReserveNonFinExclGenGovtYen,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; long term
    DebtAssetsExclReserveNonFinExclGenGovtYenLt,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; short term
    DebtAssetsExclReserveNonFinExclGenGovtYenSt,
    /// U.S. debt assets except reserve assets; other foreign currency
    DebtAssetsExclReserveOthFc,
    /// U.S. debt assets except reserve assets; other foreign currency; long term
    DebtAssetsExclReserveOthFcLt,
    /// U.S. debt assets except reserve assets; other foreign currency; short term
    DebtAssetsExclReserveOthFcSt,
    /// U.S. debt assets except reserve assets; other financial institutions
    DebtAssetsExclReserveOthFin,
    /// U.S. debt assets except reserve assets; other financial institutions; euro
    DebtAssetsExclReserveOthFinEuro,
    /// U.S. debt assets except reserve assets; other financial institutions; euro; long term
    DebtAssetsExclReserveOthFinEuroLt,
    /// U.S. debt assets except reserve assets; other financial institutions; euro; short term
    DebtAssetsExclReserveOthFinEuroSt,
    /// U.S. debt assets except reserve assets; other financial institutions; foreign currency
    DebtAssetsExclReserveOthFinFc,
    /// U.S. debt assets except reserve assets; other financial institutions; foreign currency; long term
    DebtAssetsExclReserveOthFinFcLt,
    /// U.S. debt assets except reserve assets; other financial institutions; foreign currency; short term
    DebtAssetsExclReserveOthFinFcSt,
    /// U.S. debt assets except reserve assets; other financial institutions; long term
    DebtAssetsExclReserveOthFinLt,
    /// U.S. debt assets except reserve assets; other financial institutions; other foreign currency
    DebtAssetsExclReserveOthFinOthFc,
    /// U.S. debt assets except reserve assets; other financial institutions; other foreign currency; long term
    DebtAssetsExclReserveOthFinOthFcLt,
    /// U.S. debt assets except reserve assets; other financial institutions; other foreign currency; short term
    DebtAssetsExclReserveOthFinOthFcSt,
    /// U.S. debt assets except reserve assets; other financial institutions; short term
    DebtAssetsExclReserveOthFinSt,
    /// U.S. debt assets except reserve assets; other financial institutions; U.S. dollar
    DebtAssetsExclReserveOthFinUsd,
    /// U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; long term
    DebtAssetsExclReserveOthFinUsdLt,
    /// U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; short term
    DebtAssetsExclReserveOthFinUsdSt,
    /// U.S. debt assets except reserve assets; other financial institutions; yen
    DebtAssetsExclReserveOthFinYen,
    /// U.S. debt assets except reserve assets; other financial institutions; yen; long term
    DebtAssetsExclReserveOthFinYenLt,
    /// U.S. debt assets except reserve assets; other financial institutions; yen; short term
    DebtAssetsExclReserveOthFinYenSt,
    /// U.S. debt assets except reserve assets; short term
    DebtAssetsExclReserveSt,
    /// U.S. debt assets except reserve assets; U.S. dollar
    DebtAssetsExclReserveUsd,
    /// U.S. debt assets except reserve assets; U.S. dollar; long term
    DebtAssetsExclReserveUsdLt,
    /// U.S. debt assets except reserve assets; U.S. dollar; short term
    DebtAssetsExclReserveUsdSt,
    /// U.S. debt assets except reserve assets; yen
    DebtAssetsExclReserveYen,
    /// U.S. debt assets except reserve assets; yen; long term
    DebtAssetsExclReserveYenLt,
    /// U.S. debt assets except reserve assets; yen; short term
    DebtAssetsExclReserveYenSt,
    /// U.S. debt liabilities
    DebtLiabs,
    /// U.S. debt liabilities; central bank
    DebtLiabsCenBank,
    /// U.S. debt liabilities; central bank; euro
    DebtLiabsCenBankEuro,
    /// U.S. debt liabilities; central bank; euro; long term
    DebtLiabsCenBankEuroLt,
    /// U.S. debt liabilities; central bank; euro; short term
    DebtLiabsCenBankEuroSt,
    /// U.S. debt liabilities; central bank; foreign currency
    DebtLiabsCenBankFc,
    /// U.S. debt liabilities; central bank; foreign currency; long term
    DebtLiabsCenBankFcLt,
    /// U.S. debt liabilities; central bank; foreign currency; short term
    DebtLiabsCenBankFcSt,
    /// U.S. debt liabilities; central bank; long term
    DebtLiabsCenBankLt,
    /// U.S. debt liabilities; central bank; other foreign currency
    DebtLiabsCenBankOthFc,
    /// U.S. debt liabilities; central bank; other foreign currency; long term
    DebtLiabsCenBankOthFcLt,
    /// U.S. debt liabilities; central bank; other foreign currency; short term
    DebtLiabsCenBankOthFcSt,
    /// U.S. debt liabilities; central bank; short term
    DebtLiabsCenBankSt,
    /// U.S. debt liabilities; central bank; U.S. dollar
    DebtLiabsCenBankUsd,
    /// U.S. debt liabilities; central bank; U.S. dollar; long term
    DebtLiabsCenBankUsdLt,
    /// U.S. debt liabilities; central bank; U.S. dollar; short term
    DebtLiabsCenBankUsdSt,
    /// U.S. debt liabilities; central bank; yen
    DebtLiabsCenBankYen,
    /// U.S. debt liabilities; central bank; yen; long term
    DebtLiabsCenBankYenLt,
    /// U.S. debt liabilities; central bank; yen; short term
    DebtLiabsCenBankYenSt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank
    DebtLiabsDepExclCenBank,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; euro
    DebtLiabsDepExclCenBankEuro,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; euro; long term
    DebtLiabsDepExclCenBankEuroLt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; euro; short term
    DebtLiabsDepExclCenBankEuroSt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency
    DebtLiabsDepExclCenBankFc,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; long term
    DebtLiabsDepExclCenBankFcLt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; short term
    DebtLiabsDepExclCenBankFcSt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; long term
    DebtLiabsDepExclCenBankLt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency
    DebtLiabsDepExclCenBankOthFc,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; long term
    DebtLiabsDepExclCenBankOthFcLt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; short term
    DebtLiabsDepExclCenBankOthFcSt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; short term
    DebtLiabsDepExclCenBankSt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar
    DebtLiabsDepExclCenBankUsd,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; long term
    DebtLiabsDepExclCenBankUsdLt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; short term
    DebtLiabsDepExclCenBankUsdSt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; yen
    DebtLiabsDepExclCenBankYen,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; yen; long term
    DebtLiabsDepExclCenBankYenLt,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; yen; short term
    DebtLiabsDepExclCenBankYenSt,
    /// U.S. debt liabilities; euro
    DebtLiabsEuro,
    /// U.S. debt liabilities; euro; long term
    DebtLiabsEuroLt,
    /// U.S. debt liabilities; euro; short term
    DebtLiabsEuroSt,
    /// U.S. debt liabilities; foreign currency
    DebtLiabsFc,
    /// U.S. debt liabilities; foreign currency; long term
    DebtLiabsFcLt,
    /// U.S. debt liabilities; foreign currency; short term
    DebtLiabsFcSt,
    /// U.S. debt liabilities; general government
    DebtLiabsGenGovt,
    /// U.S. debt liabilities; general government; euro
    DebtLiabsGenGovtEuro,
    /// U.S. debt liabilities; general government; euro; long term
    DebtLiabsGenGovtEuroLt,
    /// U.S. debt liabilities; general government; euro; short term
    DebtLiabsGenGovtEuroSt,
    /// U.S. debt liabilities; general government; foreign currency
    DebtLiabsGenGovtFc,
    /// U.S. debt liabilities; general government; foreign currency; long term
    DebtLiabsGenGovtFcLt,
    /// U.S. debt liabilities; general government; foreign currency; short term
    DebtLiabsGenGovtFcSt,
    /// U.S. debt liabilities; general government; long term
    DebtLiabsGenGovtLt,
    /// U.S. debt liabilities; general government; other foreign currency
    DebtLiabsGenGovtOthFc,
    /// U.S. debt liabilities; general government; other foreign currency; long term
    DebtLiabsGenGovtOthFcLt,
    /// U.S. debt liabilities; general government; other foreign currency; short term
    DebtLiabsGenGovtOthFcSt,
    /// U.S. debt liabilities; general government; short term
    DebtLiabsGenGovtSt,
    /// U.S. debt liabilities; general government; U.S. dollar
    DebtLiabsGenGovtUsd,
    /// U.S. debt liabilities; general government; U.S. dollar; long term
    DebtLiabsGenGovtUsdLt,
    /// U.S. debt liabilities; general government; U.S. dollar; short term
    DebtLiabsGenGovtUsdSt,
    /// U.S. debt liabilities; general government; yen
    DebtLiabsGenGovtYen,
    /// U.S. debt liabilities; general government; yen; long term
    DebtLiabsGenGovtYenLt,
    /// U.S. debt liabilities; general government; yen; short term
    DebtLiabsGenGovtYenSt,
    /// U.S. debt liabilities; long term
    DebtLiabsLt,
    /// U.S. debt liabilities; nonfinancial institutions except general government
    DebtLiabsNonFinExclGenGovt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; euro
    DebtLiabsNonFinExclGenGovtEuro,
    /// U.S. debt liabilities; nonfinancial institutions except general government; euro; long term
    DebtLiabsNonFinExclGenGovtEuroLt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; euro; short term
    DebtLiabsNonFinExclGenGovtEuroSt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; foreign currency
    DebtLiabsNonFinExclGenGovtFc,
    /// U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; long term
    DebtLiabsNonFinExclGenGovtFcLt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; short term
    DebtLiabsNonFinExclGenGovtFcSt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; long term
    DebtLiabsNonFinExclGenGovtLt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency
    DebtLiabsNonFinExclGenGovtOthFc,
    /// U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; long term
    DebtLiabsNonFinExclGenGovtOthFcLt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; short term
    DebtLiabsNonFinExclGenGovtOthFcSt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; short term
    DebtLiabsNonFinExclGenGovtSt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar
    DebtLiabsNonFinExclGenGovtUsd,
    /// U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; long term
    DebtLiabsNonFinExclGenGovtUsdLt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; short term
    DebtLiabsNonFinExclGenGovtUsdSt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; yen
    DebtLiabsNonFinExclGenGovtYen,
    /// U.S. debt liabilities; nonfinancial institutions except general government; yen; long term
    DebtLiabsNonFinExclGenGovtYenLt,
    /// U.S. debt liabilities; nonfinancial institutions except general government; yen; short term
    DebtLiabsNonFinExclGenGovtYenSt,
    /// U.S. debt liabilities; other foreign currency
    DebtLiabsOthFc,
    /// U.S. debt liabilities; other foreign currency; long term
    DebtLiabsOthFcLt,
    /// U.S. debt liabilities; other foreign currency; short term
    DebtLiabsOthFcSt,
    /// U.S. debt liabilities; other financial institutions
    DebtLiabsOthFin,
    /// U.S. debt liabilities; other financial institutions; euro
    DebtLiabsOthFinEuro,
    /// U.S. debt liabilities; other financial institutions; euro; long term
    DebtLiabsOthFinEuroLt,
    /// U.S. debt liabilities; other financial institutions; euro; short term
    DebtLiabsOthFinEuroSt,
    /// U.S. debt liabilities; other financial institutions; foreign currency
    DebtLiabsOthFinFc,
    /// U.S. debt liabilities; other financial institutions; foreign currency; long term
    DebtLiabsOthFinFcLt,
    /// U.S. debt liabilities; other financial institutions; foreign currency; short term
    DebtLiabsOthFinFcSt,
    /// U.S. debt liabilities; other financial institutions; long term
    DebtLiabsOthFinLt,
    /// U.S. debt liabilities; other financial institutions; other foreign currency
    DebtLiabsOthFinOthFc,
    /// U.S. debt liabilities; other financial institutions; other foreign currency; long term
    DebtLiabsOthFinOthFcLt,
    /// U.S. debt liabilities; other financial institutions; other foreign currency; short term
    DebtLiabsOthFinOthFcSt,
    /// U.S. debt liabilities; other financial institutions; short term
    DebtLiabsOthFinSt,
    /// U.S. debt liabilities; other financial institutions; U.S. dollar
    DebtLiabsOthFinUsd,
    /// U.S. debt liabilities; other financial institutions; U.S. dollar; long term
    DebtLiabsOthFinUsdLt,
    /// U.S. debt liabilities; other financial institutions; U.S. dollar; short term
    DebtLiabsOthFinUsdSt,
    /// U.S. debt liabilities; other financial institutions; yen
    DebtLiabsOthFinYen,
    /// U.S. debt liabilities; other financial institutions; yen; long term
    DebtLiabsOthFinYenLt,
    /// U.S. debt liabilities; other financial institutions; yen; short term
    DebtLiabsOthFinYenSt,
    /// U.S. debt liabilities; short term
    DebtLiabsSt,
    /// U.S. debt liabilities; U.S. dollar
    DebtLiabsUsd,
    /// U.S. debt liabilities; U.S. dollar; long term
    DebtLiabsUsdLt,
    /// U.S. debt liabilities; U.S. dollar; short term
    DebtLiabsUsdSt,
    /// U.S. debt liabilities; yen
    DebtLiabsYen,
    /// U.S. debt liabilities; yen; long term
    DebtLiabsYenLt,
    /// U.S. debt liabilities; yen; short term
    DebtLiabsYenSt,
    /// U.S. assets; portfolio investment; debt securities
    DebtSecAssets,
    /// U.S. liabilities; portfolio investment; debt securities
    DebtSecLiabs,
    /// U.S. liabilities to foreign official agencies; portfolio investment; debt securities
    DebtSecLiabsFoa,
    /// U.S. assets; direct investment at market value, asset/liability basis
    DiInvAssets,
    /// U.S. assets; direct investment at current cost, asset/liability basis
    DiInvAssetsCurrCost,
    /// U.S. assets; direct investment; adjustment to revalue equity from historical cost to market value
    DiInvAssetsHistCostToMarketValueAdj,
    /// U.S. assets; direct investment at market value, asset/liability basis; non-SPEs
    DiInvAssetsNonSpe,
    /// U.S. assets; direct investment at market value, asset/liability basis; SPEs
    DiInvAssetsSpe,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments
    DiInvDebtInstAssets,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; euro
    DiInvDebtInstAssetsEuro,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; euro; long term
    DiInvDebtInstAssetsEuroLt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; euro; short term
    DiInvDebtInstAssetsEuroSt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency
    DiInvDebtInstAssetsFc,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; long term
    DiInvDebtInstAssetsFcLt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; short term
    DiInvDebtInstAssetsFcSt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; long term
    DiInvDebtInstAssetsLt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; non-SPEs
    DiInvDebtInstAssetsNonSpe,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency
    DiInvDebtInstAssetsOthFc,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; long term
    DiInvDebtInstAssetsOthFcLt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; short term
    DiInvDebtInstAssetsOthFcSt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; SPEs
    DiInvDebtInstAssetsSpe,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; short term
    DiInvDebtInstAssetsSt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar
    DiInvDebtInstAssetsUsd,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term
    DiInvDebtInstAssetsUsdLt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term
    DiInvDebtInstAssetsUsdSt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; yen
    DiInvDebtInstAssetsYen,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; yen; long term
    DiInvDebtInstAssetsYenLt,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; yen; short term
    DiInvDebtInstAssetsYenSt,
    /// Inward direct investment (foreign direct investment in the United States), directional basis; debt instruments
    DiInvDebtInstInward,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments
    DiInvDebtInstLiabs,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro
    DiInvDebtInstLiabsEuro,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; long term
    DiInvDebtInstLiabsEuroLt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; short term
    DiInvDebtInstLiabsEuroSt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency
    DiInvDebtInstLiabsFc,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; long term
    DiInvDebtInstLiabsFcLt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; short term
    DiInvDebtInstLiabsFcSt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; long term
    DiInvDebtInstLiabsLt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; non-SPEs
    DiInvDebtInstLiabsNonSpe,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency
    DiInvDebtInstLiabsOthFc,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; long term
    DiInvDebtInstLiabsOthFcLt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; short term
    DiInvDebtInstLiabsOthFcSt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; SPEs
    DiInvDebtInstLiabsSpe,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; short term
    DiInvDebtInstLiabsSt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar
    DiInvDebtInstLiabsUsd,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term
    DiInvDebtInstLiabsUsdLt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term
    DiInvDebtInstLiabsUsdSt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen
    DiInvDebtInstLiabsYen,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; long term
    DiInvDebtInstLiabsYenLt,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; short term
    DiInvDebtInstLiabsYenSt,
    /// Outward direct investment (U.S. direct investment abroad), directional basis; debt instruments
    DiInvDebtInstOutward,
    /// Direct investment; debt instruments; U.S. affiliates' claims
    DiInvDebtInstUsAffiliatesClaims,
    /// U.S. non-SPE affiliates' debt asset position in their foreign parent groups
    DiInvDebtInstUsAffiliatesClaimsByNonSpe,
    /// U.S. SPE affiliates' debt asset position in their foreign parent groups
    DiInvDebtInstUsAffiliatesClaimsBySpe,
    /// Direct investment; debt instruments; U.S. affiliates' claims; non-SPEs
    DiInvDebtInstUsAffiliatesClaimsNonSpe,
    /// Direct investment; debt instruments; U.S. affiliates' claims; SPEs
    DiInvDebtInstUsAffiliatesClaimsSpe,
    /// Direct investment; debt instruments; U.S. affiliates' liabilites
    DiInvDebtInstUsAffiliatesLiabs,
    /// Direct investment; debt instruments; U.S. affiliates' liabilites; non-SPEs
    DiInvDebtInstUsAffiliatesLiabsNonSpe,
    /// Direct investment; debt instruments; U.S. affiliates' liabilites; SPEs
    DiInvDebtInstUsAffiliatesLiabsSpe,
    /// Direct investment; debt instruments; U.S. parents' claims
    DiInvDebtInstUsParentsClaims,
    /// Direct investment; debt instruments; U.S. parents' claims; non-SPEs
    DiInvDebtInstUsParentsClaimsNonSpe,
    /// Direct investment; debt instruments; U.S. parents' claims; SPEs
    DiInvDebtInstUsParentsClaimsSpe,
    /// Direct investment; debt instruments; U.S. parents' liabilites
    DiInvDebtInstUsParentsLiabs,
    /// U.S. parents' debt liability position in their foreign non-SPE affiliates
    DiInvDebtInstUsParentsLiabsInNonSpe,
    /// U.S. parents' debt liability position in their foreign SPE affiliates
    DiInvDebtInstUsParentsLiabsInSpe,
    /// Direct investment; debt instruments; U.S. parents' liabilites; non-SPEs
    DiInvDebtInstUsParentsLiabsNonSpe,
    /// Direct investment; debt instruments; U.S. parents' liabilites; SPEs
    DiInvDebtInstUsParentsLiabsSpe,
    /// Direct investment; adjustments to convert to directional basis
    DiInvDirectionalBasisAdj,
    /// U.S. assets; direct investment at market value; equity
    DiInvEquityAssets,
    /// U.S. assets; direct investment at current cost; equity
    DiInvEquityAssetsCurrCost,
    /// U.S. assets; direct investment at historical cost; equity
    DiInvEquityAssetsHistCost,
    /// U.S. assets; direct investment at market value; equity; non-SPEs
    DiInvEquityAssetsNonSpe,
    /// U.S. assets; direct investment at market value; equity; SPEs
    DiInvEquityAssetsSpe,
    /// U.S. liabilities; direct investment at market value; equity
    DiInvEquityInward,
    /// U.S. liabilities; direct investment at market value; equity
    DiInvEquityLiabs,
    /// U.S. liabilities; direct investment at current cost; equity
    DiInvEquityLiabsCurrCost,
    /// U.S. liabilities; direct investment at historical cost; equity
    DiInvEquityLiabsHistCost,
    /// U.S. liabilities; direct investment at market value; equity; non-SPEs
    DiInvEquityLiabsNonSpe,
    /// U.S. liabilities; direct investment at market value; equity; SPEs
    DiInvEquityLiabsSpe,
    /// Inward direct investment (foreign direct investment in the United States) at current cost, directional basis
    DiInvInwardCurrCost,
    /// Inward direct investment (foreign direct investment in the United States) at historical cost, directional basis
    DiInvInwardHistCost,
    /// Inward direct investment (foreign direct investment in the United States) at market value, directional basis
    DiInvInwardMarketValue,
    /// U.S. liabilities; direct investment at market value, asset/liability basis
    DiInvLiabs,
    /// U.S. liabilities; direct investment at current cost, asset/liability basis
    DiInvLiabsCurrCost,
    /// U.S. liabilities; direct investment; adjustment to revalue equity from historical cost to market value
    DiInvLiabsHistCostToMarketValueAdj,
    /// U.S. liabilities; direct investment at market value, asset/liability basis; non-SPEs
    DiInvLiabsNonSpe,
    /// U.S. liabilities; direct investment at market value, asset/liability basis; SPEs
    DiInvLiabsSpe,
    /// Outward direct investment (U.S. direct investment abroad) at current cost, directional basis
    DiInvOutwardCurrCost,
    /// Outward direct investment (U.S. direct investment abroad) at historical cost, directional basis
    DiInvOutwardHistCost,
    /// Outward direct investment (U.S. direct investment abroad) at market value, directional basis
    DiInvOutwardMarketValue,
    /// U.S. assets; portfolio investment; equity and investment fund shares
    EquityAndInvFundSharesAssets,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares
    EquityAndInvFundSharesLiabs,
    /// U.S. liabilities to foreign official agencies; portfolio investment; equity and investment fund shares
    EquityAndInvFundSharesLiabsFoa,
    /// U.S. assets
    FinAssets,
    /// U.S. assets excluding financial derivatives
    FinAssetsExclFinDeriv,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value
    FinDerivAssets,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts
    FinDerivExchTradedAssets,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts
    FinDerivExchTradedLiabs,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts
    FinDerivForExAssets,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts
    FinDerivForExLiabs,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value
    FinDerivLiabs,
    /// U.S. net international investment position; financial derivatives other than reserves
    FinDerivNet,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts
    FinDerivOtcAssets,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts
    FinDerivOtcLiabs,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts
    FinDerivOthAssets,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts
    FinDerivOthLiabs,
    /// U.S. assets; other reserve assets; financial derivatives
    FinDerivReserveAssets,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts
    FinDerivSingleCurrAssets,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts
    FinDerivSingleCurrLiabs,
    /// U.S. liabilities
    FinLiabs,
    /// U.S. liabilities excluding financial derivatives
    FinLiabsExclFinDeriv,
    /// U.S. liabilities to foreign official agencies
    FinLiabsFoa,
    /// U.S. assets; reserve assets; monetary gold
    GoldReserveAssets,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund
    ImfReserveAssets,
    /// U.S. assets; other investment; insurance technical reserves
    InsTechReservesAssets,
    /// U.S. liabilities; other investment; insurance technical reserves
    InsTechReservesLiabs,
    /// U.S. assets; other investment; loans
    LoansAssets,
    /// U.S. liabilities; other investment; loans
    LoansLiabs,
    /// U.S. liabilities to foreign official agencies; other investment; loans
    LoansLiabsFoa,
    /// U.S. assets; portfolio investment; long-term debt securities
    LtDebtSecAssets,
    /// U.S. liabilities; portfolio investment; long-term debt securities
    LtDebtSecLiabs,
    /// U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities
    LtDebtSecLiabsFoa,
    /// U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities; Treasury bonds and notes
    LtDebtSecTreasLiabsFoa,
    /// U.S. net international investment position
    Net,
    /// U.S. net international investment position excluding financial derivatives
    NetExclFinDeriv,
    /// U.S. assets; other reserve assets; other claims
    OthClmReserveAssets,
    /// U.S. assets; other investment; other equity
    OthEquityAssets,
    /// U.S. liabilities; other investment; other equity
    OthEquityLiabs,
    /// U.S. liabilities to foreign official agencies; other investment; other equity
    OthEquityLiabsFoa,
    /// U.S. assets; other investment
    OthInvAssets,
    /// U.S. liabilities; other investment
    OthInvLiabs,
    /// U.S. liabilities to foreign official agencies; other investment
    OthInvLiabsFoa,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes
    OthLtDebtSecLiabs,
    /// U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities excluding Treasury bonds and notes
    OthLtDebtSecLiabsFoa,
    /// U.S. assets; other reserve assets
    OthReserveAssets,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates
    OthStDebtSecLiabs,
    /// U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities excluding Treasury bills and certificates
    OthStDebtSecLiabsFoa,
    /// U.S. assets; portfolio investment
    PfInvAssets,
    /// U.S. liabilities; portfolio investment
    PfInvLiabs,
    /// U.S. liabilities to foreign official agencies; portfolio investment
    PfInvLiabsFoa,
    /// U.S. assets; reserve assets
    ReserveAssets,
    /// U.S. assets; reserve assets; central bank
    ReserveAssetsCenBank,
    /// U.S. assets; reserve assets; central bank; long term
    ReserveAssetsCenBankLt,
    /// U.S. assets; reserve assets; central bank; not in special drawing rights basket
    ReserveAssetsCenBankNotSdrBasket,
    /// U.S. assets; reserve assets; central bank; not in special drawing rights basket; long term
    ReserveAssetsCenBankNotSdrBasketLt,
    /// U.S. assets; reserve assets; central bank; not in special drawing rights basket; short term
    ReserveAssetsCenBankNotSdrBasketSt,
    /// U.S. assets; reserve assets; central bank; in special drawing rights basket
    ReserveAssetsCenBankSdrBasket,
    /// U.S. assets; reserve assets; central bank; in special drawing rights basket; long term
    ReserveAssetsCenBankSdrBasketLt,
    /// U.S. assets; reserve assets; central bank; in special drawing rights basket; short term
    ReserveAssetsCenBankSdrBasketSt,
    /// U.S. assets; reserve assets; central bank; short term
    ReserveAssetsCenBankSt,
    /// U.S. assets; reserve assets; general government
    ReserveAssetsGenGovt,
    /// U.S. assets; reserve assets; general government; long term
    ReserveAssetsGenGovtLt,
    /// U.S. assets; reserve assets; general government; not in special drawing rights basket
    ReserveAssetsGenGovtNotSdrBasket,
    /// U.S. assets; reserve assets; general government; not in special drawing rights basket; long term
    ReserveAssetsGenGovtNotSdrBasketLt,
    /// U.S. assets; reserve assets; general government; not in special drawing rights basket; short term
    ReserveAssetsGenGovtNotSdrBasketSt,
    /// U.S. assets; reserve assets; general government; in special drawing rights basket
    ReserveAssetsGenGovtSdrBasket,
    /// U.S. assets; reserve assets; general government; in special drawing rights basket; long term
    ReserveAssetsGenGovtSdrBasketLt,
    /// U.S. assets; reserve assets; general government; in special drawing rights basket; short term
    ReserveAssetsGenGovtSdrBasketSt,
    /// U.S. assets; reserve assets; general government; short term
    ReserveAssetsGenGovtSt,
    /// U.S. assets; reserve assets; long term
    ReserveAssetsLt,
    /// U.S. assets; reserve assets; not in special drawing rights basket
    ReserveAssetsNotSdrBasket,
    /// U.S. assets; reserve assets; not in special drawing rights basket; long term
    ReserveAssetsNotSdrBasketLt,
    /// U.S. assets; reserve assets; not in special drawing rights basket; short term
    ReserveAssetsNotSdrBasketSt,
    /// U.S. assets; reserve assets; in special drawing rights basket
    ReserveAssetsSdrBasket,
    /// U.S. assets; reserve assets; in special drawing rights basket; long term
    ReserveAssetsSdrBasketLt,
    /// U.S. assets; reserve assets; in special drawing rights basket; short term
    ReserveAssetsSdrBasketSt,
    /// U.S. assets; reserve assets; short term
    ReserveAssetsSt,
    /// U.S. liabilities; other investment; special drawing rights allocations
    SdrAllocLiabs,
    /// U.S. liabilities to foreign official agencies; other investment; special drawing rights allocations
    SdrAllocLiabsFoa,
    /// U.S. assets; reserve assets; special drawing rights
    SdrReserveAssets,
    /// U.S. assets; other reserve assets; securities
    SecReserveAssets,
    /// U.S. assets; portfolio investment; short-term debt securities
    StDebtSecAssets,
    /// U.S. liabilities; portfolio investment; short-term debt securities
    StDebtSecLiabs,
    /// U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities
    StDebtSecLiabsFoa,
    /// U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities; Treasury bills and certificates
    StDebtSecTreasLiabsFoa,
    /// U.S. assets; other investment; trade credit and advances
    TrdCredAndAdvAssets,
    /// U.S. liabilities; other investment; trade credit and advances
    TrdCredAndAdvLiabs,
    /// U.S. liabilities to foreign official agencies; other investment; trade credit and advances
    TrdCredAndAdvLiabsFoa,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates
    TreasBillsAndCertsLiabs,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes
    TreasBondsAndNotesLiabs,
}

impl Investment {
    pub fn description(&self) -> &'static str {
        match self {
            Self::CurrAndDepAssets => "U.S. assets; other investment; currency and deposits",
            Self::CurrAndDepLiabs => "U.S. liabilities; other investment; currency and deposits",
            Self::CurrAndDepLiabsFoa => {
                "U.S. liabilities to foreign official agencies; other investment; currency and deposits"
            }
            Self::CurrAndDepReserveAssets => {
                "U.S. assets; other reserve assets; currency and deposits"
            }
            Self::DebtAssetsExclReserve => "U.S. debt assets except reserve assets",
            Self::DebtAssetsExclReserveCenBank => {
                "U.S. debt assets except reserve assets; central bank"
            }
            Self::DebtAssetsExclReserveCenBankEuro => {
                "U.S. debt assets except reserve assets; central bank; euro"
            }
            Self::DebtAssetsExclReserveCenBankEuroLt => {
                "U.S. debt assets except reserve assets; central bank; euro; long term"
            }
            Self::DebtAssetsExclReserveCenBankEuroSt => {
                "U.S. debt assets except reserve assets; central bank; euro; short term"
            }
            Self::DebtAssetsExclReserveCenBankFc => {
                "U.S. debt assets except reserve assets; central bank; foreign currency"
            }
            Self::DebtAssetsExclReserveCenBankFcLt => {
                "U.S. debt assets except reserve assets; central bank; foreign currency; long term"
            }
            Self::DebtAssetsExclReserveCenBankFcSt => {
                "U.S. debt assets except reserve assets; central bank; foreign currency; short term"
            }
            Self::DebtAssetsExclReserveCenBankLt => {
                "U.S. debt assets except reserve assets; central bank; long term"
            }
            Self::DebtAssetsExclReserveCenBankOthFc => {
                "U.S. debt assets except reserve assets; central bank; other foreign currency"
            }
            Self::DebtAssetsExclReserveCenBankOthFcLt => {
                "U.S. debt assets except reserve assets; central bank; other foreign currency; long term"
            }
            Self::DebtAssetsExclReserveCenBankOthFcSt => {
                "U.S. debt assets except reserve assets; central bank; other foreign currency; short term"
            }
            Self::DebtAssetsExclReserveCenBankSt => {
                "U.S. debt assets except reserve assets; central bank; short term"
            }
            Self::DebtAssetsExclReserveCenBankUsd => {
                "U.S. debt assets except reserve assets; central bank; U.S. dollar"
            }
            Self::DebtAssetsExclReserveCenBankUsdLt => {
                "U.S. debt assets except reserve assets; central bank; U.S. dollar; long term"
            }
            Self::DebtAssetsExclReserveCenBankUsdSt => {
                "U.S. debt assets except reserve assets; central bank; U.S. dollar; short term"
            }
            Self::DebtAssetsExclReserveCenBankYen => {
                "U.S. debt assets except reserve assets; central bank; yen"
            }
            Self::DebtAssetsExclReserveCenBankYenLt => {
                "U.S. debt assets except reserve assets; central bank; yen; long term"
            }
            Self::DebtAssetsExclReserveCenBankYenSt => {
                "U.S. debt assets except reserve assets; central bank; yen; short term"
            }
            Self::DebtAssetsExclReserveDepExclCenBank => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank"
            }
            Self::DebtAssetsExclReserveDepExclCenBankEuro => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro"
            }
            Self::DebtAssetsExclReserveDepExclCenBankEuroLt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; long term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankEuroSt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; short term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankFc => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency"
            }
            Self::DebtAssetsExclReserveDepExclCenBankFcLt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; long term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankFcSt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; short term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankLt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; long term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankOthFc => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency"
            }
            Self::DebtAssetsExclReserveDepExclCenBankOthFcLt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; long term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankOthFcSt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; short term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankSt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; short term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankUsd => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar"
            }
            Self::DebtAssetsExclReserveDepExclCenBankUsdLt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; long term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankUsdSt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; short term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankYen => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen"
            }
            Self::DebtAssetsExclReserveDepExclCenBankYenLt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; long term"
            }
            Self::DebtAssetsExclReserveDepExclCenBankYenSt => {
                "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; short term"
            }
            Self::DebtAssetsExclReserveEuro => "U.S. debt assets except reserve assets; euro",
            Self::DebtAssetsExclReserveEuroLt => {
                "U.S. debt assets except reserve assets; euro; long term"
            }
            Self::DebtAssetsExclReserveEuroSt => {
                "U.S. debt assets except reserve assets; euro; short term"
            }
            Self::DebtAssetsExclReserveFc => {
                "U.S. debt assets except reserve assets; foreign currency"
            }
            Self::DebtAssetsExclReserveFcLt => {
                "U.S. debt assets except reserve assets; foreign currency; long term"
            }
            Self::DebtAssetsExclReserveFcSt => {
                "U.S. debt assets except reserve assets; foreign currency; short term"
            }
            Self::DebtAssetsExclReserveGenGovt => {
                "U.S. debt assets except reserve assets; general government"
            }
            Self::DebtAssetsExclReserveGenGovtEuro => {
                "U.S. debt assets except reserve assets; general government; euro"
            }
            Self::DebtAssetsExclReserveGenGovtEuroLt => {
                "U.S. debt assets except reserve assets; general government; euro; long term"
            }
            Self::DebtAssetsExclReserveGenGovtEuroSt => {
                "U.S. debt assets except reserve assets; general government; euro; short term"
            }
            Self::DebtAssetsExclReserveGenGovtFc => {
                "U.S. debt assets except reserve assets; general government; foreign currency"
            }
            Self::DebtAssetsExclReserveGenGovtFcLt => {
                "U.S. debt assets except reserve assets; general government; foreign currency; long term"
            }
            Self::DebtAssetsExclReserveGenGovtFcSt => {
                "U.S. debt assets except reserve assets; general government; foreign currency; short term"
            }
            Self::DebtAssetsExclReserveGenGovtLt => {
                "U.S. debt assets except reserve assets; general government; long term"
            }
            Self::DebtAssetsExclReserveGenGovtOthFc => {
                "U.S. debt assets except reserve assets; general government; other foreign currency"
            }
            Self::DebtAssetsExclReserveGenGovtOthFcLt => {
                "U.S. debt assets except reserve assets; general government; other foreign currency; long term"
            }
            Self::DebtAssetsExclReserveGenGovtOthFcSt => {
                "U.S. debt assets except reserve assets; general government; other foreign currency; short term"
            }
            Self::DebtAssetsExclReserveGenGovtSt => {
                "U.S. debt assets except reserve assets; general government; short term"
            }
            Self::DebtAssetsExclReserveGenGovtUsd => {
                "U.S. debt assets except reserve assets; general government; U.S. dollar"
            }
            Self::DebtAssetsExclReserveGenGovtUsdLt => {
                "U.S. debt assets except reserve assets; general government; U.S. dollar; long term"
            }
            Self::DebtAssetsExclReserveGenGovtUsdSt => {
                "U.S. debt assets except reserve assets; general government; U.S. dollar; short term"
            }
            Self::DebtAssetsExclReserveGenGovtYen => {
                "U.S. debt assets except reserve assets; general government; yen"
            }
            Self::DebtAssetsExclReserveGenGovtYenLt => {
                "U.S. debt assets except reserve assets; general government; yen; long term"
            }
            Self::DebtAssetsExclReserveGenGovtYenSt => {
                "U.S. debt assets except reserve assets; general government; yen; short term"
            }
            Self::DebtAssetsExclReserveLt => "U.S. debt assets except reserve assets; long term",
            Self::DebtAssetsExclReserveNonFinExclGenGovt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtEuro => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtEuroLt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; long term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtEuroSt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; short term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtFc => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtFcLt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; long term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtFcSt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; short term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtLt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; long term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtOthFc => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtOthFcLt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; long term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtOthFcSt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; short term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtSt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; short term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtUsd => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtUsdLt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; long term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtUsdSt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; short term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtYen => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtYenLt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; long term"
            }
            Self::DebtAssetsExclReserveNonFinExclGenGovtYenSt => {
                "U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; short term"
            }
            Self::DebtAssetsExclReserveOthFc => {
                "U.S. debt assets except reserve assets; other foreign currency"
            }
            Self::DebtAssetsExclReserveOthFcLt => {
                "U.S. debt assets except reserve assets; other foreign currency; long term"
            }
            Self::DebtAssetsExclReserveOthFcSt => {
                "U.S. debt assets except reserve assets; other foreign currency; short term"
            }
            Self::DebtAssetsExclReserveOthFin => {
                "U.S. debt assets except reserve assets; other financial institutions"
            }
            Self::DebtAssetsExclReserveOthFinEuro => {
                "U.S. debt assets except reserve assets; other financial institutions; euro"
            }
            Self::DebtAssetsExclReserveOthFinEuroLt => {
                "U.S. debt assets except reserve assets; other financial institutions; euro; long term"
            }
            Self::DebtAssetsExclReserveOthFinEuroSt => {
                "U.S. debt assets except reserve assets; other financial institutions; euro; short term"
            }
            Self::DebtAssetsExclReserveOthFinFc => {
                "U.S. debt assets except reserve assets; other financial institutions; foreign currency"
            }
            Self::DebtAssetsExclReserveOthFinFcLt => {
                "U.S. debt assets except reserve assets; other financial institutions; foreign currency; long term"
            }
            Self::DebtAssetsExclReserveOthFinFcSt => {
                "U.S. debt assets except reserve assets; other financial institutions; foreign currency; short term"
            }
            Self::DebtAssetsExclReserveOthFinLt => {
                "U.S. debt assets except reserve assets; other financial institutions; long term"
            }
            Self::DebtAssetsExclReserveOthFinOthFc => {
                "U.S. debt assets except reserve assets; other financial institutions; other foreign currency"
            }
            Self::DebtAssetsExclReserveOthFinOthFcLt => {
                "U.S. debt assets except reserve assets; other financial institutions; other foreign currency; long term"
            }
            Self::DebtAssetsExclReserveOthFinOthFcSt => {
                "U.S. debt assets except reserve assets; other financial institutions; other foreign currency; short term"
            }
            Self::DebtAssetsExclReserveOthFinSt => {
                "U.S. debt assets except reserve assets; other financial institutions; short term"
            }
            Self::DebtAssetsExclReserveOthFinUsd => {
                "U.S. debt assets except reserve assets; other financial institutions; U.S. dollar"
            }
            Self::DebtAssetsExclReserveOthFinUsdLt => {
                "U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; long term"
            }
            Self::DebtAssetsExclReserveOthFinUsdSt => {
                "U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; short term"
            }
            Self::DebtAssetsExclReserveOthFinYen => {
                "U.S. debt assets except reserve assets; other financial institutions; yen"
            }
            Self::DebtAssetsExclReserveOthFinYenLt => {
                "U.S. debt assets except reserve assets; other financial institutions; yen; long term"
            }
            Self::DebtAssetsExclReserveOthFinYenSt => {
                "U.S. debt assets except reserve assets; other financial institutions; yen; short term"
            }
            Self::DebtAssetsExclReserveSt => "U.S. debt assets except reserve assets; short term",
            Self::DebtAssetsExclReserveUsd => "U.S. debt assets except reserve assets; U.S. dollar",
            Self::DebtAssetsExclReserveUsdLt => {
                "U.S. debt assets except reserve assets; U.S. dollar; long term"
            }
            Self::DebtAssetsExclReserveUsdSt => {
                "U.S. debt assets except reserve assets; U.S. dollar; short term"
            }
            Self::DebtAssetsExclReserveYen => "U.S. debt assets except reserve assets; yen",
            Self::DebtAssetsExclReserveYenLt => {
                "U.S. debt assets except reserve assets; yen; long term"
            }
            Self::DebtAssetsExclReserveYenSt => {
                "U.S. debt assets except reserve assets; yen; short term"
            }
            Self::DebtLiabs => "U.S. debt liabilities",
            Self::DebtLiabsCenBank => "U.S. debt liabilities; central bank",
            Self::DebtLiabsCenBankEuro => "U.S. debt liabilities; central bank; euro",
            Self::DebtLiabsCenBankEuroLt => "U.S. debt liabilities; central bank; euro; long term",
            Self::DebtLiabsCenBankEuroSt => "U.S. debt liabilities; central bank; euro; short term",
            Self::DebtLiabsCenBankFc => "U.S. debt liabilities; central bank; foreign currency",
            Self::DebtLiabsCenBankFcLt => {
                "U.S. debt liabilities; central bank; foreign currency; long term"
            }
            Self::DebtLiabsCenBankFcSt => {
                "U.S. debt liabilities; central bank; foreign currency; short term"
            }
            Self::DebtLiabsCenBankLt => "U.S. debt liabilities; central bank; long term",
            Self::DebtLiabsCenBankOthFc => {
                "U.S. debt liabilities; central bank; other foreign currency"
            }
            Self::DebtLiabsCenBankOthFcLt => {
                "U.S. debt liabilities; central bank; other foreign currency; long term"
            }
            Self::DebtLiabsCenBankOthFcSt => {
                "U.S. debt liabilities; central bank; other foreign currency; short term"
            }
            Self::DebtLiabsCenBankSt => "U.S. debt liabilities; central bank; short term",
            Self::DebtLiabsCenBankUsd => "U.S. debt liabilities; central bank; U.S. dollar",
            Self::DebtLiabsCenBankUsdLt => {
                "U.S. debt liabilities; central bank; U.S. dollar; long term"
            }
            Self::DebtLiabsCenBankUsdSt => {
                "U.S. debt liabilities; central bank; U.S. dollar; short term"
            }
            Self::DebtLiabsCenBankYen => "U.S. debt liabilities; central bank; yen",
            Self::DebtLiabsCenBankYenLt => "U.S. debt liabilities; central bank; yen; long term",
            Self::DebtLiabsCenBankYenSt => "U.S. debt liabilities; central bank; yen; short term",
            Self::DebtLiabsDepExclCenBank => {
                "U.S. debt liabilities; deposit-taking institutions except central bank"
            }
            Self::DebtLiabsDepExclCenBankEuro => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; euro"
            }
            Self::DebtLiabsDepExclCenBankEuroLt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; euro; long term"
            }
            Self::DebtLiabsDepExclCenBankEuroSt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; euro; short term"
            }
            Self::DebtLiabsDepExclCenBankFc => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency"
            }
            Self::DebtLiabsDepExclCenBankFcLt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; long term"
            }
            Self::DebtLiabsDepExclCenBankFcSt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; short term"
            }
            Self::DebtLiabsDepExclCenBankLt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; long term"
            }
            Self::DebtLiabsDepExclCenBankOthFc => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency"
            }
            Self::DebtLiabsDepExclCenBankOthFcLt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; long term"
            }
            Self::DebtLiabsDepExclCenBankOthFcSt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; short term"
            }
            Self::DebtLiabsDepExclCenBankSt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; short term"
            }
            Self::DebtLiabsDepExclCenBankUsd => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar"
            }
            Self::DebtLiabsDepExclCenBankUsdLt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; long term"
            }
            Self::DebtLiabsDepExclCenBankUsdSt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; short term"
            }
            Self::DebtLiabsDepExclCenBankYen => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; yen"
            }
            Self::DebtLiabsDepExclCenBankYenLt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; yen; long term"
            }
            Self::DebtLiabsDepExclCenBankYenSt => {
                "U.S. debt liabilities; deposit-taking institutions except central bank; yen; short term"
            }
            Self::DebtLiabsEuro => "U.S. debt liabilities; euro",
            Self::DebtLiabsEuroLt => "U.S. debt liabilities; euro; long term",
            Self::DebtLiabsEuroSt => "U.S. debt liabilities; euro; short term",
            Self::DebtLiabsFc => "U.S. debt liabilities; foreign currency",
            Self::DebtLiabsFcLt => "U.S. debt liabilities; foreign currency; long term",
            Self::DebtLiabsFcSt => "U.S. debt liabilities; foreign currency; short term",
            Self::DebtLiabsGenGovt => "U.S. debt liabilities; general government",
            Self::DebtLiabsGenGovtEuro => "U.S. debt liabilities; general government; euro",
            Self::DebtLiabsGenGovtEuroLt => {
                "U.S. debt liabilities; general government; euro; long term"
            }
            Self::DebtLiabsGenGovtEuroSt => {
                "U.S. debt liabilities; general government; euro; short term"
            }
            Self::DebtLiabsGenGovtFc => {
                "U.S. debt liabilities; general government; foreign currency"
            }
            Self::DebtLiabsGenGovtFcLt => {
                "U.S. debt liabilities; general government; foreign currency; long term"
            }
            Self::DebtLiabsGenGovtFcSt => {
                "U.S. debt liabilities; general government; foreign currency; short term"
            }
            Self::DebtLiabsGenGovtLt => "U.S. debt liabilities; general government; long term",
            Self::DebtLiabsGenGovtOthFc => {
                "U.S. debt liabilities; general government; other foreign currency"
            }
            Self::DebtLiabsGenGovtOthFcLt => {
                "U.S. debt liabilities; general government; other foreign currency; long term"
            }
            Self::DebtLiabsGenGovtOthFcSt => {
                "U.S. debt liabilities; general government; other foreign currency; short term"
            }
            Self::DebtLiabsGenGovtSt => "U.S. debt liabilities; general government; short term",
            Self::DebtLiabsGenGovtUsd => "U.S. debt liabilities; general government; U.S. dollar",
            Self::DebtLiabsGenGovtUsdLt => {
                "U.S. debt liabilities; general government; U.S. dollar; long term"
            }
            Self::DebtLiabsGenGovtUsdSt => {
                "U.S. debt liabilities; general government; U.S. dollar; short term"
            }
            Self::DebtLiabsGenGovtYen => "U.S. debt liabilities; general government; yen",
            Self::DebtLiabsGenGovtYenLt => {
                "U.S. debt liabilities; general government; yen; long term"
            }
            Self::DebtLiabsGenGovtYenSt => {
                "U.S. debt liabilities; general government; yen; short term"
            }
            Self::DebtLiabsLt => "U.S. debt liabilities; long term",
            Self::DebtLiabsNonFinExclGenGovt => {
                "U.S. debt liabilities; nonfinancial institutions except general government"
            }
            Self::DebtLiabsNonFinExclGenGovtEuro => {
                "U.S. debt liabilities; nonfinancial institutions except general government; euro"
            }
            Self::DebtLiabsNonFinExclGenGovtEuroLt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; euro; long term"
            }
            Self::DebtLiabsNonFinExclGenGovtEuroSt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; euro; short term"
            }
            Self::DebtLiabsNonFinExclGenGovtFc => {
                "U.S. debt liabilities; nonfinancial institutions except general government; foreign currency"
            }
            Self::DebtLiabsNonFinExclGenGovtFcLt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; long term"
            }
            Self::DebtLiabsNonFinExclGenGovtFcSt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; short term"
            }
            Self::DebtLiabsNonFinExclGenGovtLt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; long term"
            }
            Self::DebtLiabsNonFinExclGenGovtOthFc => {
                "U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency"
            }
            Self::DebtLiabsNonFinExclGenGovtOthFcLt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; long term"
            }
            Self::DebtLiabsNonFinExclGenGovtOthFcSt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; short term"
            }
            Self::DebtLiabsNonFinExclGenGovtSt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; short term"
            }
            Self::DebtLiabsNonFinExclGenGovtUsd => {
                "U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar"
            }
            Self::DebtLiabsNonFinExclGenGovtUsdLt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; long term"
            }
            Self::DebtLiabsNonFinExclGenGovtUsdSt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; short term"
            }
            Self::DebtLiabsNonFinExclGenGovtYen => {
                "U.S. debt liabilities; nonfinancial institutions except general government; yen"
            }
            Self::DebtLiabsNonFinExclGenGovtYenLt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; yen; long term"
            }
            Self::DebtLiabsNonFinExclGenGovtYenSt => {
                "U.S. debt liabilities; nonfinancial institutions except general government; yen; short term"
            }
            Self::DebtLiabsOthFc => "U.S. debt liabilities; other foreign currency",
            Self::DebtLiabsOthFcLt => "U.S. debt liabilities; other foreign currency; long term",
            Self::DebtLiabsOthFcSt => "U.S. debt liabilities; other foreign currency; short term",
            Self::DebtLiabsOthFin => "U.S. debt liabilities; other financial institutions",
            Self::DebtLiabsOthFinEuro => {
                "U.S. debt liabilities; other financial institutions; euro"
            }
            Self::DebtLiabsOthFinEuroLt => {
                "U.S. debt liabilities; other financial institutions; euro; long term"
            }
            Self::DebtLiabsOthFinEuroSt => {
                "U.S. debt liabilities; other financial institutions; euro; short term"
            }
            Self::DebtLiabsOthFinFc => {
                "U.S. debt liabilities; other financial institutions; foreign currency"
            }
            Self::DebtLiabsOthFinFcLt => {
                "U.S. debt liabilities; other financial institutions; foreign currency; long term"
            }
            Self::DebtLiabsOthFinFcSt => {
                "U.S. debt liabilities; other financial institutions; foreign currency; short term"
            }
            Self::DebtLiabsOthFinLt => {
                "U.S. debt liabilities; other financial institutions; long term"
            }
            Self::DebtLiabsOthFinOthFc => {
                "U.S. debt liabilities; other financial institutions; other foreign currency"
            }
            Self::DebtLiabsOthFinOthFcLt => {
                "U.S. debt liabilities; other financial institutions; other foreign currency; long term"
            }
            Self::DebtLiabsOthFinOthFcSt => {
                "U.S. debt liabilities; other financial institutions; other foreign currency; short term"
            }
            Self::DebtLiabsOthFinSt => {
                "U.S. debt liabilities; other financial institutions; short term"
            }
            Self::DebtLiabsOthFinUsd => {
                "U.S. debt liabilities; other financial institutions; U.S. dollar"
            }
            Self::DebtLiabsOthFinUsdLt => {
                "U.S. debt liabilities; other financial institutions; U.S. dollar; long term"
            }
            Self::DebtLiabsOthFinUsdSt => {
                "U.S. debt liabilities; other financial institutions; U.S. dollar; short term"
            }
            Self::DebtLiabsOthFinYen => "U.S. debt liabilities; other financial institutions; yen",
            Self::DebtLiabsOthFinYenLt => {
                "U.S. debt liabilities; other financial institutions; yen; long term"
            }
            Self::DebtLiabsOthFinYenSt => {
                "U.S. debt liabilities; other financial institutions; yen; short term"
            }
            Self::DebtLiabsSt => "U.S. debt liabilities; short term",
            Self::DebtLiabsUsd => "U.S. debt liabilities; U.S. dollar",
            Self::DebtLiabsUsdLt => "U.S. debt liabilities; U.S. dollar; long term",
            Self::DebtLiabsUsdSt => "U.S. debt liabilities; U.S. dollar; short term",
            Self::DebtLiabsYen => "U.S. debt liabilities; yen",
            Self::DebtLiabsYenLt => "U.S. debt liabilities; yen; long term",
            Self::DebtLiabsYenSt => "U.S. debt liabilities; yen; short term",
            Self::DebtSecAssets => "U.S. assets; portfolio investment; debt securities",
            Self::DebtSecLiabs => "U.S. liabilities; portfolio investment; debt securities",
            Self::DebtSecLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; debt securities"
            }
            Self::DiInvAssets => {
                "U.S. assets; direct investment at market value, asset/liability basis"
            }
            Self::DiInvAssetsCurrCost => {
                "U.S. assets; direct investment at current cost, asset/liability basis"
            }
            Self::DiInvAssetsHistCostToMarketValueAdj => {
                "U.S. assets; direct investment; adjustment to revalue equity from historical cost to market value"
            }
            Self::DiInvAssetsNonSpe => {
                "U.S. assets; direct investment at market value, asset/liability basis; non-SPEs"
            }
            Self::DiInvAssetsSpe => {
                "U.S. assets; direct investment at market value, asset/liability basis; SPEs"
            }
            Self::DiInvDebtInstAssets => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments"
            }
            Self::DiInvDebtInstAssetsEuro => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; euro"
            }
            Self::DiInvDebtInstAssetsEuroLt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; euro; long term"
            }
            Self::DiInvDebtInstAssetsEuroSt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; euro; short term"
            }
            Self::DiInvDebtInstAssetsFc => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency"
            }
            Self::DiInvDebtInstAssetsFcLt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; long term"
            }
            Self::DiInvDebtInstAssetsFcSt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; short term"
            }
            Self::DiInvDebtInstAssetsLt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; long term"
            }
            Self::DiInvDebtInstAssetsNonSpe => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; non-SPEs"
            }
            Self::DiInvDebtInstAssetsOthFc => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency"
            }
            Self::DiInvDebtInstAssetsOthFcLt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; long term"
            }
            Self::DiInvDebtInstAssetsOthFcSt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; short term"
            }
            Self::DiInvDebtInstAssetsSpe => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; SPEs"
            }
            Self::DiInvDebtInstAssetsSt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; short term"
            }
            Self::DiInvDebtInstAssetsUsd => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar"
            }
            Self::DiInvDebtInstAssetsUsdLt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term"
            }
            Self::DiInvDebtInstAssetsUsdSt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term"
            }
            Self::DiInvDebtInstAssetsYen => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; yen"
            }
            Self::DiInvDebtInstAssetsYenLt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; yen; long term"
            }
            Self::DiInvDebtInstAssetsYenSt => {
                "U.S. assets; direct investment, asset/liability basis; debt instruments; yen; short term"
            }
            Self::DiInvDebtInstInward => {
                "Inward direct investment (foreign direct investment in the United States), directional basis; debt instruments"
            }
            Self::DiInvDebtInstLiabs => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments"
            }
            Self::DiInvDebtInstLiabsEuro => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro"
            }
            Self::DiInvDebtInstLiabsEuroLt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; long term"
            }
            Self::DiInvDebtInstLiabsEuroSt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; short term"
            }
            Self::DiInvDebtInstLiabsFc => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency"
            }
            Self::DiInvDebtInstLiabsFcLt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; long term"
            }
            Self::DiInvDebtInstLiabsFcSt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; short term"
            }
            Self::DiInvDebtInstLiabsLt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; long term"
            }
            Self::DiInvDebtInstLiabsNonSpe => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; non-SPEs"
            }
            Self::DiInvDebtInstLiabsOthFc => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency"
            }
            Self::DiInvDebtInstLiabsOthFcLt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; long term"
            }
            Self::DiInvDebtInstLiabsOthFcSt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; short term"
            }
            Self::DiInvDebtInstLiabsSpe => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; SPEs"
            }
            Self::DiInvDebtInstLiabsSt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; short term"
            }
            Self::DiInvDebtInstLiabsUsd => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar"
            }
            Self::DiInvDebtInstLiabsUsdLt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term"
            }
            Self::DiInvDebtInstLiabsUsdSt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term"
            }
            Self::DiInvDebtInstLiabsYen => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen"
            }
            Self::DiInvDebtInstLiabsYenLt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; long term"
            }
            Self::DiInvDebtInstLiabsYenSt => {
                "U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; short term"
            }
            Self::DiInvDebtInstOutward => {
                "Outward direct investment (U.S. direct investment abroad), directional basis; debt instruments"
            }
            Self::DiInvDebtInstUsAffiliatesClaims => {
                "Direct investment; debt instruments; U.S. affiliates' claims"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsByNonSpe => {
                "U.S. non-SPE affiliates' debt asset position in their foreign parent groups"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsBySpe => {
                "U.S. SPE affiliates' debt asset position in their foreign parent groups"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsNonSpe => {
                "Direct investment; debt instruments; U.S. affiliates' claims; non-SPEs"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsSpe => {
                "Direct investment; debt instruments; U.S. affiliates' claims; SPEs"
            }
            Self::DiInvDebtInstUsAffiliatesLiabs => {
                "Direct investment; debt instruments; U.S. affiliates' liabilites"
            }
            Self::DiInvDebtInstUsAffiliatesLiabsNonSpe => {
                "Direct investment; debt instruments; U.S. affiliates' liabilites; non-SPEs"
            }
            Self::DiInvDebtInstUsAffiliatesLiabsSpe => {
                "Direct investment; debt instruments; U.S. affiliates' liabilites; SPEs"
            }
            Self::DiInvDebtInstUsParentsClaims => {
                "Direct investment; debt instruments; U.S. parents' claims"
            }
            Self::DiInvDebtInstUsParentsClaimsNonSpe => {
                "Direct investment; debt instruments; U.S. parents' claims; non-SPEs"
            }
            Self::DiInvDebtInstUsParentsClaimsSpe => {
                "Direct investment; debt instruments; U.S. parents' claims; SPEs"
            }
            Self::DiInvDebtInstUsParentsLiabs => {
                "Direct investment; debt instruments; U.S. parents' liabilites"
            }
            Self::DiInvDebtInstUsParentsLiabsInNonSpe => {
                "U.S. parents' debt liability position in their foreign non-SPE affiliates"
            }
            Self::DiInvDebtInstUsParentsLiabsInSpe => {
                "U.S. parents' debt liability position in their foreign SPE affiliates"
            }
            Self::DiInvDebtInstUsParentsLiabsNonSpe => {
                "Direct investment; debt instruments; U.S. parents' liabilites; non-SPEs"
            }
            Self::DiInvDebtInstUsParentsLiabsSpe => {
                "Direct investment; debt instruments; U.S. parents' liabilites; SPEs"
            }
            Self::DiInvDirectionalBasisAdj => {
                "Direct investment; adjustments to convert to directional basis"
            }
            Self::DiInvEquityAssets => "U.S. assets; direct investment at market value; equity",
            Self::DiInvEquityAssetsCurrCost => {
                "U.S. assets; direct investment at current cost; equity"
            }
            Self::DiInvEquityAssetsHistCost => {
                "U.S. assets; direct investment at historical cost; equity"
            }
            Self::DiInvEquityAssetsNonSpe => {
                "U.S. assets; direct investment at market value; equity; non-SPEs"
            }
            Self::DiInvEquityAssetsSpe => {
                "U.S. assets; direct investment at market value; equity; SPEs"
            }
            Self::DiInvEquityInward => {
                "U.S. liabilities; direct investment at market value; equity"
            }
            Self::DiInvEquityLiabs => "U.S. liabilities; direct investment at market value; equity",
            Self::DiInvEquityLiabsCurrCost => {
                "U.S. liabilities; direct investment at current cost; equity"
            }
            Self::DiInvEquityLiabsHistCost => {
                "U.S. liabilities; direct investment at historical cost; equity"
            }
            Self::DiInvEquityLiabsNonSpe => {
                "U.S. liabilities; direct investment at market value; equity; non-SPEs"
            }
            Self::DiInvEquityLiabsSpe => {
                "U.S. liabilities; direct investment at market value; equity; SPEs"
            }
            Self::DiInvInwardCurrCost => {
                "Inward direct investment (foreign direct investment in the United States) at current cost, directional basis"
            }
            Self::DiInvInwardHistCost => {
                "Inward direct investment (foreign direct investment in the United States) at historical cost, directional basis"
            }
            Self::DiInvInwardMarketValue => {
                "Inward direct investment (foreign direct investment in the United States) at market value, directional basis"
            }
            Self::DiInvLiabs => {
                "U.S. liabilities; direct investment at market value, asset/liability basis"
            }
            Self::DiInvLiabsCurrCost => {
                "U.S. liabilities; direct investment at current cost, asset/liability basis"
            }
            Self::DiInvLiabsHistCostToMarketValueAdj => {
                "U.S. liabilities; direct investment; adjustment to revalue equity from historical cost to market value"
            }
            Self::DiInvLiabsNonSpe => {
                "U.S. liabilities; direct investment at market value, asset/liability basis; non-SPEs"
            }
            Self::DiInvLiabsSpe => {
                "U.S. liabilities; direct investment at market value, asset/liability basis; SPEs"
            }
            Self::DiInvOutwardCurrCost => {
                "Outward direct investment (U.S. direct investment abroad) at current cost, directional basis"
            }
            Self::DiInvOutwardHistCost => {
                "Outward direct investment (U.S. direct investment abroad) at historical cost, directional basis"
            }
            Self::DiInvOutwardMarketValue => {
                "Outward direct investment (U.S. direct investment abroad) at market value, directional basis"
            }
            Self::EquityAndInvFundSharesAssets => {
                "U.S. assets; portfolio investment; equity and investment fund shares"
            }
            Self::EquityAndInvFundSharesLiabs => {
                "U.S. liabilities; portfolio investment; equity and investment fund shares"
            }
            Self::EquityAndInvFundSharesLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; equity and investment fund shares"
            }
            Self::FinAssets => "U.S. assets",
            Self::FinAssetsExclFinDeriv => "U.S. assets excluding financial derivatives",
            Self::FinDerivAssets => {
                "U.S. assets; financial derivatives other than reserves, gross positive fair value"
            }
            Self::FinDerivExchTradedAssets => {
                "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts"
            }
            Self::FinDerivExchTradedLiabs => {
                "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts"
            }
            Self::FinDerivForExAssets => {
                "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts"
            }
            Self::FinDerivForExLiabs => {
                "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts"
            }
            Self::FinDerivLiabs => {
                "U.S. liabilities; financial derivatives other than reserves, gross negative fair value"
            }
            Self::FinDerivNet => {
                "U.S. net international investment position; financial derivatives other than reserves"
            }
            Self::FinDerivOtcAssets => {
                "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts"
            }
            Self::FinDerivOtcLiabs => {
                "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts"
            }
            Self::FinDerivOthAssets => {
                "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts"
            }
            Self::FinDerivOthLiabs => {
                "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts"
            }
            Self::FinDerivReserveAssets => {
                "U.S. assets; other reserve assets; financial derivatives"
            }
            Self::FinDerivSingleCurrAssets => {
                "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts"
            }
            Self::FinDerivSingleCurrLiabs => {
                "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts"
            }
            Self::FinLiabs => "U.S. liabilities",
            Self::FinLiabsExclFinDeriv => "U.S. liabilities excluding financial derivatives",
            Self::FinLiabsFoa => "U.S. liabilities to foreign official agencies",
            Self::GoldReserveAssets => "U.S. assets; reserve assets; monetary gold",
            Self::ImfReserveAssets => {
                "U.S. assets; reserve assets; reserve position in the International Monetary Fund"
            }
            Self::InsTechReservesAssets => {
                "U.S. assets; other investment; insurance technical reserves"
            }
            Self::InsTechReservesLiabs => {
                "U.S. liabilities; other investment; insurance technical reserves"
            }
            Self::LoansAssets => "U.S. assets; other investment; loans",
            Self::LoansLiabs => "U.S. liabilities; other investment; loans",
            Self::LoansLiabsFoa => {
                "U.S. liabilities to foreign official agencies; other investment; loans"
            }
            Self::LtDebtSecAssets => "U.S. assets; portfolio investment; long-term debt securities",
            Self::LtDebtSecLiabs => {
                "U.S. liabilities; portfolio investment; long-term debt securities"
            }
            Self::LtDebtSecLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities"
            }
            Self::LtDebtSecTreasLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities; Treasury bonds and notes"
            }
            Self::Net => "U.S. net international investment position",
            Self::NetExclFinDeriv => {
                "U.S. net international investment position excluding financial derivatives"
            }
            Self::OthClmReserveAssets => "U.S. assets; other reserve assets; other claims",
            Self::OthEquityAssets => "U.S. assets; other investment; other equity",
            Self::OthEquityLiabs => "U.S. liabilities; other investment; other equity",
            Self::OthEquityLiabsFoa => {
                "U.S. liabilities to foreign official agencies; other investment; other equity"
            }
            Self::OthInvAssets => "U.S. assets; other investment",
            Self::OthInvLiabs => "U.S. liabilities; other investment",
            Self::OthInvLiabsFoa => {
                "U.S. liabilities to foreign official agencies; other investment"
            }
            Self::OthLtDebtSecLiabs => {
                "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes"
            }
            Self::OthLtDebtSecLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities excluding Treasury bonds and notes"
            }
            Self::OthReserveAssets => "U.S. assets; other reserve assets",
            Self::OthStDebtSecLiabs => {
                "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates"
            }
            Self::OthStDebtSecLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities excluding Treasury bills and certificates"
            }
            Self::PfInvAssets => "U.S. assets; portfolio investment",
            Self::PfInvLiabs => "U.S. liabilities; portfolio investment",
            Self::PfInvLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment"
            }
            Self::ReserveAssets => "U.S. assets; reserve assets",
            Self::ReserveAssetsCenBank => "U.S. assets; reserve assets; central bank",
            Self::ReserveAssetsCenBankLt => "U.S. assets; reserve assets; central bank; long term",
            Self::ReserveAssetsCenBankNotSdrBasket => {
                "U.S. assets; reserve assets; central bank; not in special drawing rights basket"
            }
            Self::ReserveAssetsCenBankNotSdrBasketLt => {
                "U.S. assets; reserve assets; central bank; not in special drawing rights basket; long term"
            }
            Self::ReserveAssetsCenBankNotSdrBasketSt => {
                "U.S. assets; reserve assets; central bank; not in special drawing rights basket; short term"
            }
            Self::ReserveAssetsCenBankSdrBasket => {
                "U.S. assets; reserve assets; central bank; in special drawing rights basket"
            }
            Self::ReserveAssetsCenBankSdrBasketLt => {
                "U.S. assets; reserve assets; central bank; in special drawing rights basket; long term"
            }
            Self::ReserveAssetsCenBankSdrBasketSt => {
                "U.S. assets; reserve assets; central bank; in special drawing rights basket; short term"
            }
            Self::ReserveAssetsCenBankSt => "U.S. assets; reserve assets; central bank; short term",
            Self::ReserveAssetsGenGovt => "U.S. assets; reserve assets; general government",
            Self::ReserveAssetsGenGovtLt => {
                "U.S. assets; reserve assets; general government; long term"
            }
            Self::ReserveAssetsGenGovtNotSdrBasket => {
                "U.S. assets; reserve assets; general government; not in special drawing rights basket"
            }
            Self::ReserveAssetsGenGovtNotSdrBasketLt => {
                "U.S. assets; reserve assets; general government; not in special drawing rights basket; long term"
            }
            Self::ReserveAssetsGenGovtNotSdrBasketSt => {
                "U.S. assets; reserve assets; general government; not in special drawing rights basket; short term"
            }
            Self::ReserveAssetsGenGovtSdrBasket => {
                "U.S. assets; reserve assets; general government; in special drawing rights basket"
            }
            Self::ReserveAssetsGenGovtSdrBasketLt => {
                "U.S. assets; reserve assets; general government; in special drawing rights basket; long term"
            }
            Self::ReserveAssetsGenGovtSdrBasketSt => {
                "U.S. assets; reserve assets; general government; in special drawing rights basket; short term"
            }
            Self::ReserveAssetsGenGovtSt => {
                "U.S. assets; reserve assets; general government; short term"
            }
            Self::ReserveAssetsLt => "U.S. assets; reserve assets; long term",
            Self::ReserveAssetsNotSdrBasket => {
                "U.S. assets; reserve assets; not in special drawing rights basket"
            }
            Self::ReserveAssetsNotSdrBasketLt => {
                "U.S. assets; reserve assets; not in special drawing rights basket; long term"
            }
            Self::ReserveAssetsNotSdrBasketSt => {
                "U.S. assets; reserve assets; not in special drawing rights basket; short term"
            }
            Self::ReserveAssetsSdrBasket => {
                "U.S. assets; reserve assets; in special drawing rights basket"
            }
            Self::ReserveAssetsSdrBasketLt => {
                "U.S. assets; reserve assets; in special drawing rights basket; long term"
            }
            Self::ReserveAssetsSdrBasketSt => {
                "U.S. assets; reserve assets; in special drawing rights basket; short term"
            }
            Self::ReserveAssetsSt => "U.S. assets; reserve assets; short term",
            Self::SdrAllocLiabs => {
                "U.S. liabilities; other investment; special drawing rights allocations"
            }
            Self::SdrAllocLiabsFoa => {
                "U.S. liabilities to foreign official agencies; other investment; special drawing rights allocations"
            }
            Self::SdrReserveAssets => "U.S. assets; reserve assets; special drawing rights",
            Self::SecReserveAssets => "U.S. assets; other reserve assets; securities",
            Self::StDebtSecAssets => {
                "U.S. assets; portfolio investment; short-term debt securities"
            }
            Self::StDebtSecLiabs => {
                "U.S. liabilities; portfolio investment; short-term debt securities"
            }
            Self::StDebtSecLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities"
            }
            Self::StDebtSecTreasLiabsFoa => {
                "U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities; Treasury bills and certificates"
            }
            Self::TrdCredAndAdvAssets => "U.S. assets; other investment; trade credit and advances",
            Self::TrdCredAndAdvLiabs => {
                "U.S. liabilities; other investment; trade credit and advances"
            }
            Self::TrdCredAndAdvLiabsFoa => {
                "U.S. liabilities to foreign official agencies; other investment; trade credit and advances"
            }
            Self::TreasBillsAndCertsLiabs => {
                "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates"
            }
            Self::TreasBondsAndNotesLiabs => {
                "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes"
            }
        }
    }

    /// The String representation of a variant serves as the value for the corresponding parameter
    /// name in BEA API calls. The `params` method formats the parameter name as a key, as the
    /// variant name as the value for use in building complex API queries.
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::TypeOfInvestment.to_string();
        let value = self.to_string();
        (key, value)
    }
}
