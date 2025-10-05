use crate::{
    BeaErr, KeyMissing, Measure, ParameterFields, ParameterName, ParameterValueTable,
    ParameterValueTableVariant, ParseInt, Scale,
};

/// Represents different types of input-output tables in an economic analysis framework.
///
/// These tables are categorized by their relationship type (Commodity-by-Commodity,
/// Industry-by-Commodity, Industry-by-Industry) and their level of detail (Summary or Sector).
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
)]
pub enum InputOutputTable {
    /// Commodity-by-Commodity Total Requirements, After Redefinitions - Summary (Key: 59)
    CommodityTotalRequirementsSummary,
    /// Commodity-by-Commodity Total Requirements, After Redefinitions - Sector (Key: 58)
    CommodityTotalRequirementsSector,
    /// Industry-by-Commodity Total Requirements, After Redefinitions - Summary (Key: 57)
    IndustryByCommodityRequirementsSummary,
    /// Industry-by-Commodity Total Requirements, After Redefinitions - Sector (Key: 56)
    IndustryByCommodityRequirementsSector,
    /// Industry-by-Industry Total Requirements, After Redefinitions - Summary (Key: 61)
    IndustryTotalRequirementsSummary,
    /// Industry-by-Industry Total Requirements, After Redefinitions - Sector (Key: 60)
    IndustryTotalRequirementsSector,
    /// The Domestic Supply of Commodities by Industries - Summary (Key: 262)
    DomesticSupplySummary,
    /// The Domestic Supply of Commodities by Industries - Sector (Key: 261)
    DomesticSupplySector,
    /// The Use of Commodities by Industries - Summary (Key: 259)
    CommodityUseSummary,
    /// The Use of Commodities by Industries - Sector (Key: 258)
    CommodityUseSector,
}

