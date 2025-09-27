use bears_species::{
    ApiMetadata, BeaErr, Dataset, FixedAssets, GdpByIndustry, Iip, InputOutput, IntlServSta,
    IntlServTrade, Ita, Mne, NiUnderlyingDetail, Nipa, Regional,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_more::From)]
pub enum ValueSet {
    #[from(ApiMetadata)]
    APIDatasetMetadata(ApiMetadata),
    #[from(FixedAssets)]
    FixedAssets(FixedAssets),
    #[from(Iip)]
    Iip(Iip),
    #[from(InputOutput)]
    InputOutput(InputOutput),
    #[from(IntlServTrade)]
    IntlServTrade(IntlServTrade),
    #[from(IntlServSta)]
    IntlServSTA(IntlServSta),
    #[from(Ita)]
    Ita(Ita),
    #[from(GdpByIndustry)]
    GDPbyIndustry(GdpByIndustry),
    #[from(Mne)]
    Mne(Mne),
    #[from(Nipa)]
    Nipa(Nipa),
    #[from(NiUnderlyingDetail)]
    NIUnderlyingDetail(NiUnderlyingDetail),
    #[from(Regional)]
    Regional(Regional),
    UnderlyingGDPbyIndustry(GdpByIndustry),
}

impl ValueSet {
    pub fn from_path(path: &std::path::PathBuf, dataset: Dataset) -> Result<Self, BeaErr> {
        match dataset {
            Dataset::APIDatasetMetadata => {
                let set = ApiMetadata::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Metadata values.", set.len());
                Ok(set.into())
            }
            Dataset::FixedAssets => {
                let set = FixedAssets::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
                Ok(set.into())
            }
            Dataset::GDPbyIndustry | Dataset::UnderlyingGDPbyIndustry => {
                let set = GdpByIndustry::from_file(path, dataset)?;
                tracing::trace!("{dataset} values read.");
                Ok(set.into())
            }
            Dataset::Iip => {
                let set = Iip::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Component values.", set.component().len());
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!(
                    "{dataset} has {} TypeOfInvestment values.",
                    set.type_of_investment().len()
                );
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::InputOutput => {
                let set = InputOutput::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} TableID values.", set.table_id().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::Ita => {
                let set = Ita::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} AreaOrCounty values.",
                    set.area_or_country().len()
                );
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!("{dataset} has {} Indicator values.", set.indicator().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::IntlServSTA => {
                let set = IntlServSta::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} AreaOrCounty values.",
                    set.area_or_country().len()
                );
                tracing::trace!("{dataset} has {} Channel values.", set.channel().len());
                tracing::trace!(
                    "{dataset} has {} Destination values.",
                    set.destination().len()
                );
                tracing::trace!("{dataset} has {} Industry values.", set.industry().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::IntlServTrade => {
                let set = IntlServTrade::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} Affiliation values.",
                    set.affiliation().len()
                );
                tracing::trace!(
                    "{dataset} has {} AreaOrCounty values.",
                    set.area_or_country().len()
                );
                tracing::trace!(
                    "{dataset} has {} TradeDirection values.",
                    set.trade_direction().len()
                );
                tracing::trace!(
                    "{dataset} has {} TypeOfService values.",
                    set.type_of_service().len()
                );
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::Nipa => {
                let set = Nipa::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!(
                    "{dataset} has {} ShowMillions values.",
                    set.show_millions().len()
                );
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
                Ok(set.into())
            }
            Dataset::Mne => {
                let set = Mne::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} Classification values.",
                    set.classification().len()
                );
                tracing::trace!("{dataset} has {} Country values.", set.country().len());
                tracing::trace!(
                    "{dataset} has {} DirectionOfInvestment values.",
                    set.direction_of_investment().len()
                );
                tracing::trace!(
                    "{dataset} has {} GetFootnotes values.",
                    set.get_footnotes().len()
                );
                tracing::trace!("{dataset} has {} Industry values.", set.industry().len());
                tracing::trace!(
                    "{dataset} has {} Investment values.",
                    set.investment().len()
                );
                tracing::trace!(
                    "{dataset} has {} NonbankAffiliatesOnly values.",
                    set.nonbank_affiliates_only().len()
                );
                tracing::trace!(
                    "{dataset} has {} OwnershipLevel values.",
                    set.ownership_level().len()
                );
                tracing::trace!(
                    "{dataset} has {} ParentInvestment values.",
                    set.parent_investment().len()
                );
                tracing::trace!("{dataset} has {} YearOptions values.", set.year().len());
                Ok(set.into())
            }
            Dataset::NIUnderlyingDetail => {
                let set = NiUnderlyingDetail::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
                Ok(set.into())
            }
            Dataset::Regional => {
                let set = Regional::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} GeoFips values.", set.geo_fips().len());
                tracing::trace!("{dataset} has {} LineCode values.", set.line_code().len());
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct ValueSets(Vec<ValueSet>);

impl ValueSets {
    pub fn from_path(path: &std::path::PathBuf, datasets: &[Dataset]) -> Result<Self, BeaErr> {
        let mut sets = Vec::new();
        for dataset in datasets {
            let set = ValueSet::from_path(path, *dataset)?;
            sets.push(set);
        }
        Ok(Self(sets))
    }
}
