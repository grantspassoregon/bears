use strum::IntoEnumIterator;

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
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
)]
/// Fixed asset line classifications
pub enum FixedAssetLine {
    #[default]
    /// 1 to 4 unit
    OneToFourUnit,
    /// 5-or more-unit
    FiveOrMoreUnit,
    /// Accommodation
    Accommodation,
    /// Accommodation and food services
    AccommodationFoodServices,
    /// Administrative and support services
    AdministrativeSupportServices,
    /// Administrative and waste management services
    AdministrativeWasteManagementServices,
    /// Aerospace products and parts manufacturing
    AerospaceProductsPartsManufacturing,
    /// Agricultural machinery
    AgriculturalMachinery,
    /// Agriculture, forestry, fishing, and hunting
    AgricultureForestryFishingHunting,
    /// Air
    Air,
    /// Air transportation
    AirTransportation,
    /// Aircraft
    Aircraft,
    /// All other nonmanufacturing
    AllOtherNonmanufacturing,
    /// Ambulatory health care services
    AmbulatoryHealthCareServices,
    /// Amusement and recreation
    AmusementRecreation,
    /// Amusements, gambling, and recreation industries
    AmusementsGamblingRecreationIndustries,
    /// Apparel and leather and allied products
    ApparelLeatherAlliedProducts,
    /// Arts, entertainment, and recreation
    ArtsEntertainmentRecreation,
    /// Autos
    Autos,
    /// Beverage and tobacco product manufacturing
    BeverageTobaccoProductManufacturing,
    /// Books
    Books,
    /// Broadcasting and telecommunications
    BroadcastingTelecommunications,
    /// Brokers' commissions and other ownership transfer costs
    BrokersCommissionsOtherOwnershipTransferCosts,
    /// Buildings
    Buildings,
    /// Business
    Business,
    /// Chemical manufacturing, excluding pharmaceutical and medicine
    ChemicalManufacturingExcludingPharmaceuticalMedicine,
    /// Chemical products
    ChemicalProducts,
    /// Commercial
    Commercial,
    /// Commercial and health care
    CommercialHealthCare,
    /// Communication
    Communication,
    /// Communication equipment
    CommunicationEquipment,
    /// Computer and electronic products
    ComputerElectronicProducts,
    /// Computer systems design and related services
    ComputerSystemsDesignRelatedServices,
    /// Computers and peripheral equipment
    ComputersPeripheralEquipment,
    /// Conservation and development
    ConservationDevelopment,
    /// Construction
    Construction,
    /// Construction machinery
    ConstructionMachinery,
    /// Consumer durable goods
    ConsumerDurableGoods,
    /// Corporate
    Corporate,
    /// Credit intermediation and related activities
    CreditIntermediationRelatedActivities,
    /// Custom
    Custom,
    /// Depository credit intermediation
    DepositoryreditIntermediation,
    /// Durable goods
    DurableGoods,
    /// Educational
    Educational,
    /// Educational and vocational
    EducationalVocational,
    /// Educational books
    EducationalBooks,
    /// Educational services
    EducationalServices,
    /// Electric
    Electric,
    /// Electric power
    ElectricPower,
    /// Electrical equipment, appliances, and components
    ElectricalEquipmentAppliancesComponents,
    /// Electrical equipment, n.e.c.
    ElectricalEquipmentNec,
    /// Electrical transmission, distribution, and industrial apparatus
    ElectricalTransmissionDistributionIndustrialApparatus,
    /// Electronics
    Electronics,
    /// Engines and turbines
    EnginesTurbines,
    /// Entertainment, literary, and artistic originals
    EntertainmentLiteraryArtisticOriginals,
    /// Equipment
    Equipment,
    /// Fabricated metal products
    FabricatedMetalProducts,
    /// Farm
    Farm,
    /// Farms
    Farms,
    /// Federal
    Federal,
    /// Federal Reserve banks
    FederalReserveBanks,
    /// Finance and insurance
    FinanceInsurance,
    /// Financial
    Financial,
    /// Fixed assets
    FixedAssets,
    /// Fixed assets and consumer durable goods
    FixedAssetsConsumerDurableGoods,
    /// Food and beverage and tobacco products
    FoodBeverageTobaccoProducts,
    /// Food and beverage establishments
    FoodBeverageEstablishments,
    /// Food and beverage stores
    FoodBeverageStores,
    /// Food manufacturing
    FoodManufacturing,
    /// Food services and drinking places
    FoodServicesDrinkingPlaces,
    /// Forestry, fishing, and related activities
    ForestryFishingRelatedActivities,
    /// Funds, trusts, and other financial vehicles
    FundsTrustsOtherFinancialVehicles,
    /// Furnishings and durable household equipment
    FurnishingsDurableHouseholdEquipment,
    /// Furniture and fixtures
    FurnitureFixtures,
    /// Furniture and furnishings
    FurnitureFurnishings,
    /// Furniture and related products
    FurnitureRelatedProducts,
    /// General government fixed assets
    GeneralGovernmentFixedAssets,
    /// General industrial, including materials handling, equipment
    GeneralIndustrialIncludingMaterialsHandlingEquipment,
    /// General merchandise stores
    GeneralMerchandiseStores,
    /// Glassware, tableware, and household utensils
    GlasswareTablewareHouseholdUtensils,
    /// Government
    Government,
    /// Government enterprise fixed assets
    GovernmentEnterpriseFixedAssets,
    /// Government fixed assets
    GovernmentFixedAssets,
    /// Government nonresidential fixed assets
    GovernmentNonresidentialFixedAssets,
    /// Health and social assistance
    HealthSocialAssistance,
    /// Health care
    HealthCare,
    /// Highways and streets
    HighwaysStreets,
    /// Hospitals
    Hospitals,
    /// Hospitals and special care
    HospitalsSpecialCare,
    /// Household appliances
    HouseholdAppliances,
    /// Households
    Households,
    /// Housing units
    HousingUnits,
    /// Improvements
    Improvements,
    /// Industrial
    Industrial,
    /// Industrial equipment
    IndustrialEquipment,
    /// Information
    Information,
    /// Information and data processing services
    InformationDataProcessingServices,
    /// Information processing equipment
    InformationProcessingEquipment,
    /// Insurance agencies and brokers and related services
    InsuranceAgenciesBrokersRelatedServices,
    /// Insurance carriers
    InsuranceCarriers,
    /// Insurance carriers and related activities
    InsuranceCarriersRelatedActivities,
    /// Intellectual property products
    IntellectualPropertyProducts,
    /// Jewelry and watches
    JewelryWatches,
    /// Land
    Land,
    /// Legal services
    LegalServices,
    /// Light trucks
    LightTrucks,
    /// Light trucks (including utility vehicles)
    LightTrucksIncludingUtilityVehicles,
    /// Lodging
    Lodging,
    /// Long-lived television programs
    LongLivedTelevisionPrograms,
    /// Luggage and similar personal items
    LuggageSimilarPersonalItems,
    /// Machinery
    Machinery,
    /// Management of companies and enterprises
    ManagementOfCompaniesEnterprises,
    /// Manufactured homes
    ManufacturedHomes,
    /// Manufacturing
    Manufacturing,
    /// Medical buildings
    MedicalBuildings,
    /// Medical equipment and instruments
    MedicalEquipmentInstruments,
    /// Medical equipment and supplies
    MedicalEquipmentSupplies,
    /// Metalworking machinery
    MetalworkingMachinery,
    /// Military facilities
    MilitaryFacilities,
    /// Mining
    Mining,
    /// Mining and oilfield machinery
    MiningOilfieldMachinery,
    /// Mining exploration, shafts, and wells
    MiningExplorationShaftsWells,
    /// Mining, except oil and gas
    MiningExceptOilGas,
    /// Miscellaneous manufacturing
    MiscellaneousManufacturing,
    /// Miscellaneous professional, scientific, and technical services
    MiscellaneousProfessionalScientificTechnicalServices,
    /// Missiles
    Missiles,
    /// Motion picture and sound recording industries
    MotionPictureSoundRecordingIndustries,
    /// Motor vehicle and parts dealers
    MotorVehiclePartsDealers,
    /// Motor vehicle parts and accessories
    MotorVehiclePartsAccessories,
    /// Motor vehicles and parts
    MotorVehiclesParts,
    /// Motor vehicles, bodies and trailers, and parts
    MotorVehiclesBodiesTrailersParts,
    /// Motor vehicles, bodies and trailers, and parts manufacturing
    MotorVehiclesBodiesTrailersPartsManufacturing,
    /// Multimerchandise shopping
    MultimerchandiseShopping,
    /// Music
    Music,
    /// Musical instruments
    MusicalInstruments,
    /// National defense
    NationalDefense,
    /// Natural gas distribution
    NaturalGasDistribution,
    /// Noncorporate
    Noncorporate,
    /// Nondefense
    Nondefense,
    /// Nondepository credit intermediation
    NondepositoryCreditIntermediation,
    /// Nondurable goods
    NondurableGoods,
    /// Nonfarm nonmanufacturing
    NonfarmNonmanufacturing,
    /// Nonfarm nonmanufacturing (nonresidential fixed assets only)
    NonfarmNonmanufacturingNonresidentialFixedAssetsOnly,
    /// Nonfinancial
    Nonfinancial,
    /// Nonmanufacturing
    Nonmanufacturing,
    /// Nonmedical instruments
    NonmedicalInstruments,
    /// Nonmetallic mineral products
    NonmetallicMineralProducts,
    /// Nonprofit institutions
    NonprofitInstitutions,
    /// Nonprofit institutions serving households (NPISHs)
    NonprofitInstitutionsServingHouseholdsNpishs,
    /// Nonresidential
    Nonresidential,
    /// Nonresidential equipment
    NonresidentialEquipment,
    /// Nonresidential intellectual property products
    NonresidentialIntellectualPropertyProducts,
    /// Nonresidential structures
    NonresidentialStructures,
    /// Nursing and residential care facilities
    NursingResidentialCareFacilities,
    /// Office
    Office,
    /// Office and accounting equipment
    OfficeAccountingEquipment,
    /// Oil and gas extraction
    OilGasExtraction,
    /// Other
    Other,
    /// Other commercial
    OtherCommercial,
    /// Other computer and electronic product manufacturing
    OtherComputerElectronicProductManufacturing,
    /// Other durable goods
    OtherDurableGoods,
    /// Other equipment
    OtherEquipment,
    /// Other intermediation
    OtherIntermediation,
    /// Other manufacturing
    OtherManufacturing,
    /// Other nonprofit institutions
    OtherNonprofitInstitutions,
    /// Other nonresidential equipment
    OtherNonresidentialEquipment,
    /// Other power
    OtherPower,
    /// Other residential
    OtherResidential,
    /// Other retail
    OtherRetail,
    /// Other services, except government
    OtherServicesExceptGovernment,
    /// Other structures
    OtherStructures,
    /// Other transportation and support activities
    OtherTransportationSupportActivities,
    /// Other transportation equipment
    OtherTransportationEquipment,
    /// Other trucks, buses, and truck trailers
    OtherTrucksBusesTruckTrailers,
    /// Own account
    OwnAccount,
    /// Owner-occupied
    OwnerOccupied,
    /// Paper products
    PaperProducts,
    /// Partnerships
    Partnerships,
    /// Performing arts, spectator sports, museums, and related activities
    PerformingArtsSpectatorSportsMuseumsRelatedActivities,
    /// Permanent site
    PermanentSite,
    /// Petroleum and coal products
    PetroleumCoalProducts,
    /// Petroleum and natural gas
    PetroleumNaturalGas,
    /// Pharmaceutical and medicine manufacturing
    PharmaceuticalMedicineManufacturing,
    /// Photocopy and related equipment
    PhotocopyRelatedEquipment,
    /// Pipeline transportation
    PipelineTransportation,
    /// Plastics and rubber products
    PlasticsRubberProducts,
    /// Power
    Power,
    /// Power and communication
    PowerCommunication,
    /// Prepackaged
    Prepackaged,
    /// Primary metals
    PrimaryMetals,
    /// Printing and related support activities
    PrintingRelatedSupportActivities,
    /// Private
    Private,
    /// Private and government fixed assets
    PrivateGovernmentFixedAssets,
    /// Private fixed assets
    PrivateFixedAssets,
    /// Private nonresidential fixed assets
    PrivateNonresidentialFixedAssets,
    /// Professional, scientific, and technical services
    ProfessionalScientificTechnicalServices,
    /// Public safety
    PublicSafety,
    /// Publishing industries (includes software)
    PublishingIndustriesIncludesSoftware,
    /// Railroad equipment
    RailroadEquipment,
    /// Railroad transportation
    RailroadTransportation,
    /// Real estate
    RealEstate,
    /// Real estate and rental and leasing
    RealEstateRentalLeasing,
    /// Recreational books
    RecreationalBooks,
    /// Recreational goods and vehicles
    RecreationalGoodsVehicles,
    /// Religious
    Religious,
    /// Rental and leasing services and lessors of intangible assets
    RentalLeasingServicesLessorsOfIntangibleAssets,
    /// Research and development
    ResearchDevelopment,
    /// Residential
    Residential,
    /// Residential equipment
    ResidentialEquipment,
    /// Residential fixed assets
    ResidentialFixedAssets,
    /// Residential structures
    ResidentialStructures,
    /// Retail trade
    RetailTrade,
    /// Scientific research and development services
    ScientificResearchDevelopmentServices,
    /// Securities, commodity contracts, and investments
    SecuritiesCommodityContractsInvestments,
    /// Semiconductor and other electronic component manufacturing
    SemiconductorOtherElectronicComponentManufacturing,
    /// Service industry machinery
    ServiceIndustryMachinery,
    /// Sewer systems
    SewerSystems,
    /// Ships
    Ships,
    /// Ships and boats
    ShipsBoats,
    /// Social assistance
    SocialAssistance,
    /// Software
    Software,
    /// Sole proprietorships
    SoleProprietorships,
    /// Sole proprietorships and partnerships
    SoleProprietorshipsPartnerships,
    /// Special care
    SpecialCare,
    /// Special industry machinery, n.e.c.
    SpecialIndustryMachineryNec,
    /// Sporting equipment, supplies, guns, and ammunition
    SportingEquipmentSuppliesGunsAmmunition,
    /// Sports and recreational vehicles
    SportsRecreationalVehicles,
    /// State and local
    StateLocal,
    /// Structures
    Structures,
    /// Support activities for mining
    SupportActivitiesForMining,
    /// Tax-exempt cooperatives
    TaxExemptCooperatives,
    /// Telephone and facsimile equipment
    TelephoneFacsimileEquipment,
    /// Tenant-occupied
    TenantOccupied,
    /// Textile mills and textile product mills
    TextileMillsTextileProductMills,
    /// Theatrical movies
    TheatricalMovies,
    /// Therapeutic appliances and equipment
    TherapeuticAppliancesEquipment,
    /// Tools and equipment for house and garden
    ToolsEquipmentForHouseGarden,
    /// Transit and ground passenger transportation
    TransitGroundPassengerTransportation,
    /// Transportation
    Transportation,
    /// Transportation and warehousing
    TransportationWarehousing,
    /// Transportation equipment
    TransportationEquipment,
    /// Transportion
    Transportion,
    /// Truck transportation
    TruckTransportation,
    /// Trucks, buses, and truck trailers
    TrucksBusesTruckTrailers,
    /// Universities and colleges
    UniversitiesColleges,
    /// Utilities
    Utilities,
    /// Vehicles
    Vehicles,
    /// Video, audio, photographic, and information processing equipment and media
    VideoAudioPhotographicInformationProcessingEquipmentMedia,
    /// Warehouses
    Warehouses,
    /// Warehousing and storage
    WarehousingStorage,
    /// Waste management and remediation services
    WasteManagementRemediationServices,
    /// Water systems
    WaterSystems,
    /// Water transportation
    WaterTransportation,
    /// Water, sewage, and other systems
    WaterSewageOtherSystems,
    /// Wholesale trade
    WholesaleTrade,
    /// Wood products
    WoodProducts,
}

