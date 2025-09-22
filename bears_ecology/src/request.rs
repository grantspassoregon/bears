use crate::{App, History, Mode, Options, Overwrite, Queue, Scope, bea_data, init};
use bears_species::{
    BeaErr, BeaResponse, Data, Dataset, DatasetMissing, FixedAssets, GdpByIndustry, Iip,
    InputOutput, IoError, Ita, Method, Mne, NiUnderlyingDetail, Nipa, ParameterName, ReqwestError,
    Results, SerdeJson, UnderlyingGdpByIndustry, VariantMissing,
};
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
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum Request {
    #[default]
    Data,
    Dataset,
    Parameter,
    ParameterValue,
    ParameterValueFilter,
}

impl Request {
    #[tracing::instrument(skip_all)]
    pub fn init(&self) -> Result<App, BeaErr> {
        match self {
            Self::Data => {
                let method = Method::GetData;
                init_method(method)
            }
            Self::Dataset => {
                let method = Method::GetDataSetList;
                init_method(method)
            }
            Self::Parameter => {
                let method = Method::GetParameterList;
                init_method(method)
            }
            Self::ParameterValue => {
                let method = Method::GetParameterValues;
                init_method(method)
            }
            Self::ParameterValueFilter => {
                let method = Method::GetParameterValuesFiltered;
                init_method(method)
            }
        }
    }
}

#[tracing::instrument]
fn init_method(method: Method) -> Result<App, BeaErr> {
    let mut app = init()?;
    tracing::info!("App initialized.");
    let mut options = Options::default();
    let _ = options.with_method(method);
    app.with_options(options);
    tracing::info!("App configured for {method}.");
    Ok(app)
}

