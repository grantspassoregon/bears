/// NAICS Subsector definitions based on North American Industry Classification System
///
/// This enum represents subsector codes and descriptions from the NAICS classification system.
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
pub enum NaicsSubsector {
    /// Crop Production
    CropProduction,
    /// Animal Production and Aquaculture
    AnimalProductionAndAquaculture,
    /// Forestry and Logging
    ForestryAndLogging,
    /// Fishing, Hunting and Trapping
    FishingHuntingAndTrapping,
    /// Support Activities for Agriculture and Forestry
    SupportActivitiesForAgricultureAndForestry,
    /// Oil and Gas Extraction
    OilAndGasExtraction,
    /// Mining (except Oil and Gas)
    MiningExceptOilAndGas,
    /// Support Activities for Mining
    SupportActivitiesForMining,
    /// Utilities
    Utilities,
    /// Construction of Buildings
    ConstructionOfBuildings,
    /// Heavy and Civil Engineering Construction
    HeavyAndCivilEngineeringConstruction,
    /// Specialty Trade Contractors
    SpecialtyTradeContractors,
    /// Food Manufacturing
    FoodManufacturing,
    /// Beverage and Tobacco Product Manufacturing
    BeverageAndTobaccoProductManufacturing,
    /// Textile Mills
    TextileMills,
    /// Textile Product Mills
    TextileProductMills,
    /// Apparel Manufacturing
    ApparelManufacturing,
    /// Leather and Allied Product Manufacturing
    LeatherAndAlliedProductManufacturing,
    /// Wood Product Manufacturing
    WoodProductManufacturing,
    /// Paper Manufacturing
    PaperManufacturing,
    /// Printing and Related Support Activities
    PrintingAndRelatedSupportActivities,
    /// Petroleum and Coal Products Manufacturing
    PetroleumAndCoalProductsManufacturing,
    /// Chemical Manufacturing
    ChemicalManufacturing,
    /// Plastics and Rubber Products Manufacturing
    PlasticsAndRubberProductsManufacturing,
    /// Nonmetallic Mineral Product Manufacturing
    NonmetallicMineralProductManufacturing,
    /// Primary Metal Manufacturing
    PrimaryMetalManufacturing,
    /// Fabricated Metal Product Manufacturing
    FabricatedMetalProductManufacturing,
    /// Machinery Manufacturing
    MachineryManufacturing,
    /// Computer and Electronic Product Manufacturing
    ComputerAndElectronicProductManufacturing,
    /// Electrical Equipment, Appliance, and Component Manufacturing
    ElectricalEquipmentApplianceAndComponentManufacturing,
    /// Transportation Equipment Manufacturing
    TransportationEquipmentManufacturing,
    /// Furniture and Related Product Manufacturing
    FurnitureAndRelatedProductManufacturing,
    /// Miscellaneous Manufacturing
    MiscellaneousManufacturing,
    /// Merchant Wholesalers, Durable Goods
    MerchantWholesalersDurableGoods,
    /// Merchant Wholesalers, Nondurable Goods
    MerchantWholesalersNondurableGoods,
    /// Wholesale Trade Agents and Brokers
    WholesaleTradeAgentsAndBrokers,
    /// Motor Vehicle and Parts Dealers
    MotorVehicleAndPartsDealers,
    /// Furniture and Home Furnishings Stores
    FurnitureAndHomeFurnishingsStores,
    /// Electronics and Appliance Stores
    ElectronicsAndApplianceStores,
    /// Building Material and Garden Equipment and Supplies Dealers
    BuildingMaterialAndGardenEquipmentAndSuppliesDealers,
    /// Food and Beverage Retailers
    FoodAndBeverageRetailers,
    /// Furniture, Home Furnishings, Electronics, and Appliance Retailers
    FurnitureHomeFurnishingsElectronicsAndApplianceRetailers,
    /// General Merchandise Retailers
    GeneralMerchandiseRetailers,
    /// Health and Personal Care Retailers
    HealthAndPersonalCareRetailers,
    /// Gasoline Stations and Fuel Dealers
    GasolineStationsAndFuelDealers,
    /// Clothing, Clothing Accessories, Shoe, and Jewelry Retailers
    ClothingClothingAccessoriesShoeAndJewelryRetailers,
    /// Sporting Goods, Hobby, Musical Instrument, Book, and Miscellaneous Retailers
    SportingGoodsHobbyMusicalInstrumentBookAndMiscellaneousRetailers,
    /// Air Transportation
    AirTransportation,
    /// Rail Transportation
    RailTransportation,
    /// Water Transportation
    WaterTransportation,
    /// Truck Transportation
    TruckTransportation,
    /// Transit and Ground Passenger Transportation
    TransitAndGroundPassengerTransportation,
    /// Pipeline Transportation
    PipelineTransportation,
    /// Scenic and Sightseeing Transportation
    ScenicAndSightseeingTransportation,
    /// Support Activities for Transportation
    SupportActivitiesForTransportation,
    /// Postal Service
    PostalService,
    /// Couriers and Messengers
    CouriersAndMessengers,
    /// Warehousing and Storage
    WarehousingAndStorage,
    /// Publishing Industries (except Internet)
    PublishingIndustriesExceptInternet,
    /// Motion Picture and Sound Recording Industries
    MotionPictureAndSoundRecordingIndustries,
    /// Publishing Industries
    PublishingIndustries,
    /// Broadcasting (except Internet)
    BroadcastingExceptInternet,
    /// Broadcasting and Content Providers
    BroadcastingAndContentProviders,
    /// Telecommunications
    Telecommunications,
    /// Computing Infrastructure Providers, Data Processing, Web Hosting, and Related Services
    ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices,
    /// Other Information Services
    OtherInformationServices,
    /// Monetary Authorities-Central Bank
    MonetaryAuthoritiesCentralBank,
    /// Credit Intermediation and Related Activities
    CreditIntermediationAndRelatedActivities,
    /// Securities, Commodity Contracts, and Other Financial Investments and Related Activities
    SecuritiesCommodityContractsAndOtherFinancialInvestmentsAndRelatedActivities,
    /// Insurance Carriers and Related Activities
    InsuranceCarriersAndRelatedActivities,
    /// Funds, Trusts, and Other Financial Vehicles
    FundsTrustsAndOtherFinancialVehicles,
    /// Real Estate
    RealEstate,
    /// Rental and Leasing Services
    RentalAndLeasingServices,
    /// Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)
    LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks,
    /// Professional, Scientific, and Technical Services
    ProfessionalScientificAndTechnicalServices,
    /// Management of Companies and Enterprises
    ManagementOfCompaniesAndEnterprises,
    /// Administrative and Support Services
    AdministrativeAndSupportServices,
    /// Waste Management and Remediation Services
    WasteManagementAndRemediationServices,
    /// Educational Services
    EducationalServices,
    /// Ambulatory Health Care Services
    AmbulatoryHealthCareServices,
    /// Hospitals
    Hospitals,
    /// Nursing and Residential Care Facilities
    NursingAndResidentialCareFacilities,
    /// Social Assistance
    SocialAssistance,
    /// Performing Arts, Spectator Sports, and Related Industries
    PerformingArtsSpectatorSportsAndRelatedIndustries,
    /// Museums, Historical Sites, and Similar Institutions
    MuseumsHistoricalSitesAndSimilarInstitutions,
    /// Amusement, Gambling, and Recreation Industries
    AmusementGamblingAndRecreationIndustries,
    /// Accommodation
    Accommodation,
    /// Food Services and Drinking Places
    FoodServicesAndDrinkingPlaces,
    /// Repair and Maintenance
    RepairAndMaintenance,
    /// Personal and Laundry Services
    PersonalAndLaundryServices,
    /// Religious, Grantmaking, Civic, Professional, and Similar Organizations
    ReligiousGrantmakingCivicProfessionalAndSimilarOrganizations,
    /// Private Households
    PrivateHouseholds,
    /// Executive, Legislative, and Other General Government Support
    ExecutiveLegislativeAndOtherGeneralGovernmentSupport,
    /// Justice, Public Order, and Safety Activities
    JusticePublicOrderAndSafetyActivities,
    /// Administration of Human Resource Programs
    AdministrationOfHumanResourcePrograms,
    /// Administration of Environmental Quality Programs
    AdministrationOfEnvironmentalQualityPrograms,
    /// Administration of Housing Programs, Urban Planning, and Community Development
    AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment,
    /// Administration of Economic Programs
    AdministrationOfEconomicPrograms,
    /// Space Research and Technology
    SpaceResearchAndTechnology,
    /// National Security and International Affairs
    NationalSecurityAndInternationalAffairs,
    /// Unclassified Establishments
    UnclassifiedEstablishments,
}