impl FixedAssetLine {
    /// Returns the original string description for this fixed asset line
    pub fn description(&self) -> &'static str {
        match self {
            Self::OneToFourUnit => "1 to 4 unit",
            Self::FiveOrMoreUnit => "5-or more-unit",
            Self::Accommodation => "Accommodation",
            Self::AccommodationFoodServices => "Accommodation and food services",
            Self::AdministrativeSupportServices => "Administrative and support services",
            Self::AdministrativeWasteManagementServices => {
                "Administrative and waste management services"
            }
            Self::AerospaceProductsPartsManufacturing => {
                "Aerospace products and parts manufacturing"
            }
            Self::AgriculturalMachinery => "Agricultural machinery",
            Self::AgricultureForestryFishingHunting => {
                "Agriculture, forestry, fishing, and hunting"
            }
            Self::Air => "Air",
            Self::AirTransportation => "Air transportation",
            Self::Aircraft => "Aircraft",
            Self::AllOtherNonmanufacturing => "All other nonmanufacturing",
            Self::AmbulatoryHealthCareServices => "Ambulatory health care services",
            Self::AmusementRecreation => "Amusement and recreation",
            Self::AmusementsGamblingRecreationIndustries => {
                "Amusements, gambling, and recreation industries"
            }
            Self::ApparelLeatherAlliedProducts => "Apparel and leather and allied products",
            Self::ArtsEntertainmentRecreation => "Arts, entertainment, and recreation",
            Self::Autos => "Autos",
            Self::BeverageTobaccoProductManufacturing => {
                "Beverage and tobacco product manufacturing"
            }
            Self::Books => "Books",
            Self::BroadcastingTelecommunications => "Broadcasting and telecommunications",
            Self::BrokersCommissionsOtherOwnershipTransferCosts => {
                "Brokers' commissions and other ownership transfer costs"
            }
            Self::Buildings => "Buildings",
            Self::Business => "Business",
            Self::ChemicalManufacturingExcludingPharmaceuticalMedicine => {
                "Chemical manufacturing, excluding pharmaceutical and medicine"
            }
            Self::ChemicalProducts => "Chemical products",
            Self::Commercial => "Commercial",
            Self::CommercialHealthCare => "Commercial and health care",
            Self::Communication => "Communication",
            Self::CommunicationEquipment => "Communication equipment",
            Self::ComputerElectronicProducts => "Computer and electronic products",
            Self::ComputerSystemsDesignRelatedServices => {
                "Computer systems design and related services"
            }
            Self::ComputersPeripheralEquipment => "Computers and peripheral equipment",
            Self::ConservationDevelopment => "Conservation and development",
            Self::Construction => "Construction",
            Self::ConstructionMachinery => "Construction machinery",
            Self::ConsumerDurableGoods => "Consumer durable goods",
            Self::Corporate => "Corporate",
            Self::CreditIntermediationRelatedActivities => {
                "Credit intermediation and related activities"
            }
            Self::Custom => "Custom",
            Self::DepositoryreditIntermediation => "Depository credit intermediation",
            Self::DurableGoods => "Durable goods",
            Self::Educational => "Educational",
            Self::EducationalVocational => "Educational and vocational",
            Self::EducationalBooks => "Educational books",
            Self::EducationalServices => "Educational services",
            Self::Electric => "Electric",
            Self::ElectricPower => "Electric power",
            Self::ElectricalEquipmentAppliancesComponents => {
                "Electrical equipment, appliances, and components"
            }
            Self::ElectricalEquipmentNec => "Electrical equipment, n.e.c.",
            Self::ElectricalTransmissionDistributionIndustrialApparatus => {
                "Electrical transmission, distribution, and industrial apparatus"
            }
            Self::Electronics => "Electronics",
            Self::EnginesTurbines => "Engines and turbines",
            Self::EntertainmentLiteraryArtisticOriginals => {
                "Entertainment, literary, and artistic originals"
            }
            Self::Equipment => "Equipment",
            Self::FabricatedMetalProducts => "Fabricated metal products",
            Self::Farm => "Farm",
            Self::Farms => "Farms",
            Self::Federal => "Federal",
            Self::FederalReserveBanks => "Federal Reserve banks",
            Self::FinanceInsurance => "Finance and insurance",
            Self::Financial => "Financial",
            Self::FixedAssets => "Fixed assets",
            Self::FixedAssetsConsumerDurableGoods => "Fixed assets and consumer durable goods",
            Self::FoodBeverageTobaccoProducts => "Food and beverage and tobacco products",
            Self::FoodBeverageEstablishments => "Food and beverage establishments",
            Self::FoodBeverageStores => "Food and beverage stores",
            Self::FoodManufacturing => "Food manufacturing",
            Self::FoodServicesDrinkingPlaces => "Food services and drinking places",
            Self::ForestryFishingRelatedActivities => "Forestry, fishing, and related activities",
            Self::FundsTrustsOtherFinancialVehicles => {
                "Funds, trusts, and other financial vehicles"
            }
            Self::FurnishingsDurableHouseholdEquipment => {
                "Furnishings and durable household equipment"
            }
            Self::FurnitureFixtures => "Furniture and fixtures",
            Self::FurnitureFurnishings => "Furniture and furnishings",
            Self::FurnitureRelatedProducts => "Furniture and related products",
            Self::GeneralGovernmentFixedAssets => "General government fixed assets",
            Self::GeneralIndustrialIncludingMaterialsHandlingEquipment => {
                "General industrial, including materials handling, equipment"
            }
            Self::GeneralMerchandiseStores => "General merchandise stores",
            Self::GlasswareTablewareHouseholdUtensils => {
                "Glassware, tableware, and household utensils"
            }
            Self::Government => "Government",
            Self::GovernmentEnterpriseFixedAssets => "Government enterprise fixed assets",
            Self::GovernmentFixedAssets => "Government fixed assets",
            Self::GovernmentNonresidentialFixedAssets => "Government nonresidential fixed assets",
            Self::HealthSocialAssistance => "Health and social assistance",
            Self::HealthCare => "Health care",
            Self::HighwaysStreets => "Highways and streets",
            Self::Hospitals => "Hospitals",
            Self::HospitalsSpecialCare => "Hospitals and special care",
            Self::HouseholdAppliances => "Household appliances",
            Self::Households => "Households",
            Self::HousingUnits => "Housing units",
            Self::Improvements => "Improvements",
            Self::Industrial => "Industrial",
            Self::IndustrialEquipment => "Industrial equipment",
            Self::Information => "Information",
            Self::InformationDataProcessingServices => "Information and data processing services",
            Self::InformationProcessingEquipment => "Information processing equipment",
            Self::InsuranceAgenciesBrokersRelatedServices => {
                "Insurance agencies and brokers and related services"
            }
            Self::InsuranceCarriers => "Insurance carriers",
            Self::InsuranceCarriersRelatedActivities => "Insurance carriers and related activities",
            Self::IntellectualPropertyProducts => "Intellectual property products",
            Self::JewelryWatches => "Jewelry and watches",
            Self::Land => "Land",
            Self::LegalServices => "Legal services",
            Self::LightTrucks => "Light trucks",
            Self::LightTrucksIncludingUtilityVehicles => {
                "Light trucks (including utility vehicles)"
            }
            Self::Lodging => "Lodging",
            Self::LongLivedTelevisionPrograms => "Long-lived television programs",
            Self::LuggageSimilarPersonalItems => "Luggage and similar personal items",
            Self::Machinery => "Machinery",
            Self::ManagementOfCompaniesEnterprises => "Management of companies and enterprises",
            Self::ManufacturedHomes => "Manufactured homes",
            Self::Manufacturing => "Manufacturing",
            Self::MedicalBuildings => "Medical buildings",
            Self::MedicalEquipmentInstruments => "Medical equipment and instruments",
            Self::MedicalEquipmentSupplies => "Medical equipment and supplies",
            Self::MetalworkingMachinery => "Metalworking machinery",
            Self::MilitaryFacilities => "Military facilities",
            Self::Mining => "Mining",
            Self::MiningOilfieldMachinery => "Mining and oilfield machinery",
            Self::MiningExplorationShaftsWells => "Mining exploration, shafts, and wells",
            Self::MiningExceptOilGas => "Mining, except oil and gas",
            Self::MiscellaneousManufacturing => "Miscellaneous manufacturing",
            Self::MiscellaneousProfessionalScientificTechnicalServices => {
                "Miscellaneous professional, scientific, and technical services"
            }
            Self::Missiles => "Missiles",
            Self::MotionPictureSoundRecordingIndustries => {
                "Motion picture and sound recording industries"
            }
            Self::MotorVehiclePartsDealers => "Motor vehicle and parts dealers",
            Self::MotorVehiclePartsAccessories => "Motor vehicle parts and accessories",
            Self::MotorVehiclesParts => "Motor vehicles and parts",
            Self::MotorVehiclesBodiesTrailersParts => {
                "Motor vehicles, bodies and trailers, and parts"
            }
            Self::MotorVehiclesBodiesTrailersPartsManufacturing => {
                "Motor vehicles, bodies and trailers, and parts manufacturing"
            }
            Self::MultimerchandiseShopping => "Multimerchandise shopping",
            Self::Music => "Music",
            Self::MusicalInstruments => "Musical instruments",
            Self::NationalDefense => "National defense",
            Self::NaturalGasDistribution => "Natural gas distribution",
            Self::Noncorporate => "Noncorporate",
            Self::Nondefense => "Nondefense",
            Self::NondepositoryCreditIntermediation => "Nondepository credit intermediation",
            Self::NondurableGoods => "Nondurable goods",
            Self::NonfarmNonmanufacturing => "Nonfarm nonmanufacturing",
            Self::NonfarmNonmanufacturingNonresidentialFixedAssetsOnly => {
                "Nonfarm nonmanufacturing (nonresidential fixed assets only)"
            }
            Self::Nonfinancial => "Nonfinancial",
            Self::Nonmanufacturing => "Nonmanufacturing",
            Self::NonmedicalInstruments => "Nonmedical instruments",
            Self::NonmetallicMineralProducts => "Nonmetallic mineral products",
            Self::NonprofitInstitutions => "Nonprofit institutions",
            Self::NonprofitInstitutionsServingHouseholdsNpishs => {
                "Nonprofit institutions serving households (NPISHs)"
            }
            Self::Nonresidential => "Nonresidential",
            Self::NonresidentialEquipment => "Nonresidential equipment",
            Self::NonresidentialIntellectualPropertyProducts => {
                "Nonresidential intellectual property products"
            }
            Self::NonresidentialStructures => "Nonresidential structures",
            Self::NursingResidentialCareFacilities => "Nursing and residential care facilities",
            Self::Office => "Office",
            Self::OfficeAccountingEquipment => "Office and accounting equipment",
            Self::OilGasExtraction => "Oil and gas extraction",
            Self::Other => "Other",
            Self::OtherCommercial => "Other commercial",
            Self::OtherComputerElectronicProductManufacturing => {
                "Other computer and electronic product manufacturing"
            }
            Self::OtherDurableGoods => "Other durable goods",
            Self::OtherEquipment => "Other equipment",
            Self::OtherIntermediation => "Other intermediation",
            Self::OtherManufacturing => "Other manufacturing",
            Self::OtherNonprofitInstitutions => "Other nonprofit institutions",
            Self::OtherNonresidentialEquipment => "Other nonresidential equipment",
            Self::OtherPower => "Other power",
            Self::OtherResidential => "Other residential",
            Self::OtherRetail => "Other retail",
            Self::OtherServicesExceptGovernment => "Other services, except government",
            Self::OtherStructures => "Other structures",
            Self::OtherTransportationSupportActivities => {
                "Other transportation and support activities"
            }
            Self::OtherTransportationEquipment => "Other transportation equipment",
            Self::OtherTrucksBusesTruckTrailers => "Other trucks, buses, and truck trailers",
            Self::OwnAccount => "Own account",
            Self::OwnerOccupied => "Owner-occupied",
            Self::PaperProducts => "Paper products",
            Self::Partnerships => "Partnerships",
            Self::PerformingArtsSpectatorSportsMuseumsRelatedActivities => {
                "Performing arts, spectator sports, museums, and related activities"
            }
            Self::PermanentSite => "Permanent site",
            Self::PetroleumCoalProducts => "Petroleum and coal products",
            Self::PetroleumNaturalGas => "Petroleum and natural gas",
            Self::PharmaceuticalMedicineManufacturing => {
                "Pharmaceutical and medicine manufacturing"
            }
            Self::PhotocopyRelatedEquipment => "Photocopy and related equipment",
            Self::PipelineTransportation => "Pipeline transportation",
            Self::PlasticsRubberProducts => "Plastics and rubber products",
            Self::Power => "Power",
            Self::PowerCommunication => "Power and communication",
            Self::Prepackaged => "Prepackaged",
            Self::PrimaryMetals => "Primary metals",
            Self::PrintingRelatedSupportActivities => "Printing and related support activities",
            Self::Private => "Private",
            Self::PrivateGovernmentFixedAssets => "Private and government fixed assets",
            Self::PrivateFixedAssets => "Private fixed assets",
            Self::PrivateNonresidentialFixedAssets => "Private nonresidential fixed assets",
            Self::ProfessionalScientificTechnicalServices => {
                "Professional, scientific, and technical services"
            }
            Self::PublicSafety => "Public safety",
            Self::PublishingIndustriesIncludesSoftware => {
                "Publishing industries (includes software)"
            }
            Self::RailroadEquipment => "Railroad equipment",
            Self::RailroadTransportation => "Railroad transportation",
            Self::RealEstate => "Real estate",
            Self::RealEstateRentalLeasing => "Real estate and rental and leasing",
            Self::RecreationalBooks => "Recreational books",
            Self::RecreationalGoodsVehicles => "Recreational goods and vehicles",
            Self::Religious => "Religious",
            Self::RentalLeasingServicesLessorsOfIntangibleAssets => {
                "Rental and leasing services and lessors of intangible assets"
            }
            Self::ResearchDevelopment => "Research and development",
            Self::Residential => "Residential",
            Self::ResidentialEquipment => "Residential equipment",
            Self::ResidentialFixedAssets => "Residential fixed assets",
            Self::ResidentialStructures => "Residential structures",
            Self::RetailTrade => "Retail trade",
            Self::ScientificResearchDevelopmentServices => {
                "Scientific research and development services"
            }
            Self::SecuritiesCommodityContractsInvestments => {
                "Securities, commodity contracts, and investments"
            }
            Self::SemiconductorOtherElectronicComponentManufacturing => {
                "Semiconductor and other electronic component manufacturing"
            }
            Self::ServiceIndustryMachinery => "Service industry machinery",
            Self::SewerSystems => "Sewer systems",
            Self::Ships => "Ships",
            Self::ShipsBoats => "Ships and boats",
            Self::SocialAssistance => "Social assistance",
            Self::Software => "Software",
            Self::SoleProprietorships => "Sole proprietorships",
            Self::SoleProprietorshipsPartnerships => "Sole proprietorships and partnerships",
            Self::SpecialCare => "Special care",
            Self::SpecialIndustryMachineryNec => "Special industry machinery, n.e.c.",
            Self::SportingEquipmentSuppliesGunsAmmunition => {
                "Sporting equipment, supplies, guns, and ammunition"
            }
            Self::SportsRecreationalVehicles => "Sports and recreational vehicles",
            Self::StateLocal => "State and local",
            Self::Structures => "Structures",
            Self::SupportActivitiesForMining => "Support activities for mining",
            Self::TaxExemptCooperatives => "Tax-exempt cooperatives",
            Self::TelephoneFacsimileEquipment => "Telephone and facsimile equipment",
            Self::TenantOccupied => "Tenant-occupied",
            Self::TextileMillsTextileProductMills => "Textile mills and textile product mills",
            Self::TheatricalMovies => "Theatrical movies",
            Self::TherapeuticAppliancesEquipment => "Therapeutic appliances and equipment",
            Self::ToolsEquipmentForHouseGarden => "Tools and equipment for house and garden",
            Self::TransitGroundPassengerTransportation => {
                "Transit and ground passenger transportation"
            }
            Self::Transportation => "Transportation",
            Self::TransportationWarehousing => "Transportation and warehousing",
            Self::TransportationEquipment => "Transportation equipment",
            Self::Transportion => "Transportion",
            Self::TruckTransportation => "Truck transportation",
            Self::TrucksBusesTruckTrailers => "Trucks, buses, and truck trailers",
            Self::UniversitiesColleges => "Universities and colleges",
            Self::Utilities => "Utilities",
            Self::Vehicles => "Vehicles",
            Self::VideoAudioPhotographicInformationProcessingEquipmentMedia => {
                "Video, audio, photographic, and information processing equipment and media"
            }
            Self::Warehouses => "Warehouses",
            Self::WarehousingStorage => "Warehousing and storage",
            Self::WasteManagementRemediationServices => "Waste management and remediation services",
            Self::WaterSystems => "Water systems",
            Self::WaterTransportation => "Water transportation",
            Self::WaterSewageOtherSystems => "Water, sewage, and other systems",
            Self::WholesaleTrade => "Wholesale trade",
            Self::WoodProducts => "Wood products",
        }
    }

    /// Creates a FixedAssetLine from a description string
    pub fn from_description(description: &str) -> Option<Self> {
        Self::iter().find(|variant| variant.description() == description)
    }

    /// Returns the code values associated with this fixed asset line
    pub fn codes(&self) -> Vec<i64> {
        match self {
            Self::OneToFourUnit => vec![70],
            Self::FiveOrMoreUnit => vec![71],
            Self::Accommodation => vec![94],
            Self::AccommodationFoodServices => vec![93],
            Self::AdministrativeSupportServices => vec![82],
            Self::AdministrativeWasteManagementServices => vec![81],
            Self::AerospaceProductsPartsManufacturing => vec![90],
            Self::AgriculturalMachinery => vec![28],
            Self::AgricultureForestryFishingHunting => vec![2],
            Self::Air => vec![63],
            Self::AirTransportation => vec![49],
            Self::Aircraft => vec![23, 24],
            Self::AllOtherNonmanufacturing => vec![94],
            Self::AmbulatoryHealthCareServices => vec![86],
            Self::AmusementRecreation => vec![11, 46, 61, 64],
            Self::AmusementsGamblingRecreationIndustries => vec![92],
            Self::ApparelLeatherAlliedProducts => vec![34],
            Self::ArtsEntertainmentRecreation => vec![90],
            Self::Autos => vec![3, 22],
            Self::BeverageTobaccoProductManufacturing => vec![32],
            Self::Books => vec![101],
            Self::BroadcastingTelecommunications => vec![60],
            Self::BrokersCommissionsOtherOwnershipTransferCosts => vec![73],
            Self::Buildings => vec![31],
            Self::Business => vec![83],
            Self::ChemicalManufacturingExcludingPharmaceuticalMedicine => vec![86],
            Self::ChemicalProducts => vec![38],
            Self::Commercial => vec![7, 42, 60],
            Self::CommercialHealthCare => vec![37],
            Self::Communication => vec![53],
            Self::CommunicationEquipment => vec![6],
            Self::ComputerElectronicProducts => vec![21],
            Self::ComputerSystemsDesignRelatedServices => vec![78],
            Self::ComputersPeripheralEquipment => vec![5],
            Self::ConservationDevelopment => vec![16, 50, 70],
            Self::Construction => vec![13],
            Self::ConstructionMachinery => vec![29],
            Self::ConsumerDurableGoods => vec![1, 15],
            Self::Corporate => vec![2, 3, 17],
            Self::CreditIntermediationRelatedActivities => vec![64],
            Self::Custom => vec![80],
            Self::DepositoryreditIntermediation => vec![65],
            Self::DurableGoods => vec![15, 41],
            Self::Educational => vec![9, 44, 62],
            Self::EducationalVocational => vec![59],
            Self::EducationalBooks => vec![20],
            Self::EducationalServices => vec![84],
            Self::Electric => vec![51],
            Self::ElectricPower => vec![10],
            Self::ElectricalEquipmentAppliancesComponents => vec![22],
            Self::ElectricalEquipmentNec => vec![32],
            Self::ElectricalTransmissionDistributionIndustrialApparatus => vec![17],
            Self::Electronics => vec![28],
            Self::EnginesTurbines => vec![13],
            Self::EntertainmentLiteraryArtisticOriginals => vec![98],
            Self::Equipment => vec![
                1, 2, 5, 6, 10, 11, 14, 18, 22, 23, 26, 30, 34, 38, 39, 42, 46, 50, 54, 56, 58, 62,
                66, 70, 74, 76, 80, 84,
            ],
            Self::FabricatedMetalProducts => vec![12, 19],
            Self::Farm => vec![65],
            Self::Farms => vec![3, 5, 11, 21, 45],
            Self::Federal => vec![9, 21, 23, 86],
            Self::FederalReserveBanks => vec![63],
            Self::FinanceInsurance => vec![62],
            Self::Financial => vec![3, 33],
            Self::FixedAssets => vec![2],
            Self::FixedAssetsConsumerDurableGoods => vec![1],
            Self::FoodBeverageTobaccoProducts => vec![30],
            Self::FoodBeverageEstablishments => vec![45],
            Self::FoodBeverageStores => vec![45],
            Self::FoodManufacturing => vec![31],
            Self::FoodServicesDrinkingPlaces => vec![95],
            Self::ForestryFishingRelatedActivities => vec![4],
            Self::FundsTrustsOtherFinancialVehicles => vec![72],
            Self::FurnishingsDurableHouseholdEquipment => vec![6],
            Self::FurnitureFixtures => vec![27],
            Self::FurnitureFurnishings => vec![7],
            Self::FurnitureRelatedProducts => vec![25],
            Self::GeneralGovernmentFixedAssets => vec![75],
            Self::GeneralIndustrialIncludingMaterialsHandlingEquipment => vec![16],
            Self::GeneralMerchandiseStores => vec![46],
            Self::GlasswareTablewareHouseholdUtensils => vec![9],
            Self::Government => vec![8, 9],
            Self::GovernmentEnterpriseFixedAssets => vec![79],
            Self::GovernmentFixedAssets => vec![1, 22],
            Self::GovernmentNonresidentialFixedAssets => vec![83],
            Self::HealthSocialAssistance => vec![85],
            Self::HealthCare => vec![8, 39, 43, 61],
            Self::HighwaysStreets => vec![14, 49, 67],
            Self::Hospitals => vec![41, 87],
            Self::HospitalsSpecialCare => vec![40],
            Self::HouseholdAppliances => vec![8],
            Self::Households => vec![7, 9, 69],
            Self::HousingUnits => vec![68],
            Self::Improvements => vec![74],
            Self::Industrial => vec![5, 33],
            Self::IndustrialEquipment => vec![11],
            Self::Information => vec![57],
            Self::InformationDataProcessingServices => vec![61],
            Self::InformationProcessingEquipment => vec![4],
            Self::InsuranceAgenciesBrokersRelatedServices => vec![71],
            Self::InsuranceCarriers => vec![70],
            Self::InsuranceCarriersRelatedActivities => vec![69],
            Self::IntellectualPropertyProducts => {
                vec![
                    4, 7, 8, 12, 13, 16, 18, 20, 24, 28, 32, 35, 36, 40, 44, 48, 52, 56, 60, 64,
                    68, 72, 76, 78, 82, 90,
                ]
            }
            Self::JewelryWatches => vec![18],
            Self::Land => vec![64],
            Self::LegalServices => vec![77],
            Self::LightTrucks => vec![4],
            Self::LightTrucksIncludingUtilityVehicles => vec![20],
            Self::Lodging => vec![60],
            Self::LongLivedTelevisionPrograms => vec![100],
            Self::LuggageSimilarPersonalItems => vec![21],
            Self::Machinery => vec![20],
            Self::ManagementOfCompaniesEnterprises => vec![80],
            Self::ManufacturedHomes => vec![72],
            Self::Manufacturing => vec![9, 12, 14, 25, 48, 49, 84],
            Self::MedicalBuildings => vec![43],
            Self::MedicalEquipmentInstruments => vec![7],
            Self::MedicalEquipmentSupplies => vec![27],
            Self::MetalworkingMachinery => vec![14],
            Self::MilitaryFacilities => vec![15, 34],
            Self::Mining => vec![5, 56],
            Self::MiningOilfieldMachinery => vec![30],
            Self::MiningExplorationShaftsWells => vec![54],
            Self::MiningExceptOilGas => vec![7],
            Self::MiscellaneousManufacturing => vec![26],
            Self::MiscellaneousProfessionalScientificTechnicalServices => vec![79],
            Self::Missiles => vec![25],
            Self::MotionPictureSoundRecordingIndustries => vec![59],
            Self::MotorVehiclePartsDealers => vec![44],
            Self::MotorVehiclePartsAccessories => vec![5],
            Self::MotorVehiclesParts => vec![2],
            Self::MotorVehiclesBodiesTrailersParts => vec![23],
            Self::MotorVehiclesBodiesTrailersPartsManufacturing => vec![89],
            Self::MultimerchandiseShopping => vec![44],
            Self::Music => vec![102],
            Self::MusicalInstruments => vec![16],
            Self::NationalDefense => vec![22, 87],
            Self::NaturalGasDistribution => vec![11],
            Self::Noncorporate => vec![4, 5, 41],
            Self::Nondefense => vec![38, 88],
            Self::NondepositoryCreditIntermediation => vec![67],
            Self::NondurableGoods => vec![29, 42],
            Self::NonfarmNonmanufacturing => vec![13, 29, 53],
            Self::NonfarmNonmanufacturingNonresidentialFixedAssetsOnly => vec![13],
            Self::Nonfinancial => vec![4, 37],
            Self::Nonmanufacturing => vec![92],
            Self::NonmedicalInstruments => vec![8],
            Self::NonmetallicMineralProducts => vec![17],
            Self::NonprofitInstitutions => vec![6, 8, 65],
            Self::NonprofitInstitutionsServingHouseholdsNpishs => vec![95],
            Self::Nonresidential => vec![4, 10, 17],
            Self::NonresidentialEquipment => vec![3],
            Self::NonresidentialIntellectualPropertyProducts => vec![77],
            Self::NonresidentialStructures => vec![36],
            Self::NursingResidentialCareFacilities => vec![88],
            Self::Office => vec![6, 38, 41, 59],
            Self::OfficeAccountingEquipment => vec![10],
            Self::OilGasExtraction => vec![6],
            Self::Other => vec![66, 103],
            Self::OtherCommercial => vec![47],
            Self::OtherComputerElectronicProductManufacturing => vec![88],
            Self::OtherDurableGoods => vec![17],
            Self::OtherEquipment => vec![26, 29],
            Self::OtherIntermediation => vec![66],
            Self::OtherManufacturing => vec![28, 91],
            Self::OtherNonprofitInstitutions => vec![97],
            Self::OtherNonresidentialEquipment => vec![33],
            Self::OtherPower => vec![52],
            Self::OtherResidential => vec![75],
            Self::OtherRetail => vec![47],
            Self::OtherServicesExceptGovernment => vec![96],
            Self::OtherStructures => vec![17, 51, 57, 71],
            Self::OtherTransportationSupportActivities => vec![55],
            Self::OtherTransportationEquipment => vec![24],
            Self::OtherTrucksBusesTruckTrailers => vec![21],
            Self::OwnAccount => vec![81],
            Self::OwnerOccupied => vec![11],
            Self::PaperProducts => vec![35],
            Self::Partnerships => vec![7, 61],
            Self::PerformingArtsSpectatorSportsMuseumsRelatedActivities => vec![91],
            Self::PermanentSite => vec![69],
            Self::PetroleumCoalProducts => vec![37],
            Self::PetroleumNaturalGas => vec![55],
            Self::PharmaceuticalMedicineManufacturing => vec![85],
            Self::PhotocopyRelatedEquipment => vec![9],
            Self::PipelineTransportation => vec![54],
            Self::PlasticsRubberProducts => vec![39],
            Self::Power => vec![13, 48, 50, 66],
            Self::PowerCommunication => vec![49],
            Self::Prepackaged => vec![79],
            Self::PrimaryMetals => vec![18],
            Self::PrintingRelatedSupportActivities => vec![36],
            Self::Private => vec![2, 3],
            Self::PrivateGovernmentFixedAssets => vec![16],
            Self::PrivateFixedAssets => vec![1],
            Self::PrivateNonresidentialFixedAssets => vec![1],
            Self::ProfessionalScientificTechnicalServices => vec![76],
            Self::PublicSafety => vec![10, 45, 63],
            Self::PublishingIndustriesIncludesSoftware => vec![58],
            Self::RailroadEquipment => vec![25],
            Self::RailroadTransportation => vec![50],
            Self::RealEstate => vec![74],
            Self::RealEstateRentalLeasing => vec![73],
            Self::RecreationalBooks => vec![15],
            Self::RecreationalGoodsVehicles => vec![11],
            Self::Religious => vec![58],
            Self::RentalLeasingServicesLessorsOfIntangibleAssets => vec![75],
            Self::ResearchDevelopment => vec![20, 37, 54, 74, 82],
            Self::Residential => vec![4, 8, 14, 21, 32, 58],
            Self::ResidentialEquipment => vec![34],
            Self::ResidentialFixedAssets => vec![1],
            Self::ResidentialStructures => vec![67],
            Self::RetailTrade => vec![43],
            Self::ScientificResearchDevelopmentServices => vec![93],
            Self::SecuritiesCommodityContractsInvestments => vec![68],
            Self::SemiconductorOtherElectronicComponentManufacturing => vec![87],
            Self::ServiceIndustryMachinery => vec![31],
            Self::SewerSystems => vec![68],
            Self::Ships => vec![26],
            Self::ShipsBoats => vec![24],
            Self::SocialAssistance => vec![89],
            Self::Software => vec![19, 36, 53, 73, 78],
            Self::SoleProprietorships => vec![6, 57],
            Self::SoleProprietorshipsPartnerships => vec![5],
            Self::SpecialCare => vec![42],
            Self::SpecialIndustryMachineryNec => vec![15],
            Self::SportingEquipmentSuppliesGunsAmmunition => vec![13],
            Self::SportsRecreationalVehicles => vec![14],
            Self::StateLocal => vec![10, 24, 55, 89],
            Self::Structures => vec![
                3, 6, 7, 11, 12, 15, 19, 23, 27, 30, 31, 35, 39, 40, 43, 47, 51, 55, 57, 59, 63,
                67, 71, 75, 77, 81, 85,
            ],
            Self::SupportActivitiesForMining => vec![8],
            Self::TaxExemptCooperatives => vec![10, 73],
            Self::TelephoneFacsimileEquipment => vec![22],
            Self::TenantOccupied => vec![12],
            Self::TextileMillsTextileProductMills => vec![33],
            Self::TheatricalMovies => vec![99],
            Self::TherapeuticAppliancesEquipment => vec![19],
            Self::ToolsEquipmentForHouseGarden => vec![10],
            Self::TransitGroundPassengerTransportation => vec![53],
            Self::Transportation => vec![12, 47, 65],
            Self::TransportationWarehousing => vec![48],
            Self::TransportationEquipment => vec![18],
            Self::Transportion => vec![62],
            Self::TruckTransportation => vec![52],
            Self::TrucksBusesTruckTrailers => vec![19],
            Self::UniversitiesColleges => vec![96],
            Self::Utilities => vec![9],
            Self::Vehicles => vec![27],
            Self::VideoAudioPhotographicInformationProcessingEquipmentMedia => vec![12],
            Self::Warehouses => vec![46],
            Self::WarehousingStorage => vec![56],
            Self::WasteManagementRemediationServices => vec![83],
            Self::WaterSystems => vec![69],
            Self::WaterTransportation => vec![51],
            Self::WaterSewageOtherSystems => vec![12],
            Self::WholesaleTrade => vec![40],
            Self::WoodProducts => vec![16],
        }
    }
}