pub fn init_queue(dataset: Dataset) -> Result<Queue, BeaErr> {
    let req = Request::Data;
    let mut app = req.init()?;
    app.with_dataset(dataset);
    // dotenvy::dotenv().ok();
    let path = bea_data()?;
    let mut queue = Vec::new();

    match dataset {
        Dataset::FixedAssets => {
            let data = FixedAssets::try_from(&path)?;
            for params in data.iter_tables() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::GDPbyIndustry => {
            let data = GdpByIndustry::try_from(&path)?;
            for params in data.iter_tables() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::UnderlyingGDPbyIndustry => {
            tracing::info!("Constructing key set for {dataset}.");
            let data = UnderlyingGdpByIndustry::try_from(&path)?;
            tracing::info!("Key set constructed for {dataset}.");
            for params in data.iter() {
                tracing::trace!("Adding params {params:#?}");
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::Iip => {
            let data = Iip::try_from(&path)?;
            for params in data.iter_investments() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::InputOutput => {
            let data = InputOutput::try_from(&path)?;
            for params in data.iter() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::Ita => {
            let data = Ita::try_from(&path)?;
            for params in data.iter() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::Mne => {
            let mut data = Mne::try_from(&path)?;
            data.filter_country("all");

            for params in data.iter_mne() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
            for params in data.iter_amne() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::Nipa => {
            let data = Nipa::try_from(&path)?;
            for params in data.iter() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        Dataset::NIUnderlyingDetail => {
            let data = NiUnderlyingDetail::try_from(&path)?;
            for params in data.iter() {
                app.with_params(params.clone());
                queue.push(app.clone());
            }
        }
        _ => {
            tracing::error!("Queue not implemented for dataset {dataset}");
            let error = DatasetMissing::new(dataset.to_string(), line!(), file!().to_owned());
            return Err(error.into());
        }
    }

    Ok(Queue::new(queue))
}

/// Download the BEA dataset parameter values into the `BEA_DATA` directory.
pub async fn get_datasets() -> Result<(), BeaErr> {
    let req = Request::Dataset;
    let app = req.init()?;
    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let contents = serde_json::to_vec(&json)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            dotenvy::dotenv().ok();
            let path = bea_data()?;
            let path = path.join("datasets.json");
            std::fs::write(&path, contents)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let error = ReqwestError::new(url, method, source, line!(), file!().to_string());
            Err(error.into())
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn initial_download(dataset: Dataset) -> Result<(), BeaErr> {
    let queue = init_queue(dataset)?;
    tracing::info!("Queue length: {}", queue.len());
    queue.download(Overwrite::No).await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn download_with_history(
    dataset: Dataset,
    style: indicatif::ProgressStyle,
) -> Result<(), BeaErr> {
    let queue = init_queue(dataset)?;
    tracing::info!("Queue length: {}", queue.len());

    // get the download history for the size hints
    let history = History::try_from((dataset, Mode::Load))?;
    history.summary();
    history
        .iter()
        .download(&queue, Overwrite::No, style)
        .await?;
    Ok(())
}

/// Load all successfully downloaded files in the download [`History`] for the `Dataset`.
/// If the user provides a `load_history`, the method will exclude previously loaded files in
/// the provided [`History`].
#[tracing::instrument(skip_all)]
pub fn initial_load_par(
    dataset: Dataset,
    load_history: Option<&History>,
) -> Result<Vec<Data>, BeaErr> {
    let mut queue = init_queue(dataset)?;
    tracing::info!("Queue length: {}", queue.len());

    // A fresh queue has been downloaded, try loading the successes
    let downloads = History::try_from((dataset, Mode::Download))?;
    // only download successes in history
    // strict = true set to include no others in queue.
    queue.successes(&downloads, Scope::History)?;
    tracing::info!("Files downloaded: {}", queue.len());

    if let Some(loads) = load_history {
        // exclude previously loaded files in load history
        queue.exclude(loads)?;
        tracing::info!("Files left to load: {}", queue.len());
    }

    let data = queue.load_par()?;
    tracing::info!("{} datasets loaded.", data.len());
    Ok(data)
}

/// Load all successfully downloaded files in the download [`History`] for the `Dataset`.
/// If the user provides a `load_history`, the method will exclude previously loaded files in
/// the provided [`History`].
#[tracing::instrument(skip_all)]
pub async fn initial_load(
    dataset: Dataset,
    load_history: Option<&History>,
) -> Result<Vec<Data>, BeaErr> {
    let mut queue = init_queue(dataset)?;
    tracing::info!("Queue length: {}", queue.len());

    // A fresh queue has been downloaded, try loading the successes
    let downloads = History::try_from((dataset, Mode::Download))?;
    // only download successes in history
    // strict = true set to include no others in queue.
    queue.successes(&downloads, Scope::History)?;
    tracing::info!("Files downloaded: {}", queue.len());

    if let Some(loads) = load_history {
        // exclude previously loaded files in load history
        queue.exclude(loads)?;
        tracing::info!("Files left to load: {}", queue.len());
    }

    let data = queue.load().await?;
    let data = data.lock().await;
    tracing::info!("{} datasets loaded.", data.len());
    Ok(data.to_vec())
}

/// Tries to load any files in the history that previously failed to load.
#[tracing::instrument(skip_all)]
pub async fn retry_load(dataset: Dataset) -> Result<Vec<Data>, BeaErr> {
    let mut queue = init_queue(dataset)?;
    tracing::info!("Queue length: {}", queue.len());

    // The load history contains errors, try them again.
    let history = History::try_from((dataset, Mode::Load))?;
    // strict is true means only download errors included in the event history
    queue.errors(&history, Scope::History)?;
    tracing::info!("Files to retry: {}", queue.len());

    let data = queue.load().await?;
    let data = data.lock().await;
    tracing::info!("{} datasets loaded.", data.len());
    Ok(data.to_vec())
}

/// The `parameter` method reads a [`BeaResponse`] to json using the `serde_json` crate.
/// Saves the result to the `BEA_DATA` directory.
///
/// Called by [`Self::parameters`].
#[tracing::instrument(skip_all)]
async fn parameter(dataset: Dataset, app: &mut App) -> Result<(), BeaErr> {
    app.with_dataset(dataset);
    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let contents = serde_json::to_vec(&json)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            dotenvy::dotenv().ok();
            let path = bea_data()?;
            let path = path.join("parameters");
            if !path.exists() {
                std::fs::DirBuilder::new()
                    .create(&path)
                    .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
                tracing::info!("Target directory for Parameters created.");
            }
            let path = path.join(format!("{dataset}_parameters.json"));
            std::fs::write(&path, contents)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
            let mut error = ReqwestError::new(url, method, source, line!(), file!().to_string());
            let _ = error.with_body(body);
            Err(error.into())
        }
    }
}

/// For each variant of [`Dataset`], request the parameters.
/// Write the results in JSON format to the BEA_DATA directory.
#[tracing::instrument]
pub async fn parameters() -> Result<(), BeaErr> {
    let req = Request::Parameter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        parameter(dataset, &mut app).await?;
    }
    Ok(())
}

/// The `parameter_value` method requests the valid values for parameter `name` of the
/// `Dataset`.
///
/// Called by [`Self::parameter_values`].
#[tracing::instrument(skip_all)]
async fn parameter_value(
    dataset: Dataset,
    app: &mut App,
    name: ParameterName,
) -> Result<(), BeaErr> {
    let mut opts = app.options().clone();
    let _ = opts.with_dataset(dataset);
    let _ = opts.with_param_name(name);
    app.with_options(opts);
    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let contents = serde_json::to_vec(&json)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            dotenvy::dotenv().ok();
            let path = bea_data()?;
            let path = path.join("parameter_values");
            if !path.exists() {
                std::fs::DirBuilder::new()
                    .create(&path)
                    .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
                tracing::info!("Target directory for Parameter Values created.");
            }
            let path = path.join(format!("{dataset}_{name}_parameter_values.json"));
            std::fs::write(&path, contents)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
            let mut error = ReqwestError::new(url, method, source, line!(), file!().to_string());
            let _ = error.with_body(body);
            Err(error.into())
        }
    }
}

/// The `parameter_values` method requests the valid range of values for each parameter in the
/// [`Dataset`].
///
/// After a successfull response from an API request, the goal is to parse the response into
/// internal library data structures.  The JSON responses can include heavily nested data
/// structures, which makes deserializing directly into Rust types a brittle process.  Instead, we
/// first we deserialize the JSON into serde_json types, and then migrate the results into our
/// internal library types using the [`TryFrom`] trait.  While this is a bit heavier on
/// boilerplate, the errors and logs are easier to consume, providing a clearing path to a correct
/// implementation result during the development process.
///
/// Here we request a parameter values table from the server, parse it into serde_json types, and
/// write the results to the `BEA_DATA` directory.  Later we can attempt to parse the response
/// multiple times into our internal library types, succussfully or unsuccessfully, without making
/// repeated API calls to BEA for the same data.
#[tracing::instrument]
pub async fn parameter_values() -> Result<(), BeaErr> {
    let req = Request::ParameterValue;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        let names = dataset.names();
        for name in names {
            parameter_value(dataset, &mut app, name).await?;
        }
    }
    Ok(())
}

/// The `value` method downloads the valid values for parameter `name` in the `Dataset`.
///
/// Called by [`Self::values`].
#[tracing::instrument(skip_all)]
async fn value(dataset: Dataset, app: &mut App, name: ParameterName) -> Result<(), BeaErr> {
    let mut options = app.options().clone();
    let _ = options.with_dataset(dataset);
    let _ = options.with_target(name);
    app.with_options(options.clone());
    let data = app.get().await?;
    tracing::info!("{data:#?}");
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            tracing::info!("{json:#?}");
            let bea = BeaResponse::try_from(&json)?;
            match bea.results() {
                Results::ApiError(e) => {
                    // TODO: Map api error codes to an enum.
                    tracing::info!("{e}");
                }
                Results::ParameterValues(_) => {
                    let contents = serde_json::to_vec(&json)
                        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
                    dotenvy::dotenv().ok();
                    let path = bea_data()?;
                    let path = path.join("parameter_values");
                    if !path.exists() {
                        std::fs::DirBuilder::new()
                            .create(&path)
                            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
                        tracing::info!("Target directory for Parameter Values created.");
                    }
                    let path = path.join(format!("{dataset}_{name}_values.json"));
                    std::fs::write(&path, contents)
                        .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
                }
                unexpected => {
                    tracing::warn!("Unexpected type {unexpected:#?}");
                }
            }
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
            let mut error = ReqwestError::new(url, method, source, line!(), file!().to_string());
            let _ = error.with_body(body);
            return Err(error.into());
        }
    }
    Ok(())
}

/// For each variant of [`Dataset`], request the valid range of values for each parameter name.
/// BEA has not implemented this method for all parameters, so we expect some calls to fail.
///
/// The GdpByIndustry and UnderlyingGdpByIndustry datasets require additional parameters for some
/// keys.
#[tracing::instrument]
pub async fn values() -> Result<(), BeaErr> {
    let req = Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            value(*dataset, &mut app, name).await?;
        }
    }
    Ok(())
}

/// For each variant of [`Dataset`], request the valid range of values for each parameter name.
/// The `subset` variant of this method only requests data for datasets where the BEA has
/// implemented a response for each parameter name associated with the dataset.
#[tracing::instrument]
pub async fn values_subset() -> Result<(), BeaErr> {
    let req = Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets = vec![
        Dataset::APIDatasetMetadata,
        Dataset::Iip,
        Dataset::Ita,
        Dataset::InputOutput,
        Dataset::IntlServSTA,
        Dataset::IntlServTrade,
        Dataset::Regional,
    ];
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            value(*dataset, &mut app, name).await?;
        }
    }
    Ok(())
}

