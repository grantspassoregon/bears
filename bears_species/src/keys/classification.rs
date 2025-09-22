use crate::ParameterName;

#[derive(
    Debug,
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
pub enum Classification {
    /// Set to classification equal to 'country' to obtain data for all countries for which data are available for the given series, ownership, and year.
    Country,
    /// CountryByDestination returns values for country broken out by destination for goods supplied and services supplied.
    CountryByDestination,
    /// CountryByIndustry provides data for a large set of countries broken down by major industry.
    CountryByIndustry,
    /// Country by Type of Investment returns data on new foreign direct investment in the United States broken out by country of UBO and the type of investment (acquisition, establishment, or expansion).
    CountrybyType,
    /// â€" Presents data on foreign direct investment in the United States by the country of the Ultimate Beneficial Owner (UBO).  The ultimate beneficial owner (UBO) is the entity, proceeding up the foreign ownership chain, which is not more than 50 percent owned by another entity. The UBO is the entity that ultimately owns or controls an affiliated and thus ultimately derives the benefits and assumes the risks from owning or controlling an affiliate.
    CountryofUBO,
    /// Presents data on services supplied by U.S. affiliates of foreign MNEâ€™s, broken down by the country of Ultimate Beneficial Owner (UBO) and the destination (U.S. or foreign) of the service supplied
    CountryofUBObyDestination,
    /// Presents data on foreign direct investment in the United States broken down by the country of the Ultimate Beneficial Owner (UBO) and the industry of the affiliate.
    CountryofUBObyIndustry,
    /// Country of UBO by Type of Investment returns data on new foreign direct investment in the United States broken out by the country of the Ultimate beneficial owner (UBO) and the type of investment (acquisition, establishment, or expansion).
    #[display("countryofUBObyType")]
    CountryofUBObyType,
    /// Set to classification equal to 'industry' to obtain data for all industries for which data are available for the given series, ownership, and year.
    Industry,
    /// IndustryByCountry provides data for a large set of industries broken by major country.
    IndustryByCountry,
    /// Presents foreign direct investment data broken down by industry of the U.S. affiliate and the country of Ultimate Beneficial Owner (UBO)
    IndustrybyCountryofUBO,
    /// IndustryByDestination returns values for industry broken out by destination for goods supplied and services supplied.
    IndustryByDestination,
    /// Industry by Type of Investment returns data on new foreign direct investment in the United States broken out by industry of U.S. business and the type of investment (acquisition, establishment, or expansion).
    IndustrybyType,
    /// Industry of Sales provides data on employment and sales for majority-owned U.S. affiliates of foreign MNEs.  Each majority-owned U.S. affiliate above a minimum size threshold was required to distribute its sales and its employment among the four-digit industry in which it had sales.  These data approximate an establishment-based distribution.
    IndustryofSales,
    /// = IndustryofUSParent provides data for foreign affiliates of U.S. parents, broken down by the industry of the U.S. parent, which may differ from the industry of the affiliate.
    IndustryofUSParent,
    /// IndustryofUSParentAllIndustries provides data for foreign affiliates of U.S. parents, broken down by the most detailed industry of the U.S. parent, which may differ from the industry of the affiliate.
    IndustryofUSParentAllIndustries,
    /// IndustryofUSParentByCountry provides data for foreign affiliates of U.S. parents, broken down by the industry of the U.S. parent, which may differ from the industry of the affiliate, and broken by major country.
    IndustryofUSParentByCountry,
    /// Presents selected activities of multinational enterprises data on U.S. affiliates, broken down U.S. state and country of UBO.
    StatebyCountryofUBO,
    /// State by Type of Investment returns data on new foreign direct investment in the United States broken out by destination state and the type of investment (acquisition, establishment, or expansion).
    StatebyType,
    /// Type of Expenditure returns values on investment expenditure for new foreign direct investment in the United States, broken out by type of expenditure.
    TypeofExpenditure,
}

