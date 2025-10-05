use crate::{Options, ParameterKind, bea_data};
use bears_species::{
    BTreeKeyMissing, BeaErr, BeaResponse, Dataset, DeriveFromStr, IoError, JsonParseError,
    KeyMissing, Method, MillionsOptions, ParameterName, RateLimit, ReqwestError, Results,
    SerdeJson, VariantMissing,
};
use std::collections::BTreeMap;
use std::str::FromStr;

/// The `App` struct contains the application state.
///
/// The [`get`](Self::get) method makes calls to the BEA REST API constructing calls from the
/// [`Options`] contained in the `options` field.
///
/// Originally intended to contain the parent-level logic of the application, in practice this
/// struct contains the configuration for a single server request.  For series of requests, we
/// generate a queue of `App` structs with the desired target configurations.
///
/// The fields of the `App` struct include:
///
/// * **key** - The API key of the user obtained from the BEA.
/// * **options** - An [`Options`] struct configured for a request.
/// * **url** - The [`url::Url`] for the BEA REST API server.
/// * **query** - Query paramters formed for use in a [`reqwest::get`] request.  The query is
///   derived from the `key` and `options` fields.  Use the provided library api to update the
///   options fields ([`App::with_options`] and [`App::with_params`]), as these methods keep the `query` field updated and in sync with new
///   changes.
/// * **size_hint** - Indicates the expected size of the request payload based on a download
///   [`History`](crate::History).  In the absence of a download history, this field is `None`.  Used
///   during rate limiting by the [`Tracker`](crate::Tracker) type to avoid exceeding the 100 MB per
///   minute rate threshold of the BEA REST server.
///
///   ## Usage
///
///   The BEA_URL environmental variable is the path to the BEA REST API server.
///   The API_KEY environmental variable must be set to the user API key issued by the BEA.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_getters::Getters,
    derive_setters::Setters,
    serde::Serialize,
    serde::Deserialize,
)]
#[setters(prefix = "with_", into, borrow_self)]
pub struct App {
    key: String,
    #[setters(skip)]
    options: Options,
    url: url::Url,
    query: BTreeMap<String, String>,
    size_hint: Option<u64>,
}

impl App {
    ///   Create a new `App` from the user API key `key`, a set of user [`Options`] `options`, and the BEA url `url`,
    ///   using the [`App::new`] method.
    ///
    ///   ```
    ///   use bears::{App, Options};
    ///
    ///   // read the environmental variables from the .env file
    ///   dotenvy::dotenv().ok();
    ///   // path to the BEA REST server
    ///   let url = std::env::var("BEA_URL").unwrap();
    ///   // valid url type expected
    ///   let url = url::Url::parse(&url).unwrap();
    ///   // user API key granted by the BEA
    ///   let key = std::env::var("API_KEY").unwrap();
    ///   // and empty set of options
    ///   let options = Options::default();
    ///   // a "default" app
    ///   let app = App::new(key, options, url);
    ///
    ///   ```
    pub fn new(key: String, options: Options, url: url::Url) -> Self {
        let mut query = options.params();
        query.insert(ParameterKind::UserId.header(), key.clone());
        Self {
            key,
            options,
            url,
            query,
            size_hint: None,
        }
    }

    /// The `with_options` method sets the value of the `options` field in self, using the provided
    /// [`Options`] in the `options` parameter.  The implementation updates the value of the `query`
    /// field in self, which includes any values added to `options`, so we must update `query` when `options` changes.
    ///
    /// ```
    /// use bears::{init, App, Dataset, Options, Queue};
    /// use strum::IntoEnumIterator;
    ///
    /// // helper method to create a default app
    /// let mut app = init().unwrap();
    /// // Gather the variants of Dataset into a vector.
    /// let datasets = Dataset::iter().collect::<Vec<Dataset>>();
    /// // Collect requests in a vector.
    /// let mut queue = Vec::new();
    /// // for each variant
    /// for dataset in datasets {
    ///     // get a vector of valid parameter names
    ///     let names = dataset.names();
    ///     // for each parameter name
    ///     for name in names {
    ///         // create a mutable clone of the app options
    ///         let mut options = app.options().clone();
    ///         // add the dataset to the options
    ///         options.with_dataset(dataset);
    ///         // add the parameter name to the options
    ///         options.with_target(name);
    ///         // update the app with new options
    ///         app.with_options(options.clone());
    ///         queue.push(app.clone());
    ///     }
    /// }
    ///
    /// // Wrap the vector of requests in a Queue.
    /// let queue = Queue::new(queue);
    ///
    /// assert_eq!(queue.len(), 56);
    /// ```
    pub fn with_options(&mut self, options: Options) {
        self.options = options;
        self.query = self.params();
    }