/// The `value_gdp` method uses the ParameterValuesFiltered method to specify valid
/// values for parameter `name` in the `Dataset` based on the table name.
///
/// Used for GdpByIndustry and UnderlyingGdpByIndustry variants.  Called by [`Self::values_gdp`] and [`Self::values_ugdp`].
#[tracing::instrument(skip_all)]
async fn value_gdp(dataset: Dataset, app: &mut App, name: ParameterName) -> Result<(), BeaErr> {
    dotenvy::dotenv().ok();
    // path to bea_data directory
    let bea_data = bea_data()?;
    // set table_ids from the Dataset type
    let table_id = match dataset {
        Dataset::GDPbyIndustry => GdpByIndustry::read_table_id(&bea_data)?,
        Dataset::UnderlyingGDPbyIndustry => UnderlyingGdpByIndustry::read_table_id(&bea_data)?,
        // no other BEA datasets use table ids as a parameter value
        other => {
            tracing::error!("GdpByIndustry or UnderlyingGDPbyIndustry required, found {dataset}.");
            let error = VariantMissing::new(
                "GdpByIndustry or UnderlyingGDPbyIndustry required".to_string(),
                other.to_string(),
                line!(),
                file!().to_string(),
            );
            return Err(error.into());
        }
    };
    // Add dataset and target parameter to options
    let mut options = app.options().clone();
    let _ = options.with_dataset(dataset);
    let _ = options.with_target(name);
    // navigate to parameter_values directory
    let path = bea_data.join("parameter_values");
    // create the folder if it does not exist
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("Target directory for Parameter Values created.");
    }
    // navigate to dataset-parameter subdirectory
    let path = path.join(format!("{dataset}_{name}"));
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("Target directory for Parameter Values created.");
    }
    match name {
        ParameterName::Industry => {
            for id in table_id {
                // add table id to options
                let _ = options.with_table_id(*id.value());
                // update app with modified options
                app.with_options(options.clone());
                // fire off the get request using the configured app
                let data = app.get().await?;
                tracing::trace!("{data:#?}");
                // parse the response as json
                match data.json::<serde_json::Value>().await {
                    Ok(json) => {
                        // Convert to file storage format (Vec<u8>)
                        let contents = serde_json::to_vec(&json)
                            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
                        // update path with file name
                        let path = path.join(format!(
                            "{dataset}_{name}_byTableId_{}_values.json",
                            id.value()
                        ));
                        tracing::info!("Current target path: {path:?}");
                        // Write contents of response to file
                        std::fs::write(&path, contents)
                            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
                    }
                    // if reqwest cannot convert the response to json, it probably failed
                    Err(source) => {
                        let url = app.url().to_string();
                        let method = "get".to_string();
                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                        let mut error =
                            ReqwestError::new(url, method, source, line!(), file!().to_string());
                        let _ = error.with_body(body);
                        return Err(error.into());
                    }
                }
            }
        }
        // TODO: Test this branch
        ParameterName::Year => {
            for id in table_id {
                let _ = options.with_table_id(*id.value());
                app.with_options(options.clone());
                let data = app.get().await?;
                tracing::info!("{data:#?}");
                match data.json::<serde_json::Value>().await {
                    Ok(json) => {
                        // tracing::info!("{json:#?}");
                        let contents = serde_json::to_vec(&json)
                            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
                        let path = path.join(format!(
                            "{dataset}_{name}_byTableId_{}_values.json",
                            id.value()
                        ));
                        tracing::info!("Current target path: {path:?}");
                        std::fs::write(&path, contents)
                            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
                    }
                    Err(source) => {
                        let url = app.url().to_string();
                        let method = "get".to_string();
                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                        let mut error =
                            ReqwestError::new(url, method, source, line!(), file!().to_string());
                        let _ = error.with_body(body);
                        return Err(error.into());
                    }
                }
            }
        }
        _ => {
            // read table_id using Method::GetParameterValues
            // frequency listed in BEA USER GUIDE pg. 37
            tracing::info!("Unimplemented.");
        }
    };
    Ok(())
}