impl InputOutputTable {
    /// Format `self` for insertion into a request BTreeMap(key, value).
    /// The key is the parameter name.  The value is the parameter value.
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::TableID.to_string();
        let value = self.key().to_string();
        (key, value)
    }

    /// Returns the full description of the input-output table type.
    ///
    /// # Returns
    ///
    /// A static string containing the official description of the table type.
    pub fn description(&self) -> &'static str {
        match self {
            Self::CommodityTotalRequirementsSummary => {
                "Commodity-by-Commodity Total Requirements, After Redefinitions - Summary"
            }
            Self::CommodityTotalRequirementsSector => {
                "Commodity-by-Commodity Total Requirements, After Redefinitions - Sector"
            }
            Self::IndustryByCommodityRequirementsSummary => {
                "Industry-by-Commodity Total Requirements, After Redefinitions - Summary"
            }
            Self::IndustryByCommodityRequirementsSector => {
                "Industry-by-Commodity Total Requirements, After Redefinitions - Sector"
            }
            Self::IndustryTotalRequirementsSummary => {
                "Industry-by-Industry Total Requirements, After Redefinitions - Summary"
            }
            Self::IndustryTotalRequirementsSector => {
                "Industry-by-Industry Total Requirements, After Redefinitions - Sector"
            }
            Self::DomesticSupplySummary => {
                "The Domestic Supply of Commodities by Industries - Summary"
            }
            Self::DomesticSupplySector => {
                "The Domestic Supply of Commodities by Industries - Sector"
            }
            Self::CommodityUseSummary => "The Use of Commodities by Industries - Summary",
            Self::CommodityUseSector => "The Use of Commodities by Industries - Sector",
        }
    }

    /// Returns the numeric key associated with the input-output table type.
    ///
    /// # Returns
    ///
    /// An integer representing the key identifier for the table type.
    pub fn key(&self) -> i64 {
        match self {
            Self::CommodityTotalRequirementsSummary => 59,
            Self::CommodityTotalRequirementsSector => 58,
            Self::IndustryByCommodityRequirementsSummary => 57,
            Self::IndustryByCommodityRequirementsSector => 56,
            Self::IndustryTotalRequirementsSummary => 61,
            Self::IndustryTotalRequirementsSector => 60,
            Self::DomesticSupplySummary => 262,
            Self::DomesticSupplySector => 261,
            Self::CommodityUseSummary => 259,
            Self::CommodityUseSector => 258,
        }
    }

    /// Creates an InputOutputTable instance by parsing a string key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice containing the key to parse
    ///
    /// # Returns
    ///
    /// * `Ok(InputOutputTable)` - If the key is valid and matches a known table type
    /// * `Err(BeaErr)` - If the key cannot be parsed as i64 (ParseInt error) or if the key is not found (KeyMissing error)
    ///
    /// # Errors
    ///
    /// Returns a `ParseInt` error if the string cannot be parsed as i64.
    /// Returns a `KeyMissing` error if the parsed integer does not match any known key.
    #[tracing::instrument]
    pub fn from_key(key: &str) -> Result<Self, BeaErr> {
        let result = match key
            .parse::<i64>()
            .map_err(|e| ParseInt::new(key.to_owned(), e, line!(), file!().to_string()))?
        {
            59 => Self::CommodityTotalRequirementsSummary,
            58 => Self::CommodityTotalRequirementsSector,
            57 => Self::IndustryByCommodityRequirementsSummary,
            56 => Self::IndustryByCommodityRequirementsSector,
            61 => Self::IndustryTotalRequirementsSummary,
            60 => Self::IndustryTotalRequirementsSector,
            262 => Self::DomesticSupplySummary,
            261 => Self::DomesticSupplySector,
            259 => Self::CommodityUseSummary,
            258 => Self::CommodityUseSector,
            _ => return Err(KeyMissing::new(key.to_owned(), line!(), file!().to_owned()).into()),
        };
        Ok(result)
    }

    /// Returns the [`Measure`] for the table's data values.
    ///
    /// Input-Output tables contain two distinct types of data:
    /// - Requirements tables contain coefficients (ratios) showing the amount of input
    ///   required per dollar of output
    /// - Supply and Use tables contain actual dollar values measured in millions
    ///
    /// # Returns
    ///
    /// * `Measure::Level` - For requirements tables that contain dimensionless ratios/coefficients
    /// * `Measure::Usd` - For supply and use tables that contain dollar values in millions (UNIT_MULT = 6)
    ///
    /// # Examples
    ///
    /// ```
    /// use bears_species::{InputOutputTable, Measure};
    ///
    /// let table = InputOutputTable::CommodityTotalRequirementsSummary;
    /// assert_eq!(table.measure(), Measure::Level); // Requirements = ratios
    ///
    /// let table = InputOutputTable::DomesticSupplySummary;
    /// assert_eq!(table.measure(), Measure::Usd); // Supply = millions of dollars
    /// ```
    #[tracing::instrument(skip_all)]
    pub fn measure(&self) -> Measure {
        match self {
            Self::CommodityTotalRequirementsSummary
            | Self::CommodityTotalRequirementsSector
            | Self::IndustryByCommodityRequirementsSummary
            | Self::IndustryByCommodityRequirementsSector
            | Self::IndustryTotalRequirementsSummary
            | Self::IndustryTotalRequirementsSector => Measure::Usd,
            Self::DomesticSupplySummary
            | Self::DomesticSupplySector
            | Self::CommodityUseSummary
            | Self::CommodityUseSector => Measure::Level,
        }
    }

    /// Returns the [`Scale`] (unit multiplier) for the table's data values.
    ///
    /// The scale indicates the factor (power of 10) applied to raw data values for interpretation.
    /// Input-Output tables use different scales depending on whether they contain dollar values
    /// or dimensionless ratios.
    ///
    /// # Returns
    ///
    /// * `Scale::Unit` - For requirements tables containing ratios/coefficients (no multiplier, UNIT_MULT = 0)
    /// * `Scale::Million` - For supply and use tables containing dollar values in millions (UNIT_MULT = 6)
    ///
    /// # Examples
    ///
    /// ```
    /// use bears_species::{InputOutputTable, Scale};
    ///
    /// let table = InputOutputTable::CommodityTotalRequirementsSummary;
    /// assert_eq!(table.scale(), Scale::Unit); // Requirements = no multiplier
    ///
    /// let table = InputOutputTable::DomesticSupplySummary;
    /// assert_eq!(table.scale(), Scale::Million); // Supply = millions of dollars
    /// ```
    #[tracing::instrument(skip_all)]
    pub fn scale(&self) -> Scale {
        match self {
            Self::CommodityTotalRequirementsSummary
            | Self::CommodityTotalRequirementsSector
            | Self::IndustryByCommodityRequirementsSummary
            | Self::IndustryByCommodityRequirementsSector
            | Self::IndustryTotalRequirementsSummary
            | Self::IndustryTotalRequirementsSector => Scale::Million,
            Self::DomesticSupplySummary
            | Self::DomesticSupplySector
            | Self::CommodityUseSummary
            | Self::CommodityUseSector => Scale::Unit,
        }
    }
}

impl TryFrom<&ParameterFields> for InputOutputTable {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        Self::from_key(value.key())
    }
}

impl TryFrom<&ParameterValueTable> for InputOutputTable {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            _ => {
                let msg = format!("ParameterFields needed, found {value:#?}");
                let error = ParameterValueTableVariant::new(msg, line!(), file!().to_owned());
                Err(error.into())
            }
        }
    }
}
