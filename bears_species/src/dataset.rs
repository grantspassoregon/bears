use crate::{BeaErr, KeyMissing, NotObject, ParameterName, map_to_string};
use convert_case::Casing;
use serde::{Deserialize, Serialize};

/// The `Dataset` enum contains variants for each dataset published by the BEA.
///
/// The enum should technically be marked as exhaustive, but the developer needs to lean on Rust's
/// exhaustive enum matching as a crutch.
///
/// We match the variants against the response from the
/// [`Method::GetDataSetList`](crate::Method::GetDataSetList) in a unit test to detect new
/// additions.
///
/// The `Dataset` type contains convenience methods for downloading the valid parameter values
/// associates with the different parameters of each dataset. For datasets with a limited range of
/// potential values, there is significant overlap of coverage between the
/// [`Dataset::parameter_values`] and [`Dataset::values`] methods.
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
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum Dataset {
    #[default]
    #[display("NIPA")]
    Nipa,
    NIUnderlyingDetail,
    #[display("MNE")]
    Mne,
    FixedAssets,
    #[display("ITA")]
    Ita,
    #[display("IIP")]
    Iip,
    InputOutput,
    IntlServTrade,
    IntlServSTA,
    GDPbyIndustry,
    Regional,
    UnderlyingGDPbyIndustry,
    APIDatasetMetadata,
}

impl Dataset {
    /// The `lower` method converts the variant name to `lowercase` case using
    /// [`convert_case::Case::Flat`].
    #[tracing::instrument]
    pub fn lower(&self) -> String {
        self.to_string().to_case(convert_case::Case::Flat)
    }

