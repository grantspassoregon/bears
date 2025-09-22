use std::str::FromStr;

use crate::{
    AffiliateLevel, BeaErr, BeaResponse, Classification, Dataset, DeriveFromStr, DirectionKind,
    DirectionOfInvestment, Footnotes, Integer, IntegerOptions, IoError, OwnershipLevel,
    ParameterName, ParameterValueTable, ParameterValueTableVariant, SerdeJson, Set, State,
    YearOptions,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
)]
pub struct Mne {
    classification: Vec<Classification>,
    country: Vec<IntegerOptions>,
    direction_of_investment: Vec<DirectionOfInvestment>,
    get_footnotes: Vec<Footnotes>,
    industry: Vec<IntegerOptions>,
    investment: Vec<IntegerOptions>,
    nonbank_affiliates_only: Vec<AffiliateLevel>,
    ownership_level: Vec<OwnershipLevel>,
    parent_investment: Vec<IntegerOptions>,
    series_id: Vec<Integer>,
    state: Vec<State>,
    year: Vec<YearOptions>,
}

impl Mne {
    #[tracing::instrument(skip_all)]
    pub fn iter_mne(&self) -> MneIter<'_> {
        MneIter::new(self)
    }

    #[tracing::instrument(skip_all)]
    pub fn iter_amne(&self) -> AmneIter<'_> {
        AmneIter::new(self)
    }

    #[tracing::instrument(skip_all)]
    pub fn filter_country(&mut self, value: &str) {
        self.country.retain(|v| v.key() != value);
    }

    // pub fn queue() -> Result<Queue, BeaErr> {
    //     let req = Request::Data;
    //     let mut app = req.init()?;
    //     let dataset = Dataset::Mne;
    //     app.with_dataset(dataset);
    //     dotenvy::dotenv().ok();
    //     let path = bea_data()?;
    //     let data = Mne::try_from(&path)?;
    //     let mut queue = Vec::new();
    //     for params in data.iter() {
    //         tracing::trace!("{params:#?}");
    //         app.with_params(params.clone());
    //         queue.push(app.clone());
    //     }
    //     Ok(Queue::new(queue))
    // }
}

