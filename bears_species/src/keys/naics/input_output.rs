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
pub enum NaicsInputOutput {
    /// All industries
    AllIndustries,
    /// All other retail
    AllOtherRetail,
    /// All other transportation equipment manufacturing
    AllOtherTransportationEquipmentManufacturing,
    /// Apparel and leather and allied products
    ApparelLeatherAlliedProducts,
    /// Change in private inventories
    ChangeInPrivateInventories,
    /// CIF/FOB Adjustments on Imports
    CifFobAdjustmentsOnImports,
    /// Compensation of employees
    CompensationOfEmployees,
    /// Customs duties
    CustomsDuties,
    /// Durable goods
    DurableGoods,
    /// Education, hospital, and health structures
    EducationHospitalHealthStructures,
    /// Exports of goods and services
    ExportsGoodsServices,
    /// Farms
    Farms,
    /// Federal
    Federal,
    /// Federal general government (defense)
    FederalGeneralGovernmentDefense,
    /// Federal general government (nondefense)
    FederalGeneralGovernmentNondefense,
    /// Federal government enterprises
    FederalGovernmentEnterprises,
    /// Federal national defense: Gross investment in equipment
    FederalNationalDefenseGrossInvestmentEquipment,
    /// Federal national defense: Gross investment in intellectual property products
    FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts,
    /// Federal national defense: Gross investment in structures
    FederalNationalDefenseGrossInvestmentStructures,
    /// Federal nondefense: Gross investment in equipment
    FederalNondefenseGrossInvestmentEquipment,
    /// Federal nondefense: Gross investment in intellectual property products
    FederalNondefenseGrossInvestmentIntellectualPropertyProducts,
    /// Federal nondefense: Gross investment in structures
    FederalNondefenseGrossInvestmentStructures,
    /// Federal Reserve banks, credit intermediation, and related activities
    FederalReserveBanksCreditIntermediationRelatedActivities,
    /// Finance, insurance, real estate, rental, and leasing
    FinanceInsuranceRealEstateRentalLeasing,
    /// Food and beverage and tobacco products
    FoodBeverageTobaccoProducts,
    /// Forestry, fishing, and related activities
    ForestryFishingRelatedActivities,
    /// General government
    GeneralGovernment,
    /// Government
    Government,
    /// Government consumption expenditures and gross investment
    GovernmentConsumptionExpendituresGrossInvestment,
    /// Gross domestic product
    GrossDomesticProduct,
    /// Gross operating surplus
    GrossOperatingSurplus,
    /// Hospitals and nursing and residential care facilities (A,Q)
    HospitalsNursingResidentialCareFacilities,
    /// Housing
    Housing,
    /// Import duties
    ImportDuties,
    /// Imports
    Imports,
    /// Information-communications-technology-producing industries<sup> 4</sup>
    InformationCommunicationsTechnologyProducingIndustries,
    /// Insurance carriers, except direct life insurance
    InsuranceCarriersExceptDirectLifeInsurance,
    /// Iron and steel mills and manufacturing from purchased steel
    IronSteelMillsManufacturingFromPurchasedSteel,
    /// Less: Other subsidies on production
    LessOtherSubsidiesOnProduction,
    /// Less: Subsidies on products
    LessSubsidiesOnProducts,
    /// Maintenance and repair construction
    MaintenanceRepairConstruction,
    /// Manufacturing
    Manufacturing,
    /// Miscellaneous professional, scientific, and technical services
    MiscellaneousProfessionalScientificTechnicalServices,
    /// Motor vehicle body, trailer, and parts manufacturing
    MotorVehicleBodyTrailerPartsManufacturing,
    /// Motor vehicles, bodies and trailers, and parts
    MotorVehiclesBodiesTrailersParts,
    /// National defense: Consumption expenditures
    NationalDefenseConsumptionExpenditures,
    /// Natural gas distribution and water, sewage and other systems
    NaturalGasDistributionWaterSewageOtherSystems,
    /// Noncomparable imports and rest-of-the-world adjustment
    NoncomparableImportsRestOfTheWorldAdjustment,
    /// Nondefense: Consumption expenditures
    NondefenseConsumptionExpenditures,
    /// Nondurable goods
    NondurableGoods,
    /// Nonferrous metal production and processing and foundries
    NonferrousMetalProductionProcessingFoundries,
    /// Nonresidential private fixed investment in equipment
    NonresidentialPrivateFixedInvestmentEquipment,
    /// Nonresidential private fixed investment in intellectual property products
    NonresidentialPrivateFixedInvestmentIntellectualPropertyRights,
    /// Nonresidential private fixed investment in structures
    NonresidentialPrivateFixedInvestmentStructures,
    /// Not allocated by industry<sup>1</sup>
    NotAllocatedByIndustry,
    /// Office and commercial structures
    OfficeCommercialStructures,
    /// Other administrative and support services
    OtherAdministrativeSupportServices,
    /// Other ambulatory health care services
    OtherAmbulatoryHealthCareServices,
    /// Other chemical manufacturing
    OtherChemicalManufacturing,
    /// Other computer and electronic product manufacturing
    OtherComputerElectronicProductManufacturing,
    /// Other durable goods merchant wholesalers
    OtherDurableGoodsMerchantWholesalers,
    /// Other machinery
    OtherMachinery,
    /// Other nondurable goods merchant wholesalers
    OtherNondurableGoodsMerchantWholesalers,
    /// Other nonresidential structures
    OtherNonresidentialStructures,
    /// Other real estate
    OtherRealEstate,
    /// Other residential construction
    OtherResidentialConstruction,
    /// Other retail
    OtherRetail,
    /// Other taxes on production
    OtherTaxesOnProduction,
    /// Other telecommunications, including satellite
    OtherTelecommunicationsIncludingSatellite,
    /// Other transportation equipment
    OtherTransportationEquipment,
    /// Other transportation and support activities
    OtherTransportationSupportActivities,
    /// Owner-occupied housing
    OwnerOccupiedHousing,
    /// Performing arts, spectator sports, museums, and related activities
    PerformingArtsSpectatorSportsMuseumsRelatedActivies,
    /// Personal consumption expenditures
    PersonalConsumptionExpenditures,
    /// Power and communication structures
    PowerCommunicationStructures,
    /// Private fixed investment
    PrivateFixedInvestment,
    /// Private goods-producing industries<sup>2</sup>
    PrivateGoodsProducingIndustries,
    /// Private industries
    PrivateIndustries,
    /// Private services-producing industries<sup>3</sup>
    PrivateServicesProducingIndustries,
    /// Professional and business services
    ProfessionalBusinessServices,
    /// Rental and leasing services and lessors of intangible assets
    RentalLeasingServicesLessorsIntangibleAssets,
    /// Residential private fixed investment
    ResidentialPrivateFixedInvestment,
    /// Retail trade
    RetailTrade,
    /// Scenic and sightseeing transportation and support activities
    ScenicSightseeingTransportationSupportActivities,
    /// Scrap, used and secondhand goods
    ScrapUsedSecondhandGoods,
    /// Single-family residential structures
    SingleFamilyResidentialStructures,
    /// Specialized design services and other professional, scientific, and technical services
    SpecializedDesignServicesOtherProfessionalScientificTechnicalServices,
    /// State and local
    StateLocal,
    /// State and local government consumption expenditures
    StateLocalGovernmentConsumptionExpenditures,
    /// State and local government educational services
    StateLocalGovernmentEducationalServices,
    /// State and local government enterprises
    StateLocalGovernmentEnterprises,
    /// State and local government hospitals and health services
    StateLocalGovernmentHospitalsHealthServices,
    /// State and local government other services
    StateLocalGovernmentOtherServices,
    /// State and local general government
    StateLocalGeneralGovernment,
    /// State and local: Gross investment in equipment
    StateLocalGrossInvestmentEquipment,
    /// State and local: Gross investment in intellectual property products
    StateLocalGrossInvestmentIntellectualPropertyProducts,
    /// State and local: Gross investment in structures
    StateLocalGrossInvestmentStructures,
    /// Subsidies on products
    SubsidiesOnProducts,
    /// Taxes on products and imports
    TaxesOnProductsImports,
    /// Tax on products
    TaxOnProducts,
    /// Tenant-occupied housing
    TenantOccupiedHousing,
    /// Textile mills and textile product mills
    TextileMillsTextileProductMills,
    /// Total Commodity Output
    TotalCommodityOutput,
    /// Total industry output (basic prices)
    TotalIndustryOutputBasicPrices,
    /// Total industry supply
    TotalIndustrySupply,
    /// Total Intermediate
    TotalIntermediate,
    /// Total product supply (basic prices)
    TotalProductSupplyBasicPrices,
    /// Total product supply (purchaser prices)
    TotalProductSupplyPurchaserPrices,
    /// Total tax less subsidies on products
    TotalTaxLessSubsidiesOnProducts,
    /// Total trade and transportation margins
    TotalTradeTransportationMargins,
    /// Total use of products
    TotalUseOfProducts,
    /// Trade margins
    TradeMargins,
    /// Transportation structures and highways and streets
    TransportationStructuresHighwaysStreets,
    /// Transportation and warehousing
    TransportationWarehousing,
    /// Transport margins
    TransportMargins,
    /// Value Added (basic prices)
    ValueAddedBasicPrices,
    /// Value Added (producer prices)
    ValueAddedProducerPrices,
}