    /// The `names` method returns the vector of valid [`ParameterName`] inputs for a given
    /// `Dataset`.
    ///
    /// We match values for each variant manually against the responses from the
    /// [`Method::GetParameterList`](crate::Method::GetParameterList) call for each [`Dataset`]
    /// variant.
    ///
    /// TODO: Match the output against the responses in a unit test to detect changes or additions.
    pub fn names(&self) -> Vec<ParameterName> {
        match self {
            Self::Nipa => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::ShowMillions,
                    ParameterName::TableID,
                    ParameterName::TableName,
                    ParameterName::Year,
                ]
            }
            Self::NIUnderlyingDetail => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::TableID,
                    ParameterName::TableName,
                    ParameterName::Year,
                ]
            }
            Self::Mne => {
                vec![
                    ParameterName::DirectionOfInvestment,
                    ParameterName::OwnershipLevel,
                    ParameterName::NonbankAffiliatesOnly,
                    ParameterName::Classification,
                    ParameterName::Country,
                    ParameterName::Industry,
                    ParameterName::Year,
                    ParameterName::State,
                    ParameterName::SeriesID,
                    ParameterName::GetFootnotes,
                    ParameterName::Investment,
                    ParameterName::ParentInvestment,
                ]
            }
            Self::FixedAssets => {
                vec![ParameterName::TableName, ParameterName::Year]
            }
            Self::Ita => {
                vec![
                    ParameterName::Indicator,
                    ParameterName::AreaOrCountry,
                    ParameterName::Frequency,
                    ParameterName::Year,
                ]
            }
            Self::Iip => {
                vec![
                    ParameterName::TypeOfInvestment,
                    ParameterName::Component,
                    ParameterName::Frequency,
                    ParameterName::Year,
                ]
            }
            Self::InputOutput => {
                vec![ParameterName::TableID, ParameterName::Year]
            }
            Self::IntlServTrade => {
                vec![
                    ParameterName::TypeOfService,
                    ParameterName::TradeDirection,
                    ParameterName::Affiliation,
                    ParameterName::AreaOrCountry,
                    ParameterName::Year,
                ]
            }
            Self::IntlServSTA => {
                vec![
                    ParameterName::Channel,
                    ParameterName::Destination,
                    ParameterName::Industry,
                    ParameterName::AreaOrCountry,
                    ParameterName::Year,
                ]
            }
            Self::GDPbyIndustry => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::Industry,
                    ParameterName::TableID,
                    ParameterName::Year,
                ]
            }
            Self::Regional => {
                vec![
                    ParameterName::GeoFips,
                    ParameterName::LineCode,
                    ParameterName::TableName,
                    ParameterName::Year,
                ]
            }
            Self::UnderlyingGDPbyIndustry => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::Industry,
                    ParameterName::TableID,
                    ParameterName::Year,
                ]
            }
            Self::APIDatasetMetadata => {
                vec![ParameterName::Dataset]
            }
        }
    }

    ///// Download the BEA dataset parameter values into the `BEA_DATA` directory.
    //pub async fn get() -> Result<(), BeaErr> {
    //    let req = Request::Dataset;
    //    let app = req.init()?;
    //    let data = app.get().await?;
    //    match data.json::<serde_json::Value>().await {
    //        Ok(json) => {
    //            let contents = serde_json::to_vec(&json)
    //                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    //            dotenvy::dotenv().ok();
    //            let path = bea_data()?;
    //            let path = path.join("datasets.json");
    //            std::fs::write(&path, contents)
    //                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
    //        }
    //        Err(source) => {
    //            let url = app.url().to_string();
    //            let method = "get".to_string();
    //            let error = ReqwestError::new(url, method, source, line!(), file!().to_string());
    //            Err(error.into())
    //        }
    //    }
    //}
    //
    ///// The `load` method reads a [`BeaResponse`] from the `BEA_DATA` directory.
    ///// TODO: Unused.  Keep or delete?
    //#[tracing::instrument]
    //pub fn load() -> Result<BeaResponse, BeaErr> {
    //    dotenvy::dotenv().ok();
    //    let path = bea_data()?;
    //    let path = path.join("datasets.json");
    //    let file = std::fs::File::open(&path)
    //        .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    //    tracing::info!("File opened.");
    //    let rdr = std::io::BufReader::new(file);
    //    let json: serde_json::Value = serde_json::from_reader(rdr)
    //        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    //    let bea = BeaResponse::try_from(&json)?;
    //    tracing::info!("Response: {bea:#?}");
    //    Ok(bea)
    //}
    //
    //#[tracing::instrument(skip_all)]
    //pub async fn initial_download(&self) -> Result<(), BeaErr> {
    //    let queue = self.queue()?;
    //    tracing::info!("Queue length: {}", queue.len());
    //    queue.download(false).await?;
    //    Ok(())
    //}
    //
    //#[tracing::instrument(skip_all)]
    //pub async fn download_with_history(
    //    &self,
    //    style: indicatif::ProgressStyle,
    //) -> Result<(), BeaErr> {
    //    let queue = self.queue()?;
    //    tracing::info!("Queue length: {}", queue.len());
    //
    //    // get the download history for the size hints
    //    let history = History::try_from((*self, Mode::Load))?;
    //    history.summary();
    //    history.iter().download(&queue, false, style).await?;
    //    Ok(())
    //}
    //
    ///// Load all successfully downloaded files in the download [`History`] for the `Dataset`.
    ///// If the user provides a `load_history`, the method will exclude previously loaded files in
    ///// the provided [`History`].
    //#[tracing::instrument(skip_all)]
    //pub fn initial_load_par(&self, load_history: Option<&History>) -> Result<Vec<Data>, BeaErr> {
    //    let mut queue = self.queue()?;
    //    tracing::info!("Queue length: {}", queue.len());
    //
    //    // A fresh queue has been downloaded, try loading the successes
    //    let downloads = History::try_from((*self, Mode::Download))?;
    //    // only download successes in history
    //    // strict = true set to include no others in queue.
    //    queue.successes(&downloads, true)?;
    //    tracing::info!("Files downloaded: {}", queue.len());
    //
    //    if let Some(loads) = load_history {
    //        // exclude previously loaded files in load history
    //        queue.exclude(loads)?;
    //        tracing::info!("Files left to load: {}", queue.len());
    //    }
    //
    //    let data = queue.load_par()?;
    //    tracing::info!("{} datasets loaded.", data.len());
    //    Ok(data)
    //}
    //
    ///// Load all successfully downloaded files in the download [`History`] for the `Dataset`.
    ///// If the user provides a `load_history`, the method will exclude previously loaded files in
    ///// the provided [`History`].
    //#[tracing::instrument(skip_all)]
    //pub async fn initial_load(&self, load_history: Option<&History>) -> Result<Vec<Data>, BeaErr> {
    //    let mut queue = self.queue()?;
    //    tracing::info!("Queue length: {}", queue.len());
    //
    //    // A fresh queue has been downloaded, try loading the successes
    //    let downloads = History::try_from((*self, Mode::Download))?;
    //    // only download successes in history
    //    // strict = true set to include no others in queue.
    //    queue.successes(&downloads, true)?;
    //    tracing::info!("Files downloaded: {}", queue.len());
    //
    //    if let Some(loads) = load_history {
    //        // exclude previously loaded files in load history
    //        queue.exclude(loads)?;
    //        tracing::info!("Files left to load: {}", queue.len());
    //    }
    //
    //    let data = queue.load().await?;
    //    let data = data.lock().await;
    //    tracing::info!("{} datasets loaded.", data.len());
    //    Ok(data.to_vec())
    //}
    //
    ///// Tries to load any files in the history that previously failed to load.
    //#[tracing::instrument(skip_all)]
    //pub async fn retry_load(&self) -> Result<Vec<Data>, BeaErr> {
    //    let mut queue = self.queue()?;
    //    tracing::info!("Queue length: {}", queue.len());
    //
    //    // The load history contains errors, try them again.
    //    let history = History::try_from((*self, Mode::Load))?;
    //    // strict is true means only download errors included in the event history
    //    queue.errors(&history, true)?;
    //    tracing::info!("Files to retry: {}", queue.len());
    //
    //    let data = queue.load().await?;
    //    let data = data.lock().await;
    //    tracing::info!("{} datasets loaded.", data.len());
    //    Ok(data.to_vec())
    //}
    //
    ///// The `parameter` method reads a [`BeaResponse`] to json using the `serde_json` crate.
    ///// Saves the result to the `BEA_DATA` directory.
    /////
    ///// Called by [`Self::parameters`].
    //#[tracing::instrument(skip_all)]
    //pub async fn parameter(&self, app: &mut App) -> Result<(), BeaErr> {
    //    app.with_dataset(*self);
    //    let data = app.get().await?;
    //    match data.json::<serde_json::Value>().await {
    //        Ok(json) => {
    //            let contents = serde_json::to_vec(&json)
    //                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    //            dotenvy::dotenv().ok();
    //            let path = bea_data()?;
    //            let path = path.join("parameters");
    //            if !path.exists() {
    //                std::fs::DirBuilder::new()
    //                    .create(&path)
    //                    .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
    //                tracing::info!("Target directory for Parameters created.");
    //            }
    //            let path = path.join(format!("{self}_parameters.json"));
    //            std::fs::write(&path, contents)
    //                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
    //        }
    //        Err(source) => {
    //            let url = app.url().to_string();
    //            let method = "get".to_string();
    //            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
    //            let mut error =
    //                ReqwestError::new(url, method, source, line!(), file!().to_string());
    //            error.with_body(body);
    //            Err(error.into())
    //        }
    //    }
    //}
    //
    ///// For each variant of [`Dataset`], request the parameters.
    ///// Write the results in JSON format to the BEA_DATA directory.
    //#[tracing::instrument]
    //pub async fn parameters() -> Result<(), BeaErr> {
    //    let req = Request::Parameter;
    //    let mut app = req.init()?;
    //    let datasets: Vec<Self> = Self::iter().collect();
    //    for dataset in datasets {
    //        dataset.parameter(&mut app).await?;
    //    }
    //    Ok(())
    //}
    //
    ///// The `parameter_value` method requests the valid values for parameter `name` of the
    ///// `Dataset`.
    /////
    ///// Called by [`Self::parameter_values`].
    //#[tracing::instrument(skip_all)]
    //pub async fn parameter_value(self, app: &mut App, name: ParameterName) -> Result<(), BeaErr> {
    //    let mut opts = app.options().clone();
    //    opts.with_dataset(self);
    //    opts.with_param_name(name);
    //    app.with_options(opts);
    //    let data = app.get().await?;
    //    match data.json::<serde_json::Value>().await {
    //        Ok(json) => {
    //            let contents = serde_json::to_vec(&json)
    //                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    //            dotenvy::dotenv().ok();
    //            let path = bea_data()?;
    //            let path = path.join("parameter_values");
    //            if !path.exists() {
    //                std::fs::DirBuilder::new()
    //                    .create(&path)
    //                    .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
    //                tracing::info!("Target directory for Parameter Values created.");
    //            }
    //            let path = path.join(format!("{self}_{name}_parameter_values.json"));
    //            std::fs::write(&path, contents)
    //                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
    //        }
    //        Err(source) => {
    //            let url = app.url().to_string();
    //            let method = "get".to_string();
    //            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
    //            let mut error =
    //                ReqwestError::new(url, method, source, line!(), file!().to_string());
    //            error.with_body(body);
    //            Err(error.into())
    //        }
    //    }
    //}
    //
    ///// The `parameter_values` method requests the valid range of values for each parameter in the
    ///// [`Dataset`].
    /////
    ///// After a successfull response from an API request, the goal is to parse the response into
    ///// internal library data structures.  The JSON responses can include heavily nested data
    ///// structures, which makes deserializing directly into Rust types a brittle process.  Instead, we
    ///// first we deserialize the JSON into serde_json types, and then migrate the results into our
    ///// internal library types using the [`TryFrom`] trait.  While this is a bit heavier on
    ///// boilerplate, the errors and logs are easier to consume, providing a clearing path to a correct
    ///// implementation result during the development process.
    /////
    ///// Here we request a parameter values table from the server, parse it into serde_json types, and
    ///// write the results to the `BEA_DATA` directory.  Later we can attempt to parse the response
    ///// multiple times into our internal library types, succussfully or unsuccessfully, without making
    ///// repeated API calls to BEA for the same data.
    //#[tracing::instrument]
    //pub async fn parameter_values() -> Result<(), BeaErr> {
    //    let req = Request::ParameterValue;
    //    let mut app = req.init()?;
    //    let datasets: Vec<Self> = Self::iter().collect();
    //    for dataset in datasets {
    //        let names = dataset.names();
    //        for name in names {
    //            dataset.parameter_value(&mut app, name).await?;
    //        }
    //    }
    //    Ok(())
    //}
    //
    ///// The `value` method downloads the valid values for parameter `name` in the `Dataset`.
    /////
    ///// Called by [`Self::values`].
    //#[tracing::instrument(skip(self, app))]
    //pub async fn value(self, app: &mut App, name: ParameterName) -> Result<(), BeaErr> {
    //    let mut options = app.options().clone();
    //    options.with_dataset(self);
    //    options.with_target(name);
    //    app.with_options(options.clone());
    //    let data = app.get().await?;
    //    tracing::info!("{data:#?}");
    //    match data.json::<serde_json::Value>().await {
    //        Ok(json) => {
    //            tracing::info!("{json:#?}");
    //            let bea = BeaResponse::try_from(&json)?;
    //            match bea.results() {
    //                Results::ApiError(e) => {
    //                    // TODO: Map api error codes to an enum.
    //                    tracing::info!("{e}");
    //                }
    //                Results::ParameterValues(_) => {
    //                    let contents = serde_json::to_vec(&json)
    //                        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    //                    dotenvy::dotenv().ok();
    //                    let path = bea_data()?;
    //                    let path = path.join("parameter_values");
    //                    if !path.exists() {
    //                        std::fs::DirBuilder::new().create(&path).map_err(|e| {
    //                            IoError::new(path.clone(), e, line!(), file!().into())
    //                        })?;
    //                        tracing::info!("Target directory for Parameter Values created.");
    //                    }
    //                    let path = path.join(format!("{self}_{name}_values.json"));
    //                    std::fs::write(&path, contents)
    //                        .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    //                }
    //                unexpected => {
    //                    tracing::warn!("Unexpected type {unexpected:#?}");
    //                }
    //            }
    //        }
    //        Err(source) => {
    //            let url = app.url().to_string();
    //            let method = "get".to_string();
    //            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
    //            let mut error =
    //                ReqwestError::new(url, method, source, line!(), file!().to_string());
    //            error.with_body(body);
    //            return Err(error.into());
    //        }
    //    }
    //    Ok(())
    //}
    //
    ///// For each variant of [`Dataset`], request the valid range of values for each parameter name.
    ///// BEA has not implemented this method for all parameters, so we expect some calls to fail.
    /////
    ///// The GdpByIndustry and UnderlyingGdpByIndustry datasets require additional parameters for some
    ///// keys.
    //#[tracing::instrument]
    //pub async fn values() -> Result<(), BeaErr> {
    //    let req = Request::ParameterValueFilter;
    //    let mut app = req.init()?;
    //    let datasets: Vec<Dataset> = Dataset::iter().collect();
    //    for dataset in &datasets {
    //        let names = dataset.names();
    //        for name in names {
    //            dataset.value(&mut app, name).await?;
    //        }
    //    }
    //    Ok(())
    //}
    //
    ///// For each variant of [`Dataset`], request the valid range of values for each parameter name.
    ///// The `subset` variant of this method only requests data for datasets where the BEA has
    ///// implemented a response for each parameter name associated with the dataset.
    //#[tracing::instrument]
    //pub async fn values_subset() -> Result<(), BeaErr> {
    //    let req = Request::ParameterValueFilter;
    //    let mut app = req.init()?;
    //    let datasets = vec![
    //        Dataset::APIDatasetMetadata,
    //        Dataset::Iip,
    //        Dataset::Ita,
    //        Dataset::InputOutput,
    //        Dataset::IntlServSTA,
    //        Dataset::IntlServTrade,
    //        Dataset::Regional,
    //    ];
    //    for dataset in &datasets {
    //        let names = dataset.names();
    //        for name in names {
    //            dataset.value(&mut app, name).await?;
    //        }
    //    }
    //    Ok(())
    //}
    //
    ///// The `value_gdp` method uses the ParameterValuesFiltered method to specify valid
    ///// values for parameter `name` in the `Dataset` based on the table name.
    /////
    ///// Used for GdpByIndustry and UnderlyingGdpByIndustry variants.  Called by [`Self::values_gdp`] and [`Self::values_ugdp`].
    //#[tracing::instrument(skip(self, app))]
    //pub async fn value_gdp(self, app: &mut App, name: ParameterName) -> Result<(), BeaErr> {
    //    dotenvy::dotenv().ok();
    //    // path to bea_data directory
    //    let bea_data = bea_data()?;
    //    // set table_ids from the Dataset type
    //    let table_id = match self {
    //        Self::GDPbyIndustry => GdpByIndustry::read_table_id(&bea_data)?,
    //        Self::UnderlyingGDPbyIndustry => UnderlyingGdpByIndustry::read_table_id(&bea_data)?,
    //        // no other BEA datasets use table ids as a parameter value
    //        other => {
    //            tracing::error!("GdpByIndustry or UnderlyingGDPbyIndustry required, found {self}.");
    //            let error = VariantMissing::new(
    //                "GdpByIndustry or UnderlyingGDPbyIndustry required".to_string(),
    //                other.to_string(),
    //                line!(),
    //                file!().to_string(),
    //            );
    //            return Err(error.into());
    //        }
    //    };
    //    // Add dataset and target parameter to options
    //    let mut options = app.options().clone();
    //    options.with_dataset(self);
    //    options.with_target(name);
    //    // navigate to parameter_values directory
    //    let path = bea_data.join("parameter_values");
    //    // create the folder if it does not exist
    //    if !path.exists() {
    //        std::fs::DirBuilder::new()
    //            .create(&path)
    //            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
    //        tracing::info!("Target directory for Parameter Values created.");
    //    }
    //    // navigate to dataset-parameter subdirectory
    //    let path = path.join(format!("{self}_{name}"));
    //    if !path.exists() {
    //        std::fs::DirBuilder::new()
    //            .create(&path)
    //            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
    //        tracing::info!("Target directory for Parameter Values created.");
    //    }
    //    match name {
    //        ParameterName::Industry => {
    //            for id in table_id {
    //                // add table id to options
    //                options.with_table_id(*id.value());
    //                // update app with modified options
    //                app.with_options(options.clone());
    //                // fire off the get request using the configured app
    //                let data = app.get().await?;
    //                tracing::trace!("{data:#?}");
    //                // parse the response as json
    //                match data.json::<serde_json::Value>().await {
    //                    Ok(json) => {
    //                        // Convert to file storage format (Vec<u8>)
    //                        let contents = serde_json::to_vec(&json)
    //                            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    //                        // update path with file name
    //                        let path = path.join(format!(
    //                            "{self}_{name}_byTableId_{}_values.json",
    //                            id.value()
    //                        ));
    //                        tracing::info!("Current target path: {path:?}");
    //                        // Write contents of response to file
    //                        std::fs::write(&path, contents)
    //                            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    //                    }
    //                    // if reqwest cannot convert the response to json, it probably failed
    //                    Err(source) => {
    //                        let url = app.url().to_string();
    //                        let method = "get".to_string();
    //                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
    //                        let mut error = ReqwestError::new(
    //                            url,
    //                            method,
    //                            source,
    //                            line!(),
    //                            file!().to_string(),
    //                        );
    //                        error.with_body(body);
    //                        return Err(error.into());
    //                    }
    //                }
    //            }
    //        }
    //        // TODO: Test this branch
    //        ParameterName::Year => {
    //            for id in table_id {
    //                options.with_table_id(*id.value());
    //                app.with_options(options.clone());
    //                let data = app.get().await?;
    //                tracing::info!("{data:#?}");
    //                match data.json::<serde_json::Value>().await {
    //                    Ok(json) => {
    //                        // tracing::info!("{json:#?}");
    //                        let contents = serde_json::to_vec(&json)
    //                            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    //                        let path = path.join(format!(
    //                            "{self}_{name}_byTableId_{}_values.json",
    //                            id.value()
    //                        ));
    //                        tracing::info!("Current target path: {path:?}");
    //                        std::fs::write(&path, contents)
    //                            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    //                    }
    //                    Err(source) => {
    //                        let url = app.url().to_string();
    //                        let method = "get".to_string();
    //                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
    //                        let mut error = ReqwestError::new(
    //                            url,
    //                            method,
    //                            source,
    //                            line!(),
    //                            file!().to_string(),
    //                        );
    //                        error.with_body(body);
    //                        return Err(error.into());
    //                    }
    //                }
    //            }
    //        }
    //        _ => {
    //            // read table_id using Method::GetParameterValues
    //            // frequency listed in BEA USER GUIDE pg. 37
    //            tracing::info!("Unimplemented.");
    //        }
    //    };
    //    Ok(())
    //}
    //
    ///// The `values_gdp` method downloads the valid parameter values for [`Dataset::GDPbyIndustry`] variant.
    /////
    ///// Two parameters in the GdpByIndustry dataset have valid input sets that vary by table_id, namely
    ///// Year and Industry.  Obtain table ids using [`Method::GetParameterValues`](crate::Method::GetParameterValues) prior to running this
    ///// check. For these two parameters, we obtain a response for each table_id and write the result to
    ///// a folder in the BEA_DATA directory.
    /////
    ///// Due to the nested call to [`GdpByIndustry::read_table_id`], we have seperate checks for GDP and
    ///// Underlying GDP.  Less dry but somewhat clearer to write and read.
    //#[tracing::instrument]
    //pub async fn values_gdp() -> Result<(), BeaErr> {
    //    let req = Request::ParameterValueFilter;
    //    let mut app = req.init()?;
    //    let dataset = Dataset::GDPbyIndustry;
    //    let names = dataset.names();
    //    for name in names {
    //        dataset.value_gdp(&mut app, name).await?;
    //    }
    //    Ok(())
    //}
    //
    ///// The `values_ugdp` method downloads the valid parameter values for
    ///// [`Dataset::UnderlyingGDPbyIndustry`] variant.
    //#[tracing::instrument]
    //pub async fn values_ugdp() -> Result<(), BeaErr> {
    //    let req = Request::ParameterValueFilter;
    //    let mut app = req.init()?;
    //    let dataset = Dataset::UnderlyingGDPbyIndustry;
    //    let names = dataset.names();
    //    for name in names {
    //        dataset.value_gdp(&mut app, name).await?;
    //    }
    //    Ok(())
    //}
    //
    ///// The `values_gdp_set` method download the valid parameter values for the
    ///// [`Dataset::GDPbyIndustry`] and [`Dataset::UnderlyingGDPbyIndustry`] variants.
    /////
    ///// Calls [`Self::values_gdp`] and [`Self::values_ugdp`].
    //#[tracing::instrument]
    //pub async fn values_gdp_set() -> Result<(), BeaErr> {
    //    Self::values_gdp().await?;
    //    Self::values_ugdp().await?;
    //    Ok(())
    //}
    //
    ///// The `queue` method is a convenience wrapper that produces a [`Queue`] from the `Dataset`.
    ///// The weakness of this approach is that user cannot modify the iterator used to generate the
    ///// queue, so only the default iterator is accessible.  Since users are meant to be able to modify the
    ///// iterator, this lack of access is counter-productive.
    //#[tracing::instrument(skip_all)]
    //pub fn queue(&self) -> Result<Queue, BeaErr> {
    //    match self {
    //        Self::Nipa => Nipa::queue(),
    //        Self::NIUnderlyingDetail => NiUnderlyingDetail::queue(),
    //        Self::FixedAssets => FixedAssets::queue(),
    //        Self::Mne => Mne::queue(),
    //        Self::GDPbyIndustry => GdpByIndustry::queue(),
    //        Self::Ita => Ita::queue(),
    //        _ => {
    //            let error = DatasetMissing::new(
    //                "Nipa, NIUnderlyingDetail, FixedAssets, Mne, Ita or GDPbyIndustry variants required"
    //                    .to_string(),
    //                line!(),
    //                file!().to_string(),
    //            );
    //            Err(error.into())
    //        }
    //    }
    //}
    //
    //    /// The `value_sets` method constructs the value set for each variant of `Dataset`.
    //    /// Value sets contain the parameter values used to build requests for a [`Queue`].
    //    /// Used during testing to verify the build process.
    //    /// TODO: remove this and value_set method?
    //    #[tracing::instrument]
    //    pub fn value_sets() -> Result<(), BeaErr> {
    //        dotenvy::dotenv().ok();
    //        let path = bea_data()?;
    //        let datasets: Vec<Dataset> = Dataset::iter().collect();
    //        for dataset in &datasets {
    //            dataset.value_set(&path)?;
    //        }
    //        Ok(())
    //    }
}