    /// Produces the parameters appended to the REST endpoint.  Used by [`App::with_options`]
    /// and [`App::with_dataset`].
    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> BTreeMap<String, String> {
        let mut params = self.options.params();
        params.insert(ParameterKind::UserId.header(), self.key.clone());
        params
    }

    /// Append parameters to the cached query.  Used to update an existing `App` configuration with
    /// new parameters.  In a loop, we may change a single parameter on each iteration, and
    /// appending the new value will overwrite the old, which is the desired behavior in this
    /// context.  Primarily used to generate a [`Queue`](crate::Queue) from a data iterator.
    #[tracing::instrument(skip_all)]
    pub fn with_params(&mut self, params: BTreeMap<String, String>) {
        self.query.extend(params);
    }

    /// Borrows a mutable reference to the `options` field of self.  Used to update the options
    /// with new parameter values, prior to executing a request.  Called by [`App::with_dataset`].
    /// It my be wise to remove this method and force users to clone or create a new [`Options`] to
    /// add to the `App`.  This will prevent a user from using this method to update the options of
    /// the app without updating the `query` field, a potential foot gun.
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }

    /// A convenience wrapper around [`Options::with_dataset`], allowing the user to set the
    /// `dataset` value of the `options` in self as the [`Dataset`] provided in the `dataset` parameter.
    ///
    /// ```
    /// use bears::{init, Dataset};
    ///
    /// // helper method to create a default app
    /// let mut app = init().unwrap();
    /// // select a dataset variant
    /// let dataset = Dataset::FixedAssets;
    /// // add the dataset to the app options
    /// app.with_dataset(dataset);
    ///
    /// // retrieve dataset value from app
    /// let fixed_assets = app.dataset().unwrap();
    /// // that they are equal confirms the options was set correctly
    /// assert_eq!(dataset, fixed_assets);
    /// ```
    pub fn with_dataset(&mut self, dataset: Dataset) {
        let options = self.options_mut();
        let _ = options.with_dataset(dataset);
        self.query = self.params();
    }

    /// Internal library workhorse function for REST API calls.  Configure the desired parameters
    /// of the call using the [`Options`].
    ///
    /// ```
    /// use bears::Request;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// // helper type to create a preconfigured app
    /// let req = Request::Dataset;
    /// // initialize app
    /// let app = req.init().unwrap();
    /// // send the configured request to the BEA server
    /// let data = app.get().await.unwrap();
    /// # }
    /// ```
    #[tracing::instrument(skip_all)]
    pub async fn get(&self) -> Result<reqwest::Response, ReqwestError> {
        tracing::trace!("Calling get for App.");
        let body = self
            .query
            .clone()
            .into_iter()
            .collect::<Vec<(String, String)>>();
        let client = reqwest::Client::new();
        let req = client.get(self.url.clone()).query(&self.query);
        tracing::trace!("Sending request: {:?}", req);
        match req.send().await {
            Ok(res) => Ok(res),
            Err(source) => {
                let mut error = ReqwestError::new(
                    self.url().to_string(),
                    "get".to_string(),
                    source,
                    line!(),
                    file!().to_string(),
                );
                let _ = error.with_body(body);
                Err(error)
            }
        }
    }

    /// Returns the value of the parameter containing the selected [`Method`].  Used to determine
    /// path destinations for queries.
    pub fn method(&self) -> Result<Method, BeaErr> {
        let query = self.query();
        let key = ParameterKind::Method.header();
        let method = match query.get(&key) {
            Some(value) => value.to_string(),
            None => {
                let error = BTreeKeyMissing::new(key, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        Method::from_str(&method)
            .map_err(|source| DeriveFromStr::new(method, source, line!(), file!().into()).into())
    }

    /// Returns the value of the parameter containing the selected [`Dataset`].  Used to determine
    /// path destinations for queries.
    pub fn dataset(&self) -> Result<Dataset, BeaErr> {
        let query = self.query();
        let key = ParameterKind::Dataset.header();
        let dataset = match query.get(&key) {
            Some(value) => value.to_string(),
            None => {
                let error = BTreeKeyMissing::new(key, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        Dataset::from_str(&dataset)
            .map_err(|source| DeriveFromStr::new(dataset, source, line!(), file!().into()).into())
    }

    /// The `destination` method returns the query path of self.  The query parameters in the `query` field of self determine the path destination.
    /// Called by [`App::save`] and [`App::load`].
    // TODO: Handle year or year ranges for individual and multiple selections.
    // TODO: Break into smaller functions for improved code clarity.
    pub fn destination(&self, create: bool) -> Result<std::path::PathBuf, BeaErr> {
        let query = self.query();
        tracing::trace!("Params are {:#?}", query);
        let method = self.method()?;
        let dataset = self.dataset()?;
        dotenvy::dotenv().ok();
        let path = bea_data()?;
        match method {
            Method::GetData => {
                let path = path.join("data");
                if !path.exists() && create {
                    std::fs::DirBuilder::new()
                        .create(&path)
                        .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
                    tracing::info!("Target directory for Data created.");
                }
                let path = path.join(dataset.to_string());
                if !path.exists() && create {
                    std::fs::DirBuilder::new()
                        .create(&path)
                        .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
                    tracing::info!("Target directory for {dataset} created.");
                }
                match dataset {
                    Dataset::Nipa => {
                        let name = query["TableName"].clone();
                        let millions = query["ShowMillions"].clone();
                        let millions = MillionsOptions::from_value(&millions)?;
                        match millions {
                            MillionsOptions::Yes => {
                                Ok(path.join(format!("{dataset}_{name}_millions.json")))
                            }
                            MillionsOptions::No => Ok(path.join(format!("{dataset}_{name}.json"))),
                        }
                    }
                    Dataset::NIUnderlyingDetail | Dataset::FixedAssets => {
                        let name = query["TableName"].clone();
                        Ok(path.join(format!("{dataset}_{name}.json")))
                    }
                    Dataset::Mne => {
                        let country = query["Country"].clone();
                        let doi = query["DirectionOfInvestment"].clone();
                        let class = query["Classification"].clone();
                        if let Some(nonbank) =
                            query.get(ParameterName::NonbankAffiliatesOnly.to_string().as_str())
                        {
                            let path = path.join("AMNE");
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for AMNE created.");
                            }
                            let path = path.join(country);
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for {dataset} created.");
                            }
                            let ownership = query["OwnershipLevel"].clone();
                            let path = match (ownership.as_str(), nonbank.as_str()) {
                                ("0", "0") => path.join(format!("{class}_{doi}.json")),
                                ("0", "1") => path.join(format!("{class}_{doi}_nonbank.json")),
                                ("1", "0") => {
                                    path.join(format!("{class}_{doi}_ownership_nonbank.json"))
                                }
                                ("1", "1") => {
                                    path.join(format!("{class}_{doi}_ownership_nonbank.json"))
                                }
                                _ => {
                                    let error = KeyMissing::new(
                                        "0 or 1".to_string(),
                                        line!(),
                                        file!().to_string(),
                                    );
                                    return Err(error.into());
                                }
                            };
                            Ok(path)
                        } else {
                            let path = path.join("DirectInvestment");
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for DirectInvestment created.");
                            }
                            let path = path.join(country);
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for {dataset} created.");
                            }
                            Ok(path.join(format!("{class}_{doi}.json")))
                        }
                    }
                    Dataset::GDPbyIndustry => {
                        let table_id = query[&ParameterName::TableID.to_string()].clone();
                        let mut title = table_id;
                        if let Some(frequency) = query.get(&ParameterName::Frequency.to_string())
                            && frequency != "A,Q"
                        {
                            title.push('_');
                            title.push_str(frequency);
                        }
                        if let Some(value) = query.get(&ParameterName::Industry.to_string())
                            && value != "ALL"
                        {
                            title.push('_');
                            title.push_str(value);
                        }
                        if let Some(value) = query.get(&ParameterName::Year.to_string())
                            && value != "ALL"
                        {
                            title.push('_');
                            title.push_str(value);
                        }
                        title.push_str(".json");
                        Ok(path.join(title))
                    }
                    Dataset::UnderlyingGDPbyIndustry => {
                        let table_id = query[&ParameterName::TableID.to_string()].clone();
                        let industry = query[&ParameterName::Industry.to_string()].clone();
                        let title = format!("{dataset}_{table_id}_{industry}.json");
                        Ok(path.join(title))
                    }
                    Dataset::Ita => {
                        let aoc = query["AreaOrCountry"].clone();
                        Ok(path.join(format!("{aoc}.json")))
                    }
                    Dataset::Iip => {
                        // let component = query["Component"].clone();
                        let name = ParameterName::TypeOfInvestment;
                        let toi = query[&name.to_string()].clone();
                        Ok(path.join(format!("IIP_{toi}.json")))
                    }
                    Dataset::InputOutput => {
                        let table_id = query["TableID"].clone();
                        Ok(path.join(format!("InputOutput_{table_id}.json")))
                    }
                    _ => {
                        tracing::info!("{dataset} not yet implemented.");
                        Ok(path)
                    }
                }
            }
            _ => {
                tracing::info!("Not implemented for {method}.");
                Ok(path)
            }
        }
    }
    #[tracing::instrument(skip_all)]

    /// The `download` method sends a get request based upon the `App` configuration.
    /// If the request is successful, it writes the JSON response to the `BEA_DATA` directory.
    /// Returns a [`ResultStatus`] indicating the outcome the request.
    ///
    /// The `id` parameter is the [`Event`](crate::Event) id assigned by the [`Tracker`](crate::Tracker) to the request.  Since
    /// downloads are asynchronous tasks and must be Send, we pass the event id to the
    /// [`ResultStatus`] so the [`Tracker`](crate::Tracker) can update the appropriate event with the result.
    /// Called by [`Queue::downloader`](crate::Queue::downloader).
    ///
    /// For one-off requests, tracking is unnecessary and it is better to use the [`App::get`]
    /// method directly.
    pub async fn download(&self, id: uuid::Uuid) -> Result<ResultStatus, BeaErr> {
        tracing::trace!("Calling download.");
        let query = self.query();
        tracing::trace!("Params are {:#?}", query);
        let method = self.method()?;
        match method {
            Method::GetData => {
                let data = self.get().await?;
                let length = data.content_length().unwrap();
                match data.json::<serde_json::Value>().await {
                    Ok(json) => match BeaResponse::try_from(&json) {
                        Ok(response) => match response.results() {
                            Results::ApiError(error) => {
                                tracing::error!("{error}");
                                return Ok(ResultStatus::Error(id));
                            }
                            Results::MneError(error) => {
                                tracing::trace!("{error}");
                                return Ok(ResultStatus::Error(id));
                            }
                            Results::RequestsExceeded(error) => {
                                let error =
                                    RateLimit::new(error.to_string(), line!(), file!().to_string());
                                tracing::error!("{error}");
                                // return Err(error.into());
                                // tracing::error!("Limit rate exceeded, pausing for one hour.");
                                // tokio::time::sleep(std::time::Duration::from_millis(3600000)).await;

                                return Ok(ResultStatus::Abort);
                            }
                            _ => {
                                self.save(json)?;
                                return Ok(ResultStatus::Success(id, length));
                            }
                        },
                        Err(_) => {
                            tracing::warn!("Unrecognized response.");
                            self.save(json)?;
                            return Ok(ResultStatus::Success(id, length));
                        }
                    },
                    Err(source) => {
                        let url = self.url().to_string();
                        let method = "get".to_string();
                        let error =
                            ReqwestError::new(url, method, source, line!(), file!().to_string());
                        tracing::warn!("{error}");
                        // return Err(error.into());
                        return Ok(ResultStatus::Error(id));
                    }
                }
            }
            _ => {
                tracing::info!("{method} not implemented.");
                Ok(ResultStatus::Error(id))
            }
        }
    }

    /// The `save` method writes a [`serde_json::Value`] to the `BEA_DATA` directory.
    pub fn save(&self, json: serde_json::Value) -> Result<(), BeaErr> {
        tracing::trace!("Calling save.");
        let method = self.method()?;
        match method {
            Method::GetData => {
                let contents = serde_json::to_vec(&json)
                    .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;

                let path = self.destination(true)?;
                std::fs::write(&path, contents)
                    .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            }
            _ => {
                tracing::info!("Not implemented for {method}.");
            }
        }
        Ok(())
    }

    /// The `load` method reads a [`BeaResponse`] from the `BEA_DATA` directory.  Uses the `App`
    /// configuration to determine the file destination.
    pub fn load(&self) -> Result<BeaResponse, BeaErr> {
        tracing::trace!("Calling load.");
        let query = self.query();
        tracing::trace!("Params are {:#?}", query);
        let method = self.method()?;
        match method {
            Method::GetData => {
                let path = self.destination(false)?;
                tracing::trace!("Opening {path:?}.");
                // Create reader from path.
                let file = std::fs::File::open(&path)
                    .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
                let rdr = std::io::BufReader::new(file);
                // Deserialize to serde_json::Value.
                let json: serde_json::Value = serde_json::from_reader(rdr)
                    .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
                BeaResponse::try_from(&json)
            }
            _ => {
                let msg = format!("load not implemented for {method}");
                tracing::info!(msg);
                let error =
                    VariantMissing::new(msg, method.to_string(), line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

/// The `ResultStatus` enum represents different possible outcomes for async load and download
/// tasks.
/// Used when asynchronous processes need to provide a return value indicating the outcome status
/// of their task.  The primary purpose is for logging and to aid in control flow when managing
/// tasks.
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
)]
pub enum ResultStatus {
    /// The event resulted in a success.
    /// Contains an event id and size.
    /// The event id is used to update the status of a pending event.
    /// The size value indicates the file size downloaded in bytes.
    #[display("Success")]
    Success(uuid::Uuid, u64),
    /// The event resulted in an error.
    /// Used for MneError and ReqwestError outcomes.
    /// Contains and event id.
    /// The event id is used to update the status of a pending event.
    #[display("Error")]
    Error(uuid::Uuid),
    /// The task was skipped.
    /// Used when the download path exists and overwrite is false.
    /// Contains and event id.
    #[display("Pass")]
    Pass(uuid::Uuid),
    /// The event has not yet received a status update from the listener.
    /// New events begin in this state.
    Pending,
    /// A signal that the remaining queue of requests should be aborted.
    /// Used when the user hits the API rate limit of the BEA server and becomes locked out for one
    /// hour.
    /// Receiving this signal as a user indicates a bug in the rate limiting provided by the
    /// [`Tracker`](crate::Tracker) type.
    Abort,
}

impl FromStr for ResultStatus {
    type Err = JsonParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let status = match s {
            "Success" => Self::Success(uuid::Uuid::new_v4(), 0),
            "Error" => Self::Error(uuid::Uuid::new_v4()),
            "Pass" => Self::Pass(uuid::Uuid::new_v4()),
            "Pending" => Self::Pending,
            "Abort" => Self::Abort,
            _ => {
                let error = KeyMissing::new(s.to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
        };
        Ok(status)
    }
}