impl TryFrom<&std::path::PathBuf> for Mne {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::Mne;
        let names = dataset.names();
        // empty vectors to store values
        let mut classification = Vec::new();
        let mut country = Vec::new();
        let mut direction_of_investment = Vec::new();
        let mut get_footnotes = Vec::new();
        let mut industry = Vec::new();
        let mut investment = Vec::new();
        let mut nonbank_affiliates_only = Vec::new();
        let mut ownership_level = Vec::new();
        let mut parent_investment = Vec::new();
        let mut series_id = Vec::new();
        let mut state = Vec::new();
        let mut year = Vec::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!(
                "parameter_values/{dataset}_{name}_parameter_values.json"
            ));
            let file = std::fs::File::open(&path)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                // type of vector varies by parameter name
                match name {
                    ParameterName::Classification => {
                        for table in pv.iter() {
                            match table {
                                ParameterValueTable::MneDoi(tab) => {
                                    let value =
                                        Classification::from_str(tab.key()).map_err(|e| {
                                            DeriveFromStr::new(
                                                tab.key().to_owned(),
                                                e,
                                                line!(),
                                                file!().to_owned(),
                                            )
                                        })?;
                                    classification.push(value);
                                }
                                other => {
                                    let error = ParameterValueTableVariant::new(
                                        format!("MneDoi needed, found {other:#?}"),
                                        line!(),
                                        file!().to_string(),
                                    );
                                    return Err(error.into());
                                }
                            }
                        }
                    }
                    ParameterName::Country => {
                        for table in pv.iter() {
                            country.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::DirectionOfInvestment => {
                        for table in pv.iter() {
                            direction_of_investment.push(DirectionOfInvestment::try_from(table)?);
                        }
                    }
                    ParameterName::GetFootnotes => {
                        for table in pv.iter() {
                            get_footnotes.push(Footnotes::try_from(table)?);
                        }
                    }
                    ParameterName::Industry => {
                        for table in pv.iter() {
                            industry.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::Investment => {
                        for table in pv.iter() {
                            investment.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::NonbankAffiliatesOnly => {
                        for table in pv.iter() {
                            nonbank_affiliates_only.push(AffiliateLevel::try_from(table)?);
                        }
                    }
                    ParameterName::OwnershipLevel => {
                        for table in pv.iter() {
                            ownership_level.push(OwnershipLevel::try_from(table)?);
                        }
                    }
                    ParameterName::ParentInvestment => {
                        for table in pv.iter() {
                            parent_investment.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::SeriesID => {
                        for table in pv.iter() {
                            series_id.push(Integer::try_from(table)?);
                        }
                    }
                    ParameterName::State => {
                        tracing::trace!("Building State values.");
                        for table in pv.iter() {
                            state.push(State::try_from(table)?);
                        }
                    }
                    ParameterName::Year => {
                        tracing::trace!("Building Year values.");
                        for table in pv.iter() {
                            year.push(YearOptions::try_from(table)?);
                        }
                    }
                    other => return Err(Set::ParameterNameMissing(other.to_string()).into()),
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if classification.is_empty()
            || country.is_empty()
            || direction_of_investment.is_empty()
            || get_footnotes.is_empty()
            || industry.is_empty()
            || investment.is_empty()
            || nonbank_affiliates_only.is_empty()
            || ownership_level.is_empty()
            || parent_investment.is_empty()
            || series_id.is_empty()
            || state.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self {
                classification,
                country,
                direction_of_investment,
                get_footnotes,
                industry,
                investment,
                nonbank_affiliates_only,
                ownership_level,
                parent_investment,
                series_id,
                state,
                year,
            };
            Ok(table)
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
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum MneKind {
    #[default]
    Amne,
    Di,
}

/// Returns an iterator over the `Mne` struct for Direct Investment data.
/// Used to create API calls with SeriesID, State and Years set to "ALL".
/// Includes footnotes.
#[derive(Debug, Clone)]
pub struct MneIter<'a> {
    // Set of classification values to request
    classifications: std::slice::Iter<'a, Classification>,
    // Set of country values to request
    countries: std::slice::Iter<'a, IntegerOptions>,
    // Set of variants associated with requested MneKind
    investment_direction: Vec<DirectionKind>,
    // cached value of current country
    current_country: Option<&'a IntegerOptions>,
    // index of position in *investment_direction*
    current_direction: usize,
    // cache clones of *classifications* to consume on loops
    class_cache: std::slice::Iter<'a, Classification>,
}

impl<'a> MneIter<'a> {
    /// Creates an iterator over investment directions in the provided `Mne` struct.
    #[tracing::instrument(skip_all)]
    pub fn new(data: &'a Mne) -> Self {
        let classifications = data.classification().iter();
        let mut countries = data.country().iter();
        let investment_direction = DirectionKind::mne(MneKind::Di);
        let class_cache = classifications.clone();
        let current_country = countries.next();
        let current_direction = 0;
        Self {
            classifications,
            countries,
            investment_direction,
            current_country,
            current_direction,
            class_cache,
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn reset_class(&mut self) -> Option<&Classification> {
        self.class_cache = self.classifications.clone();
        self.class_cache.next()
    }

    #[tracing::instrument(skip_all)]
    pub fn reset_direction(&mut self) -> Option<&Classification> {
        self.current_direction = 0;
        self.reset_class()
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_class(&mut self) -> Option<&Classification> {
        match self.class_cache.next() {
            Some(class) => Some(class),
            None => self.advance_direction(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_direction(&mut self) -> Option<&Classification> {
        if self.current_direction < self.investment_direction.len() - 1 {
            self.current_direction += 1;
            self.reset_class()
        } else {
            self.advance_country()
        }
    }

    pub fn advance_country(&mut self) -> Option<&Classification> {
        match self.countries.next() {
            Some(country) => {
                self.current_country = Some(country);
                self.reset_direction()
            }
            None => None,
        }
    }
}

impl Iterator for MneIter<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // advance state
        // set classification
        let classification = self.advance_class()?;
        let (key, value) = classification.params();
        params.insert(key, value);

        // set direction of investment
        let key = ParameterName::DirectionOfInvestment.to_string();
        let value = self.investment_direction[self.current_direction]
            .key()
            .to_owned();
        params.insert(key, value);

        // set country
        if let Some(country) = self.current_country {
            let key = ParameterName::Country.to_string();
            let value = country.key().to_owned();
            params.insert(key, value);
        }

        // set footnotes
        let key = ParameterName::GetFootnotes.to_string();
        let value = Footnotes::Yes.key().to_owned();
        params.insert(key, value);

        // set seried id
        let key = ParameterName::SeriesID.to_string();
        params.insert(key, "ALL".to_string());

        // set industry
        let key = ParameterName::Industry.to_string();
        params.insert(key, "ALL".to_string());

        // set state
        let key = ParameterName::State.to_string();
        params.insert(key, "ALL".to_string());

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        Some(params)
    }
}

/// Returns an iterator over the `Mne` struct for the AMNE dataset.
/// Used to create API calls with SeriesID, State and Years set to "ALL".
/// Includes footnotes.
#[derive(Debug, Clone)]
pub struct AmneIter<'a> {
    // Set of classification values to request
    classifications: std::slice::Iter<'a, Classification>,
    // Set of country values to request
    countries: std::slice::Iter<'a, IntegerOptions>,
    // Set of variants associated with requested MneKind
    investment_direction: Vec<DirectionKind>,
    // Set of ownership levels to request
    ownership_levels: std::slice::Iter<'a, OwnershipLevel>,
    // Set of nonbank affiliates only values to request
    nonbank_affiliates: std::slice::Iter<'a, AffiliateLevel>,
    // cached value of current country
    current_country: Option<&'a IntegerOptions>,
    // index of position in *investment_direction*
    current_direction: usize,
    // cache clones of *nonbank_affiliates* to consume on loops
    affiliate_cache: std::slice::Iter<'a, AffiliateLevel>,
    // cache clones of *classifications* to consume on loops
    class_cache: std::slice::Iter<'a, Classification>,
    // cache clones of *ownership_levels* to consume on loops
    owner_cache: std::slice::Iter<'a, OwnershipLevel>,
    // records current owner during state transitions
    current_owner: Option<&'a OwnershipLevel>,
    // records current affiliate level during state transitions
    current_affiliate: Option<&'a AffiliateLevel>,
}

impl<'a> AmneIter<'a> {
    /// Creates an iterator over investment directions in the provided `Mne` struct.
    #[tracing::instrument(skip_all)]
    pub fn new(data: &'a Mne) -> Self {
        let classifications = data.classification().iter();
        let mut countries = data.country().iter();
        let investment_direction = DirectionKind::mne(MneKind::Amne);
        let ownership_levels = data.ownership_level().iter();
        let nonbank_affiliates = data.nonbank_affiliates_only().iter();
        let mut affiliate_cache = nonbank_affiliates.clone();
        let class_cache = classifications.clone();
        let mut owner_cache = ownership_levels.clone();
        let current_country = countries.next();
        let current_direction = 0;
        let current_owner = owner_cache.next();
        let current_affiliate = affiliate_cache.next();
        Self {
            classifications,
            countries,
            investment_direction,
            ownership_levels,
            nonbank_affiliates,
            current_country,
            current_direction,
            affiliate_cache,
            class_cache,
            owner_cache,
            current_owner,
            current_affiliate,
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn reset_class(&mut self) -> Option<&Classification> {
        self.class_cache = self.classifications.clone();
        self.class_cache.next()
    }

    #[tracing::instrument(skip_all)]
    pub fn reset_direction(&mut self) -> Option<&Classification> {
        self.current_direction = 0;
        self.reset_class()
    }

    #[tracing::instrument(skip_all)]
    pub fn reset_owner(&mut self) -> Option<&Classification> {
        self.owner_cache = self.ownership_levels.clone();
        self.current_owner = self.owner_cache.next();
        self.reset_direction()
    }
    #[tracing::instrument(skip_all)]
    pub fn reset_affiliate(&mut self) -> Option<&Classification> {
        self.affiliate_cache = self.nonbank_affiliates.clone();
        self.current_affiliate = self.affiliate_cache.next();
        self.reset_owner()
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_class(&mut self) -> Option<&Classification> {
        match self.class_cache.next() {
            Some(class) => Some(class),
            None => self.advance_direction(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_direction(&mut self) -> Option<&Classification> {
        if self.current_direction < self.investment_direction.len() - 1 {
            self.current_direction += 1;
            self.reset_class()
        } else {
            self.advance_ownership()
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_ownership(&mut self) -> Option<&Classification> {
        match self.owner_cache.next() {
            Some(owner) => {
                self.current_owner = Some(owner);
                self.reset_direction()
            }
            None => self.advance_affiliates(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_affiliates(&mut self) -> Option<&Classification> {
        match self.nonbank_affiliates.next() {
            Some(affilitate) => {
                self.current_affiliate = Some(affilitate);
                self.reset_owner()
            }
            None => self.advance_country(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_country(&mut self) -> Option<&Classification> {
        match self.countries.next() {
            Some(country) => {
                self.current_country = Some(country);
                self.reset_affiliate()
            }
            None => None,
        }
    }
}

impl Iterator for AmneIter<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // advance state
        // set classification
        let classification = self.advance_class()?;
        let (key, value) = classification.params();
        params.insert(key, value);

        // set direction of investment
        let key = ParameterName::DirectionOfInvestment.to_string();
        let value = self.investment_direction[self.current_direction]
            .key()
            .to_owned();
        params.insert(key, value);

        // set ownership level
        if let Some(owner) = self.current_owner {
            let (key, value) = owner.params();
            params.insert(key, value);
        }

        // set nonbank affiliates
        if let Some(affiliate) = self.current_affiliate {
            let (key, value) = affiliate.params();
            params.insert(key, value);
        }

        // set country
        if let Some(country) = self.current_country {
            let key = ParameterName::Country.to_string();
            let value = country.key().to_owned();
            params.insert(key, value);
        }

        // set footnotes
        let key = ParameterName::GetFootnotes.to_string();
        let value = Footnotes::Yes.key().to_owned();
        params.insert(key, value);

        // set seried id
        let key = ParameterName::SeriesID.to_string();
        params.insert(key, "ALL".to_string());

        // set industry
        let key = ParameterName::Industry.to_string();
        params.insert(key, "ALL".to_string());

        // set state
        let key = ParameterName::State.to_string();
        params.insert(key, "ALL".to_string());

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        Some(params)
    }
}