/// The `DatasetDetails` type represents dataset descriptions in a BEA response.  This data type is
/// an intermediary between the [`serde_json::Value`] representation and the strongly-typed
/// internal representation, implemented using the lowly `String`.
///
/// ## Fields
///
/// * **dataset_description** - String describing the dataset.
/// * **dataset_name** - String representation of the dataset name.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_new::new,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct DatasetDetails {
    dataset_description: String,
    dataset_name: String,
}

impl TryFrom<serde_json::Value> for DatasetDetails {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(m) => {
                let name = map_to_string("DatasetName", &m)?;
                let desc = map_to_string("DatasetDescription", &m)?;
                Ok(Self::new(desc, name))
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

/// The `Datasets` type is a thin wrapper around a vector of [`DatasetDetails`].
///
/// Internally, we validate the variants of [`Dataset`] against the BEA response listing valid
/// datasets.  We collect the response into a `Datasets` type, map each dataset name to an existing
/// variant, and ensure that each [`DatasetDetails`] matching an existing variant, and all variants
/// are still in use.  See [`check_datasets`](crate::check::check_datasets).
///
/// TODO: Convert from struct to tuple.  May need to remove "binary" methods first.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "PascalCase")]
pub struct Datasets {
    dataset: Vec<DatasetDetails>,
}

impl TryFrom<serde_json::Value> for Datasets {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading DatasetDetails");
        match value {
            serde_json::Value::Object(m) => {
                let key = "Dataset".to_string();
                if let Some(serde_json::Value::Array(v)) = m.get(&key) {
                    tracing::trace!("Array found.");
                    let mut dataset = Vec::new();
                    for val in v {
                        let details = DatasetDetails::try_from(val.clone())?;
                        dataset.push(details);
                    }
                    tracing::trace!("Details found.");
                    let datasets = Datasets::new(dataset);
                    Ok(datasets)
                } else {
                    tracing::trace!("Unexpected content: {m:#?}");
                    let error = KeyMissing::new(key, line!(), file!().to_string());
                    Err(error.into())
                }
            }
            _ => {
                tracing::trace!("Wrong Value type: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}