impl Classification {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Country => "Set to classification equal to 'country' to obtain data for all countries for which data are available for the given series, ownership, and year.",
            Self::CountryByDestination => "CountryByDestination returns values for country broken out by destination for goods supplied and services supplied.",
            Self::CountryByIndustry => "CountryByIndustry provides data for a large set of countries broken down by major industry.",
            Self::CountrybyType => "Country by Type of Investment returns data on new foreign direct investment in the United States broken out by country of UBO and the type of investment (acquisition, establishment, or expansion).",
            Self::CountryofUBO => "— Presents data on foreign direct investment in the United States by the country of the Ultimate Beneficial Owner (UBO).  The ultimate beneficial owner (UBO) is the entity, proceeding up the foreign ownership chain, which is not more than 50 percent owned by another entity. The UBO is the entity that ultimately owns or controls an affiliated and thus ultimately derives the benefits and assumes the risks from owning or controlling an affiliate.",
            Self::CountryofUBObyDestination => "Presents data on services supplied by U.S. affiliates of foreign MNE's, broken down by the country of Ultimate Beneficial Owner (UBO) and the destination (U.S. or foreign) of the service supplied",
            Self::CountryofUBObyIndustry => "Presents data on foreign direct investment in the United States broken down by the country of the Ultimate Beneficial Owner (UBO) and the industry of the affiliate.",
            Self::CountryofUBObyType => "Country of UBO by Type of Investment returns data on new foreign direct investment in the United States broken out by the country of the Ultimate beneficial owner (UBO) and the type of investment (acquisition, establishment, or expansion).",
            Self::Industry => "Set to classification equal to 'industry' to obtain data for all industries for which data are available for the given series, ownership, and year.",
            Self::IndustryByCountry => "IndustryByCountry provides data for a large set of industries broken by major country.",
            Self::IndustrybyCountryofUBO => "Presents foreign direct investment data broken down by industry of the U.S. affiliate and the country of Ultimate Beneficial Owner (UBO)",
            Self::IndustryByDestination => "IndustryByDestination returns values for industry broken out by destination for goods supplied and services supplied.",
            Self::IndustrybyType => "Industry by Type of Investment returns data on new foreign direct investment in the United States broken out by industry of U.S. business and the type of investment (acquisition, establishment, or expansion).",
            Self::IndustryofSales => "Industry of Sales provides data on employment and sales for majority-owned U.S. affiliates of foreign MNEs.  Each majority-owned U.S. affiliate above a minimum size threshold was required to distribute its sales and its employment among the four-digit industry in which it had sales.  These data approximate an establishment-based distribution.",
            Self::IndustryofUSParent => "= IndustryofUSParent provides data for foreign affiliates of U.S. parents, broken down by the industry of the U.S. parent, which may differ from the industry of the affiliate.",
            Self::IndustryofUSParentAllIndustries => "IndustryofUSParentAllIndustries provides data for foreign affiliates of U.S. parents, broken down by the most detailed industry of the U.S. parent, which may differ from the industry of the affiliate.",
            Self::IndustryofUSParentByCountry => "IndustryofUSParentByCountry provides data for foreign affiliates of U.S. parents, broken down by the industry of the U.S. parent, which may differ from the industry of the affiliate, and broken by major country.",
            Self::StatebyCountryofUBO => "Presents selected activities of multinational enterprises data on U.S. affiliates, broken down U.S. state and country of UBO.",
            Self::StatebyType => "State by Type of Investment returns data on new foreign direct investment in the United States broken out by destination state and the type of investment (acquisition, establishment, or expansion).",
            Self::TypeofExpenditure => "Type of Expenditure returns values on investment expenditure for new foreign direct investment in the United States, broken out by type of expenditure.",
        }
    }

    /// The String representation of a variant serves as the value for the corresponding parameter
    /// name in BEA API calls. The `params` method formats the parameter name as a key, as the
    /// variant name as the value for use in building complex API queries.
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::Classification.to_string();
        let value = self.to_string();
        (key, value)
    }
}