impl NaicsInputOutput {
    pub fn description(&self) -> &'static str {
        match self {
            Self::AllIndustries => "All industries",
            Self::AllOtherRetail => "All other retail",
            Self::AllOtherTransportationEquipmentManufacturing => {
                "All other transportation equipment manufacturing"
            }
            Self::ApparelLeatherAlliedProducts => "Apparel and leather and allied products",
            Self::ChangeInPrivateInventories => "Change in private inventories",
            Self::CifFobAdjustmentsOnImports => "CIF/FOB Adjustments on Imports",
            Self::CompensationOfEmployees => "Compensation of employees",
            Self::CustomsDuties => "Customs duties",
            Self::DurableGoods => "Durable goods",
            Self::EducationHospitalHealthStructures => "Education, hospital, and health structures",
            Self::ExportsGoodsServices => "Exports of goods and services",
            Self::Farms => "Farms",
            Self::Federal => "Federal",
            Self::FederalGeneralGovernmentDefense => "Federal general government (defense)",
            Self::FederalGeneralGovernmentNondefense => "Federal general government (nondefense)",
            Self::FederalGovernmentEnterprises => "Federal government enterprises",
            Self::FederalNationalDefenseGrossInvestmentEquipment => {
                "Federal national defense: Gross investment in equipment"
            }
            Self::FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts => {
                "Federal national defense: Gross investment in intellectual property products"
            }
            Self::FederalNationalDefenseGrossInvestmentStructures => {
                "Federal national defense: Gross investment in structures"
            }
            Self::FederalNondefenseGrossInvestmentEquipment => {
                "Federal nondefense: Gross investment in equipment"
            }
            Self::FederalNondefenseGrossInvestmentIntellectualPropertyProducts => {
                "Federal nondefense: Gross investment in intellectual property products"
            }
            Self::FederalNondefenseGrossInvestmentStructures => {
                "Federal nondefense: Gross investment in structures"
            }
            Self::FederalReserveBanksCreditIntermediationRelatedActivities => {
                "Federal Reserve banks, credit intermediation, and related activities"
            }
            Self::FinanceInsuranceRealEstateRentalLeasing => {
                "Finance, insurance, real estate, rental, and leasing"
            }
            Self::FoodBeverageTobaccoProducts => "Food and beverage and tobacco products",
            Self::ForestryFishingRelatedActivities => "Forestry, fishing, and related activities",
            Self::GeneralGovernment => "General government",
            Self::Government => "Government",
            Self::GovernmentConsumptionExpendituresGrossInvestment => {
                "Government consumption expenditures and gross investment"
            }
            Self::GrossDomesticProduct => "Gross domestic product",
            Self::GrossOperatingSurplus => "Gross operating surplus",
            Self::HospitalsNursingResidentialCareFacilities => {
                "Hospitals and nursing and residential care facilities (A,Q)"
            }
            Self::Housing => "Housing",
            Self::ImportDuties => "Import duties",
            Self::Imports => "Imports",
            Self::InformationCommunicationsTechnologyProducingIndustries => {
                "Information-communications-technology-producing industries<sup> 4</sup>"
            }
            Self::InsuranceCarriersExceptDirectLifeInsurance => {
                "Insurance carriers, except direct life insurance"
            }
            Self::IronSteelMillsManufacturingFromPurchasedSteel => {
                "Iron and steel mills and manufacturing from purchased steel"
            }
            Self::LessOtherSubsidiesOnProduction => "Less: Other subsidies on production",
            Self::LessSubsidiesOnProducts => "Less: Subsidies on products",
            Self::MaintenanceRepairConstruction => "Maintenance and repair construction",
            Self::Manufacturing => "Manufacturing",
            Self::MiscellaneousProfessionalScientificTechnicalServices => {
                "Miscellaneous professional, scientific, and technical services"
            }
            Self::MotorVehicleBodyTrailerPartsManufacturing => {
                "Motor vehicle body, trailer, and parts manufacturing"
            }
            Self::MotorVehiclesBodiesTrailersParts => {
                "Motor vehicles, bodies and trailers, and parts"
            }
            Self::NationalDefenseConsumptionExpenditures => {
                "National defense: Consumption expenditures"
            }
            Self::NaturalGasDistributionWaterSewageOtherSystems => {
                "Natural gas distribution and water, sewage and other systems"
            }
            Self::NoncomparableImportsRestOfTheWorldAdjustment => {
                "Noncomparable imports and rest-of-the-world adjustment"
            }
            Self::NondefenseConsumptionExpenditures => "Nondefense: Consumption expenditures",
            Self::NondurableGoods => "Nondurable goods",
            Self::NonferrousMetalProductionProcessingFoundries => {
                "Nonferrous metal production and processing and foundries"
            }
            Self::NonresidentialPrivateFixedInvestmentEquipment => {
                "Nonresidential private fixed investment in equipment"
            }
            Self::NonresidentialPrivateFixedInvestmentIntellectualPropertyRights => {
                "Nonresidential private fixed investment in intellectual property products"
            }
            Self::NonresidentialPrivateFixedInvestmentStructures => {
                "Nonresidential private fixed investment in structures"
            }
            Self::NotAllocatedByIndustry => "Not allocated by industry<sup>1</sup>",
            Self::OfficeCommercialStructures => "Office and commercial structures",
            Self::OtherAdministrativeSupportServices => "Other administrative and support services",
            Self::OtherAmbulatoryHealthCareServices => "Other ambulatory health care services",
            Self::OtherChemicalManufacturing => "Other chemical manufacturing",
            Self::OtherComputerElectronicProductManufacturing => {
                "Other computer and electronic product manufacturing"
            }
            Self::OtherDurableGoodsMerchantWholesalers => {
                "Other durable goods merchant wholesalers"
            }
            Self::OtherMachinery => "Other machinery",
            Self::OtherNondurableGoodsMerchantWholesalers => {
                "Other nondurable goods merchant wholesalers"
            }
            Self::OtherNonresidentialStructures => "Other nonresidential structures",
            Self::OtherRealEstate => "Other real estate",
            Self::OtherResidentialConstruction => "Other residential construction",
            Self::OtherRetail => "Other retail",
            Self::OtherTaxesOnProduction => "Other taxes on production",
            Self::OtherTelecommunicationsIncludingSatellite => {
                "Other telecommunications, including satellite"
            }
            Self::OtherTransportationEquipment => "Other transportation equipment",
            Self::OtherTransportationSupportActivities => {
                "Other transportation and support activities"
            }
            Self::OwnerOccupiedHousing => "Owner-occupied housing",
            Self::PerformingArtsSpectatorSportsMuseumsRelatedActivies => {
                "Performing arts, spectator sports, museums, and related activities"
            }
            Self::PersonalConsumptionExpenditures => "Personal consumption expenditures",
            Self::PowerCommunicationStructures => "Power and communication structures",
            Self::PrivateFixedInvestment => "Private fixed investment",
            Self::PrivateGoodsProducingIndustries => {
                "Private goods-producing industries<sup>2</sup>"
            }
            Self::PrivateIndustries => "Private industries",
            Self::PrivateServicesProducingIndustries => {
                "Private services-producing industries<sup>3</sup>"
            }
            Self::ProfessionalBusinessServices => "Professional and business services",
            Self::RentalLeasingServicesLessorsIntangibleAssets => {
                "Rental and leasing services and lessors of intangible assets"
            }
            Self::ResidentialPrivateFixedInvestment => "Residential private fixed investment",
            Self::RetailTrade => "Retail trade",
            Self::ScenicSightseeingTransportationSupportActivities => {
                "Scenic and sightseeing transportation and support activities"
            }
            Self::ScrapUsedSecondhandGoods => "Scrap, used and secondhand goods",
            Self::SingleFamilyResidentialStructures => "Single-family residential structures",
            Self::SpecializedDesignServicesOtherProfessionalScientificTechnicalServices => {
                "Specialized design services and other professional, scientific, and technical services"
            }
            Self::StateLocal => "State and local",
            Self::StateLocalGovernmentConsumptionExpenditures => {
                "State and local government consumption expenditures"
            }
            Self::StateLocalGovernmentEducationalServices => {
                "State and local government educational services"
            }
            Self::StateLocalGovernmentEnterprises => "State and local government enterprises",
            Self::StateLocalGovernmentHospitalsHealthServices => {
                "State and local government hospitals and health services"
            }
            Self::StateLocalGovernmentOtherServices => "State and local government other services",
            Self::StateLocalGeneralGovernment => "State and local general government",
            Self::StateLocalGrossInvestmentEquipment => {
                "State and local: Gross investment in equipment"
            }
            Self::StateLocalGrossInvestmentIntellectualPropertyProducts => {
                "State and local: Gross investment in intellectual property products"
            }
            Self::StateLocalGrossInvestmentStructures => {
                "State and local: Gross investment in structures"
            }
            Self::SubsidiesOnProducts => "Subsidies on products",
            Self::TaxesOnProductsImports => "Taxes on products and imports",
            Self::TaxOnProducts => "Tax on products",
            Self::TenantOccupiedHousing => "Tenant-occupied housing",
            Self::TextileMillsTextileProductMills => "Textile mills and textile product mills",
            Self::TotalCommodityOutput => "Total Commodity Output",
            Self::TotalIndustryOutputBasicPrices => "Total industry output (basic prices)",
            Self::TotalIndustrySupply => "Total industry supply",
            Self::TotalIntermediate => "Total Intermediate",
            Self::TotalProductSupplyBasicPrices => "Total product supply (basic prices)",
            Self::TotalProductSupplyPurchaserPrices => "Total product supply (purchaser prices)",
            Self::TotalTaxLessSubsidiesOnProducts => "Total tax less subsidies on products",
            Self::TotalTradeTransportationMargins => "Total trade and transportation margins",
            Self::TotalUseOfProducts => "Total use of products",
            Self::TradeMargins => "Trade margins",
            Self::TransportationStructuresHighwaysStreets => {
                "Transportation structures and highways and streets"
            }
            Self::TransportationWarehousing => "Transportation and warehousing",
            Self::TransportMargins => "Transport margins",
            Self::ValueAddedBasicPrices => "Value Added (basic prices)",
            Self::ValueAddedProducerPrices => "Value Added (producer prices)",
            // Duplicated by NABI
            // Self::NotAllocatedByIndustry1 => "Not allocated by industry<sup>1</sup>",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::AllIndustries => "II",
            Self::AllOtherRetail => "4A0X",
            Self::AllOtherTransportationEquipmentManufacturing => "3365AO",
            Self::ApparelLeatherAlliedProducts => "315AL",
            Self::ChangeInPrivateInventories => "F030",
            Self::CifFobAdjustmentsOnImports => "MADJ",
            Self::CompensationOfEmployees => "V001",
            Self::CustomsDuties => "42ID",
            Self::DurableGoods => "33DG",
            Self::EducationHospitalHealthStructures => "23EH",
            Self::ExportsGoodsServices => "F040",
            Self::Farms => "111CA",
            Self::Federal => "GF",
            Self::FederalGeneralGovernmentDefense => "GFGD",
            Self::FederalGeneralGovernmentNondefense => "GFGN",
            Self::FederalGovernmentEnterprises => "GFE",
            Self::FederalNationalDefenseGrossInvestmentEquipment => "F06E",
            Self::FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts => "F06N",
            Self::FederalNationalDefenseGrossInvestmentStructures => "F06S",
            Self::FederalNondefenseGrossInvestmentEquipment => "F07E",
            Self::FederalNondefenseGrossInvestmentIntellectualPropertyProducts => "F07N",
            Self::FederalNondefenseGrossInvestmentStructures => "F07S",
            Self::FederalReserveBanksCreditIntermediationRelatedActivities => "521CI",
            Self::FinanceInsuranceRealEstateRentalLeasing => "FIRE",
            Self::FoodBeverageTobaccoProducts => "311FT",
            Self::ForestryFishingRelatedActivities => "113FF",
            Self::GeneralGovernment => "GFG",
            Self::Government => "G",
            Self::GovernmentConsumptionExpendituresGrossInvestment => "F100",
            Self::GrossDomesticProduct => "GDP",
            Self::GrossOperatingSurplus => "V003",
            Self::HospitalsNursingResidentialCareFacilities => "622HO",
            Self::Housing => "HS",
            Self::ImportDuties => "MDTY",
            Self::Imports => "MCIF",
            Self::InformationCommunicationsTechnologyProducingIndustries => "ICT",
            Self::InsuranceCarriersExceptDirectLifeInsurance => "5241XX",
            Self::IronSteelMillsManufacturingFromPurchasedSteel => "3311IS",
            Self::LessOtherSubsidiesOnProduction => "T00OSUB",
            Self::LessSubsidiesOnProducts => "T00SUB",
            Self::MaintenanceRepairConstruction => "23MR",
            Self::Manufacturing => "31G",
            Self::MiscellaneousProfessionalScientificTechnicalServices => "5412OP",
            Self::MotorVehicleBodyTrailerPartsManufacturing => "3362BP",
            Self::MotorVehiclesBodiesTrailersParts => "3361MV",
            Self::NationalDefenseConsumptionExpenditures => "F06C",
            Self::NaturalGasDistributionWaterSewageOtherSystems => "2212NW",
            Self::NoncomparableImportsRestOfTheWorldAdjustment => "Other",
            Self::NondefenseConsumptionExpenditures => "F07C",
            Self::NondurableGoods => "31ND",
            Self::NonferrousMetalProductionProcessingFoundries => "3313NF",
            Self::NonresidentialPrivateFixedInvestmentEquipment => "F02E",
            Self::NonresidentialPrivateFixedInvestmentIntellectualPropertyRights => "F02N",
            Self::NonresidentialPrivateFixedInvestmentStructures => "F02S",
            Self::NotAllocatedByIndustry => "NABI",
            Self::OfficeCommercialStructures => "23OC",
            Self::OtherAdministrativeSupportServices => "561X",
            Self::OtherAmbulatoryHealthCareServices => "6215OH",
            Self::OtherChemicalManufacturing => "325X",
            Self::OtherComputerElectronicProductManufacturing => "334X",
            Self::OtherDurableGoodsMerchantWholesalers => "423X",
            Self::OtherMachinery => "3332OM",
            Self::OtherNondurableGoodsMerchantWholesalers => "424X",
            Self::OtherNonresidentialStructures => "23OT",
            Self::OtherRealEstate => "ORE",
            Self::OtherResidentialConstruction => "23OR",
            Self::OtherRetail => "4A0",
            Self::OtherTaxesOnProduction => "T00OTOP",
            Self::OtherTelecommunicationsIncludingSatellite => "5174OT",
            Self::OtherTransportationEquipment => "3364OT",
            Self::OtherTransportationSupportActivities => "487OS",
            Self::OwnerOccupiedHousing => "HSO",
            Self::PerformingArtsSpectatorSportsMuseumsRelatedActivies => "711AS",
            Self::PersonalConsumptionExpenditures => "F010",
            Self::PowerCommunicationStructures => "23PC",
            Self::PrivateFixedInvestment => "F020",
            Self::PrivateGoodsProducingIndustries => "PGOOD",
            Self::PrivateIndustries => "PVT",
            Self::PrivateServicesProducingIndustries => "PSERV",
            Self::ProfessionalBusinessServices => "PROF",
            Self::RentalLeasingServicesLessorsIntangibleAssets => "532RL",
            Self::ResidentialPrivateFixedInvestment => "F02R",
            Self::RetailTrade => "44RT",
            Self::ScenicSightseeingTransportationSupportActivities => "48A",
            Self::ScrapUsedSecondhandGoods => "Used",
            Self::SingleFamilyResidentialStructures => "23SF",
            Self::SpecializedDesignServicesOtherProfessionalScientificTechnicalServices => "541X",
            Self::StateLocal => "GSL",
            Self::StateLocalGovernmentConsumptionExpenditures => "F10C",
            Self::StateLocalGovernmentEducationalServices => "GSLGE",
            Self::StateLocalGovernmentEnterprises => "GSLE",
            Self::StateLocalGovernmentHospitalsHealthServices => "GSLGH",
            Self::StateLocalGovernmentOtherServices => "GSLGO",
            Self::StateLocalGeneralGovernment => "GSLG",
            Self::StateLocalGrossInvestmentEquipment => "F10E",
            Self::StateLocalGrossInvestmentIntellectualPropertyProducts => "F10N",
            Self::StateLocalGrossInvestmentStructures => "F10S",
            Self::SubsidiesOnProducts => "SUB",
            Self::TaxesOnProductsImports => "T00TOP",
            Self::TaxOnProducts => "TOP",
            Self::TenantOccupiedHousing => "HST",
            Self::TransportationStructuresHighwaysStreets => "23TH",
            Self::TotalCommodityOutput => "T007",
            Self::TotalIndustryOutputBasicPrices => "T018",
            Self::TextileMillsTextileProductMills => "313TT",
            Self::TotalIndustrySupply => "T017",
            Self::TotalIntermediate => "T005",
            Self::TotalProductSupplyBasicPrices => "T013",
            Self::TotalProductSupplyPurchaserPrices => "T016",
            Self::TotalTaxLessSubsidiesOnProducts => "T015",
            Self::TotalTradeTransportationMargins => "T014",
            Self::TotalUseOfProducts => "T019",
            Self::TradeMargins => "Trade",
            Self::TransportationWarehousing => "48TW",
            Self::TransportMargins => "Trans",
            Self::ValueAddedBasicPrices => "VABAS",
            Self::ValueAddedProducerPrices => "VAPRO",
            // Duplicated by NABI
            // Self::NotAllocatedByIndustry1 => "NABIU",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let naics = match code {
            "111CA" => Self::Farms,
            "113FF" => Self::ForestryFishingRelatedActivities,
            "2212NW" => Self::NaturalGasDistributionWaterSewageOtherSystems,
            "23EH" => Self::EducationHospitalHealthStructures,
            "23MR" => Self::MaintenanceRepairConstruction,
            "23OC" => Self::OfficeCommercialStructures,
            "23OR" => Self::OtherResidentialConstruction,
            "23OT" => Self::OtherNonresidentialStructures,
            "23PC" => Self::PowerCommunicationStructures,
            "23SF" => Self::SingleFamilyResidentialStructures,
            "23TH" => Self::TransportationStructuresHighwaysStreets,
            "311FT" => Self::FoodBeverageTobaccoProducts,
            "313TT" => Self::TextileMillsTextileProductMills,
            "315AL" => Self::ApparelLeatherAlliedProducts,
            "31G" => Self::Manufacturing,
            "31ND" => Self::NondurableGoods,
            "325X" => Self::OtherChemicalManufacturing,
            "3311IS" => Self::IronSteelMillsManufacturingFromPurchasedSteel,
            "3313NF" => Self::NonferrousMetalProductionProcessingFoundries,
            "3332OM" => Self::OtherMachinery,
            "334X" => Self::OtherComputerElectronicProductManufacturing,
            "3361MV" => Self::MotorVehiclesBodiesTrailersParts,
            "3362BP" => Self::MotorVehicleBodyTrailerPartsManufacturing,
            "3364OT" => Self::OtherTransportationEquipment,
            "3365AO" => Self::AllOtherTransportationEquipmentManufacturing,
            "33DG" => Self::DurableGoods,
            "423X" => Self::OtherDurableGoodsMerchantWholesalers,
            "424X" => Self::OtherNondurableGoodsMerchantWholesalers,
            "42ID" => Self::CustomsDuties,
            "44RT" => Self::RetailTrade,
            "487OS" => Self::OtherTransportationSupportActivities,
            "48A" => Self::ScenicSightseeingTransportationSupportActivities,
            "48TW" => Self::TransportationWarehousing,
            "4A0" => Self::OtherRetail,
            "4A0X" => Self::AllOtherRetail,
            "5174OT" => Self::OtherTelecommunicationsIncludingSatellite,
            "521CI" => Self::FederalReserveBanksCreditIntermediationRelatedActivities,
            "5241XX" => Self::InsuranceCarriersExceptDirectLifeInsurance,
            "532RL" => Self::RentalLeasingServicesLessorsIntangibleAssets,
            "5412OP" => Self::MiscellaneousProfessionalScientificTechnicalServices,
            "541X" => Self::SpecializedDesignServicesOtherProfessionalScientificTechnicalServices,
            "561X" => Self::OtherAdministrativeSupportServices,
            "6215OH" => Self::OtherAmbulatoryHealthCareServices,
            "622HO" => Self::HospitalsNursingResidentialCareFacilities,
            "711AS" => Self::PerformingArtsSpectatorSportsMuseumsRelatedActivies,
            "F010" => Self::PersonalConsumptionExpenditures,
            "F020" => Self::PrivateFixedInvestment,
            "F02E" => Self::NonresidentialPrivateFixedInvestmentEquipment,
            "F02N" => Self::NonresidentialPrivateFixedInvestmentIntellectualPropertyRights,
            "F02R" => Self::ResidentialPrivateFixedInvestment,
            "F02S" => Self::NonresidentialPrivateFixedInvestmentStructures,
            "F030" => Self::ChangeInPrivateInventories,
            "F040" => Self::ExportsGoodsServices,
            "F06C" => Self::NationalDefenseConsumptionExpenditures,
            "F06E" => Self::FederalNationalDefenseGrossInvestmentEquipment,
            "F06N" => Self::FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts,
            "F06S" => Self::FederalNationalDefenseGrossInvestmentStructures,
            "F07C" => Self::NondefenseConsumptionExpenditures,
            "F07E" => Self::FederalNondefenseGrossInvestmentEquipment,
            "F07N" => Self::FederalNondefenseGrossInvestmentIntellectualPropertyProducts,
            "F07S" => Self::FederalNondefenseGrossInvestmentStructures,
            "F100" => Self::GovernmentConsumptionExpendituresGrossInvestment,
            "F10C" => Self::StateLocalGovernmentConsumptionExpenditures,
            "F10E" => Self::StateLocalGrossInvestmentEquipment,
            "F10N" => Self::StateLocalGrossInvestmentIntellectualPropertyProducts,
            "F10S" => Self::StateLocalGrossInvestmentStructures,
            "FIRE" => Self::FinanceInsuranceRealEstateRentalLeasing,
            "G" => Self::Government,
            "GDP" => Self::GrossDomesticProduct,
            "GF" => Self::Federal,
            "GFE" => Self::FederalGovernmentEnterprises,
            "GFG" => Self::GeneralGovernment,
            "GFGD" => Self::FederalGeneralGovernmentDefense,
            "GFGN" => Self::FederalGeneralGovernmentNondefense,
            "GSL" => Self::StateLocal,
            "GSLE" => Self::StateLocalGovernmentEnterprises,
            "GSLG" => Self::StateLocalGeneralGovernment,
            "GSLGE" => Self::StateLocalGovernmentEducationalServices,
            "GSLGH" => Self::StateLocalGovernmentHospitalsHealthServices,
            "GSLGO" => Self::StateLocalGovernmentOtherServices,
            "HS" => Self::Housing,
            "HSO" => Self::OwnerOccupiedHousing,
            "HST" => Self::TenantOccupiedHousing,
            "ICT" => Self::InformationCommunicationsTechnologyProducingIndustries,
            "II" => Self::AllIndustries,
            "MADJ" => Self::CifFobAdjustmentsOnImports,
            "MCIF" => Self::Imports,
            "MDTY" => Self::ImportDuties,
            "NABI" => Self::NotAllocatedByIndustry,
            "NABIU" => Self::NotAllocatedByIndustry,
            "ORE" => Self::OtherRealEstate,
            "Other" => Self::NoncomparableImportsRestOfTheWorldAdjustment,
            "PGOOD" => Self::PrivateGoodsProducingIndustries,
            "PROF" => Self::ProfessionalBusinessServices,
            "PSERV" => Self::PrivateServicesProducingIndustries,
            "PVT" => Self::PrivateIndustries,
            "SUB" => Self::SubsidiesOnProducts,
            "T00OSUB" => Self::LessOtherSubsidiesOnProduction,
            "T00OTOP" => Self::OtherTaxesOnProduction,
            "T00SUB" => Self::LessSubsidiesOnProducts,
            "T00TOP" => Self::TaxesOnProductsImports,
            "T001" => Self::TotalIntermediate,
            "T005" => Self::TotalIntermediate,
            "T007" => Self::TotalCommodityOutput,
            "T013" => Self::TotalProductSupplyBasicPrices,
            "T014" => Self::TotalTradeTransportationMargins,
            "T015" => Self::TotalTaxLessSubsidiesOnProducts,
            "T016" => Self::TotalProductSupplyPurchaserPrices,
            "T017" => Self::TotalIndustrySupply,
            "T018" => Self::TotalIndustryOutputBasicPrices,
            "T019" => Self::TotalUseOfProducts,
            "TOP" => Self::TaxOnProducts,
            "Trade" => Self::TradeMargins,
            "Trans" => Self::TransportMargins,
            "Used" => Self::ScrapUsedSecondhandGoods,
            "V001" => Self::CompensationOfEmployees,
            "V003" => Self::GrossOperatingSurplus,
            "VABAS" => Self::ValueAddedBasicPrices,
            "VAPRO" => Self::ValueAddedProducerPrices,
            "" => Self::TotalIndustrySupply,
            _ => return None,
        };
        Some(naics)
    }
}
