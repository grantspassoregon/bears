use crate::{App, Options};
use bears_species::{BeaErr, EnvError, IoError, UrlParseError};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn bea_data() -> Result<std::path::PathBuf, EnvError> {
    dotenvy::dotenv().ok();
    let key = "BEA_DATA".to_string();
    let path = std::env::var(&key)
        .map_err(|source| EnvError::new(key, source, line!(), file!().into()))?;
    Ok(std::path::PathBuf::from(&path))
}

/// Initiates a subscriber for the tracing library. Used to instrument internal library functions
/// for debugging and diagnostics.
#[tracing::instrument]
pub fn trace_init() -> Result<(), BeaErr> {
    let path = bea_data()?;
    let path = path.join("history");
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("History directory created.");
    }
    let path = path.join("history.log");
    let history = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let history = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(std::sync::Arc::new(history))
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            metadata.target() == "download_history" || metadata.target() == "load_history"
        }));
    let default_env =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "bea=info".into());
    if tracing_subscriber::registry()
        .with(default_env)
        .with(
            tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::filter::filter_fn(
                |metadata| {
                    metadata.target() != "download_history" && metadata.target() != "load_history"
                },
            )),
        )
        .with(history)
        .try_init()
        .is_ok()
    {};
    tracing::trace!("Loading Bea...");
    Ok(())
}

/// Helper function
/// Initiates logging
/// Reads environmental variables from .env
/// Creates an instance of App
#[tracing::instrument]
pub fn init() -> Result<App, BeaErr> {
    trace_init()?;
    tracing::info!("Test logging initialized.");
    dotenvy::dotenv().ok();
    let url = "BEA_URL".to_string();
    let url = std::env::var(&url)
        .map_err(|source| EnvError::new(url, source, line!(), file!().into()))?;
    let url = url::Url::parse(&url)
        .map_err(|source| UrlParseError::new(url, source, line!(), file!().into()))?;
    let key = "API_KEY".to_string();
    let key = std::env::var(&key)
        .map_err(|source| EnvError::new(key, source, line!(), file!().into()))?;
    let options = Options::default();
    let app = App::new(key, options, url);
    Ok(app)
}

pub fn file_size<P: AsRef<std::path::Path>>(path: P) -> Option<u64> {
    let path = path.as_ref();
    if path.exists() {
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => {
                return None;
            }
        };
        let metadata = match file.metadata() {
            Ok(data) => data,
            Err(_) => return None,
        };
        Some(metadata.len())
    } else {
        None
    }
}