/// The `values_gdp` method downloads the valid parameter values for [`Dataset::GDPbyIndustry`] variant.
///
/// Two parameters in the GdpByIndustry dataset have valid input sets that vary by table_id, namely
/// Year and Industry.  Obtain table ids using [`Method::GetParameterValues`](crate::Method::GetParameterValues) prior to running this
/// check. For these two parameters, we obtain a response for each table_id and write the result to
/// a folder in the BEA_DATA directory.
///
/// Due to the nested call to [`GdpByIndustry::read_table_id`], we have seperate checks for GDP and
/// Underlying GDP.  Less dry but somewhat clearer to write and read.
#[tracing::instrument]
pub async fn values_gdp() -> Result<(), BeaErr> {
    let req = Request::ParameterValueFilter;
    let mut app = req.init()?;
    let dataset = Dataset::GDPbyIndustry;
    let names = dataset.names();
    for name in names {
        value_gdp(dataset, &mut app, name).await?;
    }
    Ok(())
}

/// The `values_ugdp` method downloads the valid parameter values for
/// [`Dataset::UnderlyingGDPbyIndustry`] variant.
#[tracing::instrument]
pub async fn values_ugdp() -> Result<(), BeaErr> {
    let req = Request::ParameterValueFilter;
    let mut app = req.init()?;
    let dataset = Dataset::UnderlyingGDPbyIndustry;
    let names = dataset.names();
    for name in names {
        value_gdp(dataset, &mut app, name).await?;
    }
    Ok(())
}

/// The `values_gdp_set` method download the valid parameter values for the
/// [`Dataset::GDPbyIndustry`] and [`Dataset::UnderlyingGDPbyIndustry`] variants.
///
/// Calls [`Self::values_gdp`] and [`Self::values_ugdp`].
#[tracing::instrument]
pub async fn values_gdp_set() -> Result<(), BeaErr> {
    values_gdp().await?;
    values_ugdp().await?;
    Ok(())
}

/// The `queue` method is a convenience wrapper that produces a [`Queue`] from the `Dataset`.
/// The weakness of this approach is that user cannot modify the iterator used to generate the
/// queue, so only the default iterator is accessible.  Since users are meant to be able to modify the
/// iterator, this lack of access is counter-productive.
/// TODO: Improve and remove.
#[tracing::instrument(skip_all)]
pub fn queue(dataset: Dataset) -> Result<Queue, BeaErr> {
    init_queue(dataset)
}