impl NaicsSubsector {
    /// Returns the descriptive title of the NAICS subsector
    ///
    /// # Returns
    /// A static string containing the title/description of the subsector
    pub fn description(&self) -> &'static str {
        match self {
            Self::CropProduction => "Crop Production",
            Self::AnimalProductionAndAquaculture => "Animal Production and Aquaculture",
            Self::ForestryAndLogging => "Forestry and Logging",
            Self::FishingHuntingAndTrapping => "Fishing, Hunting and Trapping",
            Self::SupportActivitiesForAgricultureAndForestry => {
                "Support Activities for Agriculture and Forestry"
            }
            Self::OilAndGasExtraction => "Oil and Gas Extraction",
            Self::MiningExceptOilAndGas => "Mining (except Oil and Gas)",
            Self::SupportActivitiesForMining => "Support Activities for Mining",
            Self::Utilities => "Utilities",
            Self::ConstructionOfBuildings => "Construction of Buildings",
            Self::HeavyAndCivilEngineeringConstruction => {
                "Heavy and Civil Engineering Construction"
            }
            Self::SpecialtyTradeContractors => "Specialty Trade Contractors",
            Self::FoodManufacturing => "Food Manufacturing",
            Self::BeverageAndTobaccoProductManufacturing => {
                "Beverage and Tobacco Product Manufacturing"
            }
            Self::TextileMills => "Textile Mills",
            Self::TextileProductMills => "Textile Product Mills",
            Self::ApparelManufacturing => "Apparel Manufacturing",
            Self::LeatherAndAlliedProductManufacturing => {
                "Leather and Allied Product Manufacturing"
            }
            Self::WoodProductManufacturing => "Wood Product Manufacturing",
            Self::PaperManufacturing => "Paper Manufacturing",
            Self::PrintingAndRelatedSupportActivities => "Printing and Related Support Activities",
            Self::PetroleumAndCoalProductsManufacturing => {
                "Petroleum and Coal Products Manufacturing"
            }
            Self::ChemicalManufacturing => "Chemical Manufacturing",
            Self::PlasticsAndRubberProductsManufacturing => {
                "Plastics and Rubber Products Manufacturing"
            }
            Self::NonmetallicMineralProductManufacturing => {
                "Nonmetallic Mineral Product Manufacturing"
            }
            Self::PrimaryMetalManufacturing => "Primary Metal Manufacturing",
            Self::FabricatedMetalProductManufacturing => "Fabricated Metal Product Manufacturing",
            Self::MachineryManufacturing => "Machinery Manufacturing",
            Self::ComputerAndElectronicProductManufacturing => {
                "Computer and Electronic Product Manufacturing"
            }
            Self::ElectricalEquipmentApplianceAndComponentManufacturing => {
                "Electrical Equipment, Appliance, and Component Manufacturing"
            }
            Self::TransportationEquipmentManufacturing => "Transportation Equipment Manufacturing",
            Self::FurnitureAndRelatedProductManufacturing => {
                "Furniture and Related Product Manufacturing"
            }
            Self::MiscellaneousManufacturing => "Miscellaneous Manufacturing",
            Self::MerchantWholesalersDurableGoods => "Merchant Wholesalers, Durable Goods",
            Self::MerchantWholesalersNondurableGoods => "Merchant Wholesalers, Nondurable Goods",
            Self::WholesaleTradeAgentsAndBrokers => "Wholesale Trade Agents and Brokers",
            Self::MotorVehicleAndPartsDealers => "Motor Vehicle and Parts Dealers",
            Self::FurnitureAndHomeFurnishingsStores => "Furniture and Home Furnishings Stores",
            Self::ElectronicsAndApplianceStores => "Electronics and Appliance Stores",
            Self::BuildingMaterialAndGardenEquipmentAndSuppliesDealers => {
                "Building Material and Garden Equipment and Supplies Dealers"
            }
            Self::FoodAndBeverageRetailers => "Food and Beverage Retailers",
            Self::FurnitureHomeFurnishingsElectronicsAndApplianceRetailers => {
                "Furniture, Home Furnishings, Electronics, and Appliance Retailers"
            }
            Self::GeneralMerchandiseRetailers => "General Merchandise Retailers",
            Self::HealthAndPersonalCareRetailers => "Health and Personal Care Retailers",
            Self::GasolineStationsAndFuelDealers => "Gasoline Stations and Fuel Dealers",
            Self::ClothingClothingAccessoriesShoeAndJewelryRetailers => {
                "Clothing, Clothing Accessories, Shoe, and Jewelry Retailers"
            }
            Self::SportingGoodsHobbyMusicalInstrumentBookAndMiscellaneousRetailers => {
                "Sporting Goods, Hobby, Musical Instrument, Book, and Miscellaneous Retailers"
            }
            Self::AirTransportation => "Air Transportation",
            Self::RailTransportation => "Rail Transportation",
            Self::WaterTransportation => "Water Transportation",
            Self::TruckTransportation => "Truck Transportation",
            Self::TransitAndGroundPassengerTransportation => {
                "Transit and Ground Passenger Transportation"
            }
            Self::PipelineTransportation => "Pipeline Transportation",
            Self::ScenicAndSightseeingTransportation => "Scenic and Sightseeing Transportation",
            Self::SupportActivitiesForTransportation => "Support Activities for Transportation",
            Self::PostalService => "Postal Service",
            Self::CouriersAndMessengers => "Couriers and Messengers",
            Self::WarehousingAndStorage => "Warehousing and Storage",
            Self::PublishingIndustriesExceptInternet => "Publishing Industries (except Internet)",
            Self::MotionPictureAndSoundRecordingIndustries => {
                "Motion Picture and Sound Recording Industries"
            }
            Self::PublishingIndustries => "Publishing Industries",
            Self::BroadcastingExceptInternet => "Broadcasting (except Internet)",
            Self::BroadcastingAndContentProviders => "Broadcasting and Content Providers",
            Self::Telecommunications => "Telecommunications",
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => {
                "Computing Infrastructure Providers, Data Processing, Web Hosting, and Related Services"
            }
            Self::OtherInformationServices => "Other Information Services",
            Self::MonetaryAuthoritiesCentralBank => "Monetary Authorities-Central Bank",
            Self::CreditIntermediationAndRelatedActivities => {
                "Credit Intermediation and Related Activities"
            }
            Self::SecuritiesCommodityContractsAndOtherFinancialInvestmentsAndRelatedActivities => {
                "Securities, Commodity Contracts, and Other Financial Investments and Related Activities"
            }
            Self::InsuranceCarriersAndRelatedActivities => {
                "Insurance Carriers and Related Activities"
            }
            Self::FundsTrustsAndOtherFinancialVehicles => {
                "Funds, Trusts, and Other Financial Vehicles"
            }
            Self::RealEstate => "Real Estate",
            Self::RentalAndLeasingServices => "Rental and Leasing Services",
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => {
                "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)"
            }
            Self::ProfessionalScientificAndTechnicalServices => {
                "Professional, Scientific, and Technical Services"
            }
            Self::ManagementOfCompaniesAndEnterprises => "Management of Companies and Enterprises",
            Self::AdministrativeAndSupportServices => "Administrative and Support Services",
            Self::WasteManagementAndRemediationServices => {
                "Waste Management and Remediation Services"
            }
            Self::EducationalServices => "Educational Services",
            Self::AmbulatoryHealthCareServices => "Ambulatory Health Care Services",
            Self::Hospitals => "Hospitals",
            Self::NursingAndResidentialCareFacilities => "Nursing and Residential Care Facilities",
            Self::SocialAssistance => "Social Assistance",
            Self::PerformingArtsSpectatorSportsAndRelatedIndustries => {
                "Performing Arts, Spectator Sports, and Related Industries"
            }
            Self::MuseumsHistoricalSitesAndSimilarInstitutions => {
                "Museums, Historical Sites, and Similar Institutions"
            }
            Self::AmusementGamblingAndRecreationIndustries => {
                "Amusement, Gambling, and Recreation Industries"
            }
            Self::Accommodation => "Accommodation",
            Self::FoodServicesAndDrinkingPlaces => "Food Services and Drinking Places",
            Self::RepairAndMaintenance => "Repair and Maintenance",
            Self::PersonalAndLaundryServices => "Personal and Laundry Services",
            Self::ReligiousGrantmakingCivicProfessionalAndSimilarOrganizations => {
                "Religious, Grantmaking, Civic, Professional, and Similar Organizations"
            }
            Self::PrivateHouseholds => "Private Households",
            Self::ExecutiveLegislativeAndOtherGeneralGovernmentSupport => {
                "Executive, Legislative, and Other General Government Support"
            }
            Self::JusticePublicOrderAndSafetyActivities => {
                "Justice, Public Order, and Safety Activities"
            }
            Self::AdministrationOfHumanResourcePrograms => {
                "Administration of Human Resource Programs"
            }
            Self::AdministrationOfEnvironmentalQualityPrograms => {
                "Administration of Environmental Quality Programs"
            }
            Self::AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment => {
                "Administration of Housing Programs, Urban Planning, and Community Development"
            }
            Self::AdministrationOfEconomicPrograms => "Administration of Economic Programs",
            Self::SpaceResearchAndTechnology => "Space Research and Technology",
            Self::NationalSecurityAndInternationalAffairs => {
                "National Security and International Affairs"
            }
            Self::UnclassifiedEstablishments => "Unclassified Establishments",
        }
    }

    /// Returns the NAICS code for the subsector
    ///
    /// # Returns
    /// The numeric NAICS code as an i64
    pub fn code(&self) -> i64 {
        match self {
            Self::CropProduction => 111,
            Self::AnimalProductionAndAquaculture => 112,
            Self::ForestryAndLogging => 113,
            Self::FishingHuntingAndTrapping => 114,
            Self::SupportActivitiesForAgricultureAndForestry => 115,
            Self::OilAndGasExtraction => 211,
            Self::MiningExceptOilAndGas => 212,
            Self::SupportActivitiesForMining => 213,
            Self::Utilities => 221,
            Self::ConstructionOfBuildings => 236,
            Self::HeavyAndCivilEngineeringConstruction => 237,
            Self::SpecialtyTradeContractors => 238,
            Self::FoodManufacturing => 311,
            Self::BeverageAndTobaccoProductManufacturing => 312,
            Self::TextileMills => 313,
            Self::TextileProductMills => 314,
            Self::ApparelManufacturing => 315,
            Self::LeatherAndAlliedProductManufacturing => 316,
            Self::WoodProductManufacturing => 321,
            Self::PaperManufacturing => 322,
            Self::PrintingAndRelatedSupportActivities => 323,
            Self::PetroleumAndCoalProductsManufacturing => 324,
            Self::ChemicalManufacturing => 325,
            Self::PlasticsAndRubberProductsManufacturing => 326,
            Self::NonmetallicMineralProductManufacturing => 327,
            Self::PrimaryMetalManufacturing => 331,
            Self::FabricatedMetalProductManufacturing => 332,
            Self::MachineryManufacturing => 333,
            Self::ComputerAndElectronicProductManufacturing => 334,
            Self::ElectricalEquipmentApplianceAndComponentManufacturing => 335,
            Self::TransportationEquipmentManufacturing => 336,
            Self::FurnitureAndRelatedProductManufacturing => 337,
            Self::MiscellaneousManufacturing => 339,
            Self::MerchantWholesalersDurableGoods => 423,
            Self::MerchantWholesalersNondurableGoods => 424,
            Self::WholesaleTradeAgentsAndBrokers => 425,
            Self::MotorVehicleAndPartsDealers => 441,
            Self::FurnitureAndHomeFurnishingsStores => 442,
            Self::ElectronicsAndApplianceStores => 443,
            Self::BuildingMaterialAndGardenEquipmentAndSuppliesDealers => 444,
            Self::FoodAndBeverageRetailers => 445,
            Self::FurnitureHomeFurnishingsElectronicsAndApplianceRetailers => 449,
            Self::GeneralMerchandiseRetailers => 455,
            Self::HealthAndPersonalCareRetailers => 456,
            Self::GasolineStationsAndFuelDealers => 457,
            Self::ClothingClothingAccessoriesShoeAndJewelryRetailers => 458,
            Self::SportingGoodsHobbyMusicalInstrumentBookAndMiscellaneousRetailers => 459,
            Self::AirTransportation => 481,
            Self::RailTransportation => 482,
            Self::WaterTransportation => 483,
            Self::TruckTransportation => 484,
            Self::TransitAndGroundPassengerTransportation => 485,
            Self::PipelineTransportation => 486,
            Self::ScenicAndSightseeingTransportation => 487,
            Self::SupportActivitiesForTransportation => 488,
            Self::PostalService => 491,
            Self::CouriersAndMessengers => 492,
            Self::WarehousingAndStorage => 493,
            Self::PublishingIndustriesExceptInternet => 511,
            Self::MotionPictureAndSoundRecordingIndustries => 512,
            Self::PublishingIndustries => 513,
            Self::BroadcastingExceptInternet => 515,
            Self::BroadcastingAndContentProviders => 516,
            Self::Telecommunications => 517,
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => 518,
            Self::OtherInformationServices => 519,
            Self::MonetaryAuthoritiesCentralBank => 521,
            Self::CreditIntermediationAndRelatedActivities => 522,
            Self::SecuritiesCommodityContractsAndOtherFinancialInvestmentsAndRelatedActivities => {
                523
            }
            Self::InsuranceCarriersAndRelatedActivities => 524,
            Self::FundsTrustsAndOtherFinancialVehicles => 525,
            Self::RealEstate => 531,
            Self::RentalAndLeasingServices => 532,
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => 533,
            Self::ProfessionalScientificAndTechnicalServices => 541,
            Self::ManagementOfCompaniesAndEnterprises => 551,
            Self::AdministrativeAndSupportServices => 561,
            Self::WasteManagementAndRemediationServices => 562,
            Self::EducationalServices => 611,
            Self::AmbulatoryHealthCareServices => 621,
            Self::Hospitals => 622,
            Self::NursingAndResidentialCareFacilities => 623,
            Self::SocialAssistance => 624,
            Self::PerformingArtsSpectatorSportsAndRelatedIndustries => 711,
            Self::MuseumsHistoricalSitesAndSimilarInstitutions => 712,
            Self::AmusementGamblingAndRecreationIndustries => 713,
            Self::Accommodation => 721,
            Self::FoodServicesAndDrinkingPlaces => 722,
            Self::RepairAndMaintenance => 811,
            Self::PersonalAndLaundryServices => 812,
            Self::ReligiousGrantmakingCivicProfessionalAndSimilarOrganizations => 813,
            Self::PrivateHouseholds => 814,
            Self::ExecutiveLegislativeAndOtherGeneralGovernmentSupport => 921,
            Self::JusticePublicOrderAndSafetyActivities => 922,
            Self::AdministrationOfHumanResourcePrograms => 923,
            Self::AdministrationOfEnvironmentalQualityPrograms => 924,
            Self::AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment => 925,
            Self::AdministrationOfEconomicPrograms => 926,
            Self::SpaceResearchAndTechnology => 927,
            Self::NationalSecurityAndInternationalAffairs => 928,
            Self::UnclassifiedEstablishments => 999,
        }
    }

    /// Attempts to find a subsector by its NAICS code
    ///
    /// # Arguments
    /// * `key` - A string representation of the NAICS code
    ///
    /// # Returns
    /// `Some(NaicsSubsector)` if a matching code is found, `None` otherwise
    pub fn from_code(key: &str) -> Option<Self> {
        let code = key.parse::<i64>().ok()?;

        let result = match code {
            111 => Self::CropProduction,
            112 => Self::AnimalProductionAndAquaculture,
            113 => Self::ForestryAndLogging,
            114 => Self::FishingHuntingAndTrapping,
            115 => Self::SupportActivitiesForAgricultureAndForestry,
            211 => Self::OilAndGasExtraction,
            212 => Self::MiningExceptOilAndGas,
            213 => Self::SupportActivitiesForMining,
            221 => Self::Utilities,
            236 => Self::ConstructionOfBuildings,
            237 => Self::HeavyAndCivilEngineeringConstruction,
            238 => Self::SpecialtyTradeContractors,
            311 => Self::FoodManufacturing,
            312 => Self::BeverageAndTobaccoProductManufacturing,
            313 => Self::TextileMills,
            314 => Self::TextileProductMills,
            315 => Self::ApparelManufacturing,
            316 => Self::LeatherAndAlliedProductManufacturing,
            321 => Self::WoodProductManufacturing,
            322 => Self::PaperManufacturing,
            323 => Self::PrintingAndRelatedSupportActivities,
            324 => Self::PetroleumAndCoalProductsManufacturing,
            325 => Self::ChemicalManufacturing,
            326 => Self::PlasticsAndRubberProductsManufacturing,
            327 => Self::NonmetallicMineralProductManufacturing,
            331 => Self::PrimaryMetalManufacturing,
            332 => Self::FabricatedMetalProductManufacturing,
            333 => Self::MachineryManufacturing,
            334 => Self::ComputerAndElectronicProductManufacturing,
            335 => Self::ElectricalEquipmentApplianceAndComponentManufacturing,
            336 => Self::TransportationEquipmentManufacturing,
            337 => Self::FurnitureAndRelatedProductManufacturing,
            339 => Self::MiscellaneousManufacturing,
            423 => Self::MerchantWholesalersDurableGoods,
            424 => Self::MerchantWholesalersNondurableGoods,
            425 => Self::WholesaleTradeAgentsAndBrokers,
            441 => Self::MotorVehicleAndPartsDealers,
            442 => Self::FurnitureAndHomeFurnishingsStores,
            443 => Self::ElectronicsAndApplianceStores,
            444 => Self::BuildingMaterialAndGardenEquipmentAndSuppliesDealers,
            445 => Self::FoodAndBeverageRetailers,
            449 => Self::FurnitureHomeFurnishingsElectronicsAndApplianceRetailers,
            455 => Self::GeneralMerchandiseRetailers,
            456 => Self::HealthAndPersonalCareRetailers,
            457 => Self::GasolineStationsAndFuelDealers,
            458 => Self::ClothingClothingAccessoriesShoeAndJewelryRetailers,
            459 => Self::SportingGoodsHobbyMusicalInstrumentBookAndMiscellaneousRetailers,
            481 => Self::AirTransportation,
            482 => Self::RailTransportation,
            483 => Self::WaterTransportation,
            484 => Self::TruckTransportation,
            485 => Self::TransitAndGroundPassengerTransportation,
            486 => Self::PipelineTransportation,
            487 => Self::ScenicAndSightseeingTransportation,
            488 => Self::SupportActivitiesForTransportation,
            491 => Self::PostalService,
            492 => Self::CouriersAndMessengers,
            493 => Self::WarehousingAndStorage,
            511 => Self::PublishingIndustriesExceptInternet,
            512 => Self::MotionPictureAndSoundRecordingIndustries,
            513 => Self::PublishingIndustries,
            515 => Self::BroadcastingExceptInternet,
            516 => Self::BroadcastingAndContentProviders,
            517 => Self::Telecommunications,
            518 => Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices,
            519 => Self::OtherInformationServices,
            521 => Self::MonetaryAuthoritiesCentralBank,
            522 => Self::CreditIntermediationAndRelatedActivities,
            523 => {
                Self::SecuritiesCommodityContractsAndOtherFinancialInvestmentsAndRelatedActivities
            }
            524 => Self::InsuranceCarriersAndRelatedActivities,
            525 => Self::FundsTrustsAndOtherFinancialVehicles,
            531 => Self::RealEstate,
            532 => Self::RentalAndLeasingServices,
            533 => Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks,
            541 => Self::ProfessionalScientificAndTechnicalServices,
            551 => Self::ManagementOfCompaniesAndEnterprises,
            561 => Self::AdministrativeAndSupportServices,
            562 => Self::WasteManagementAndRemediationServices,
            611 => Self::EducationalServices,
            621 => Self::AmbulatoryHealthCareServices,
            622 => Self::Hospitals,
            623 => Self::NursingAndResidentialCareFacilities,
            624 => Self::SocialAssistance,
            711 => Self::PerformingArtsSpectatorSportsAndRelatedIndustries,
            712 => Self::MuseumsHistoricalSitesAndSimilarInstitutions,
            713 => Self::AmusementGamblingAndRecreationIndustries,
            721 => Self::Accommodation,
            722 => Self::FoodServicesAndDrinkingPlaces,
            811 => Self::RepairAndMaintenance,
            812 => Self::PersonalAndLaundryServices,
            813 => Self::ReligiousGrantmakingCivicProfessionalAndSimilarOrganizations,
            814 => Self::PrivateHouseholds,
            921 => Self::ExecutiveLegislativeAndOtherGeneralGovernmentSupport,
            922 => Self::JusticePublicOrderAndSafetyActivities,
            923 => Self::AdministrationOfHumanResourcePrograms,
            924 => Self::AdministrationOfEnvironmentalQualityPrograms,
            925 => Self::AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment,
            926 => Self::AdministrationOfEconomicPrograms,
            927 => Self::SpaceResearchAndTechnology,
            928 => Self::NationalSecurityAndInternationalAffairs,
            999 => Self::UnclassifiedEstablishments,
            _ => return None,
        };

        Some(result)
    }
}
