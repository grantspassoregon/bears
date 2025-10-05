/// Metrics associated with the Fixed Assets dataset
/// ([`Dataset::FixedAssets`](crate::Dataset::FixedAssets))
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
pub enum Metric {
    /// Chained Dollars
    ChainedDollars,
    /// Current Dollars
    #[default]
    CurrentDollars,
    /// Fisher Quantity Index
    FisherQuantityIndex,
    /// Historical Cost
    HistoricalCost,
    /// Year
    Year,
}

impl Metric {
    /// Returns the BEA description associated with the variant.
    #[tracing::instrument]
    pub fn description(&self) -> &'static str {
        match self {
            Self::ChainedDollars => "Chained Dollars",
            Self::CurrentDollars => "Current Dollars",
            Self::FisherQuantityIndex => "Fisher Quantity Index",
            Self::HistoricalCost => "Historical Cost",
            Self::Year => "Year",
        }
    }

    pub fn from_key(key: &str) -> Result<Self, BadMetric> {
        let metric = match key {
            "Chained Dollars" => Self::ChainedDollars,
            "Current Dollars" => Self::CurrentDollars,
            "Fisher Quantity Index" => Self::FisherQuantityIndex,
            "Historical Cost" => Self::HistoricalCost,
            "Year" => Self::Year,
            _ => {
                return Err(BadMetric::new(key.to_owned(), line!(), file!().to_owned()));
            }
        };
        Ok(metric)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("bad Metric value: {value} at line {line} in {file}")]
pub struct BadMetric {
    value: String,
    line: u32,
    file: String,
}

impl std::error::Error for BadMetric {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
