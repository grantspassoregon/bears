//! The `check` module contains unit and integration tests for the library.
//!
//! The `bears` library constructs collections of valid parameter values by downloading the
//! associated files from the BEA server into the `BEA_DATA` directory.  Set the value of the
//! `BEA_DATA` environmental variable in a `.env` file at the project root.
//!
//! In order to construct API requests, we first must download the range of valid parameter values
//! for each dataset.  The minimum set of steps to gather the valid parameter values is:
//!
//! ```no_run
//! use bears::check;
//!
//! // Download valid parameter values into the BEA_DATA directory.
//! fn main() -> Result<(), bears::BeaErr> {
//!     // Get a list of available datasets.
//!     check::datasets_to_json()?;
//!     // Get the parameter names for each dataset.
//!     check::parameters_to_json()?;
//!     // Get the valid values for each parameter name.
//!     check::parameter_values_to_json()?;
//!     // Get the valid values for GDPbyIndustry filtered by table id.
//!     check::values_gdp_filtered()?;
//!     // Get the valid values for UnderlyingGDPbyIndustry filtered by table id.
//!     check::values_ugdp_filtered()?;
//!     Ok(())
//! }
//! ```
//!
//! Once the range of valid parameter values is available in the `BEA_DATA` directory, you can
//! download the data for each dataset into the `data` folder.
//!
//! ```no_run
//! use bears::check;
//!
//! // Download data for each dataset into the `data` folder of the `BEA_DATA` directory.
//! fn main() -> Result<(), bears::BeaErr> {
//!     // No pre-existing download history.  Try every combo and see what is avialable.
//!     check::datasets_download_initial()?;
//!     // A download history exists.  Use it to download successes and ignore calls that will fail.
//!     check::datasets_download_with_history()?;
//!     Ok(())
//! }
//! ```
//!
//! ## Submodules
//!
//! The `check` module is divided into a series of private submodules, documented here:
//!
//! ### Datasets
//!
//! The parameter values for datasets indicate the valid range of dataset names.
//! There are three unit tests associated with datasets:
//!
//! * [`datasets_to_json`] downloads the valid values and saves the response to `datasets.json`
//!   using the `GetDataSetList` BEA method.
//! * [`datasets_from_file`] deserializes `datasets.json` into the [`Dataset`](crate::Dataset)
//!   type.
//! * [`check_datasets`] validates that all returned dataset values have corresponding variants in
//!   the [`Dataset`](crate::Dataset) type.
//!
//! Running `datasets_to_json` is sufficient to provide the program with the information necessary
//! to construct API calls.  The `datasets_from_file` test confirms that deserialization will
//! succeed, and remains in the testing suite to detect any regression.  The `check_datasets`
//! test will fail if the BEA adds any new datasets to their API, providing an alert when the
//! `bears` library has become out of date.
//!
//! ### Parameters
//!
//! The BEA `GetParameterList` method contains the set of parameter names associated with each dataset.
//! Used to further query valid values for each name using the `GetParameterValues` or
//! `GetParameterValuesFiltered` method.
//!
//! There are three unit tests for the `GetParameterList` method:
//!
//! * [`parameters_to_json`] downloads valid values for each dataset, saving the responses to the
//!   `parameters` folder in the `BEA_DATA` directory.
//! * [`parameters_from_file`] deserializes the JSON files in the `parameters` folder in the
//!   [`BeaResponse`](crate::BeaResponse) type.
//! * [`parameter_names`] converts each [`ParameterName`](crate::ParameterName) variant into a
//!   string slice using the `to_string` implementation, then from a string slice back into a
//!   `ParameterName` using the `FromStr` implementation.
//!
//! The `parameters_from_file` test confirms that
//! deserialization will succeed, and remains in the testing suite to detect any regression.
//!
//! ### Parameter Values
//!
//! These tests obtain valid values for parameter names in a given dataset using the BEA `GetParameterValues`
//! method.
//!
//! Primary tests for this suite:
//!
//! * [`parameter_values_to_json`] downloads valid values for each dataset, saving the responses to
//!   the `parameter_values` folder of the `BEA_DATA` directory.
//! * [`parameter_values_from_file`] deserializes the JSON files in the `parameter_values` folder
//!   of the `BEA_DATA` directory into the [`BeaResponse`](crate::BeaResponse) type.
//!
//! ### Values
//!
//! The `values` suite contains additional tests for the `GetParameterValues` and
//! `GetParameterValuesFiltered` methods. `values` is an admittedly bad names.  Most of the names
//! in the `check` module are problematic in that they simply add an "s" to the end of an existing
//! type module, making typos surprisingly easy.
//!
//! Key tests in the `values` suite:
//!
//! * [`values_filtered_subset'] downloads the subset of parameter values confirmed return data.
//! * [`values_filtered`] attempts to download every parameter name.  Not all names are
//!   implemented, so some requests will not succeed.
//! * [`values_gdp_filtered`] downloads industry and year values for the GDPbyIndustry dataset filtered by table id.
//! * [`values_ugdp_filtered`] downloads industry and year values for the UnderlyingGDPbyIndustry dataset filtered by table id.
//!
//! The JSON files returned by the `values_filtered_subset` method contain the same information
//! produced by `parameter_values_to_json`.  We default to reading from the `parameter_values`
//! folder populated by the `parameter_values_to_json` function.
//!
//! The `values_gdp_filtered` function is required to construct API calls for the GDPbyIndustry
//! dataset, and the `values_ugdp_filtered` function is required to construct API calls for the
//! UnderlyingGDPbyIndustry dataset.
//!
//! In addition:
//!
//! * The [`api_error`] function deserializes a BEA API error as a
//!   [`BeaResponse`](crate::BeaResponse).  Present to detect regressions during testing.
//! * The [`requests_exceeded`] function deserializes a BEA requests exceeded API error as a
//!   [`BeaResponse`](crate::BeaResponse).  Present to detect regressions during testing.
//!
//! ### Data
//!
//! Key tests in the `data` suite:
//!
//! * [`datasets_download_initial`] downloads data for each dataset into the `data` folder of the
//!   `BEA_DATA` directory.  Tries every permutation of parameter values, including combinations that
//!   are not implemented.  An exploratory download used to discover new datasets.  The sizes of
//!   downloads in this method are unknown, so requests may exceed the 100MB per minute rate limit
//!   set by the BEA server.  At present the program will abort upon receiving a
//!   [`Results::RequestsExceeded`](crate::Results::RequestsExceeded) status.
//! * [`datasets_download_with_history`] downloads data for each dataset into the `data` folder of
//!   the `BEA_DATA` directory, using the download [`History`](crate::History) to select the subset
//!   of implemented endpoints from the set of possible API calls.  Since the targets have known
//!   size, the request rate is metered to prevent exceeding the 100MB per minute download limit
//!   set by the BEA server.
//! * [`datasets_initial_load`] loads all successful records from the download
//!   [`History`](crate::History). Used for initial reads when a load `History` is not available.
//! * [`datasets_initial_load_continued`] loads all successful records from the download
//!   [`Hisotry`](crate::History) that are not already present in the load `History`.
//! * [`datasets_retry_load`] attempts to reload failures in the load [`History`](crate::History).
//!
//! Use the `datasets_download_initial` and `datasets_download_with_history` functions to download
//! data.  The `datasets_initial_load*` and `datasets_retry_load` functions are used internally to
//! detect regressions during testing.
//!
//! In addition:
//!
//! * Use [`download_history`] to inspect a download history by printing it to the console.  Used
//!   during initial development and potential future schema changes.
//! * Use [`naics`] to confirm the `naics_codes.csv` file is present in the `BEA_DATA` directory
//!   and loads without issue.  Present to detect regressions during testing.
//!
//! ### Queues
//!
//! * [`inspect_queues`] generates the request queue for each dataset and reports its length to the
//!   console.  Used to verify that iterators generate sets of the expected length.
//!
//! ### Histories
//!
//! * [`download_summary`] prints summary statistics from the download history for each dataset.
//!   Used to generate the numbers for the progress report in the `readme.md` file.
mod aocs;
mod components;
mod data;
mod datasets;
mod histories;
mod iip;
mod indicators;
mod industry_codes;
mod investments;
mod naics;
mod parameter_values;
mod parameters;
mod queues;
mod value_sets;
mod values;

pub use aocs::check_aoc_sta;
pub use components::check_components;
pub use data::{
    data_from_json, data_to_json, datasets_download_initial, datasets_download_mne_initial,
    datasets_download_with_history, datasets_initial_load, datasets_initial_load_continued,
    datasets_retry_load, debug_gdpbyindustry, download_history, next_mne_error,
};
pub use datasets::{check_datasets, datasets_from_file, datasets_to_json};
pub use histories::download_summary;
pub use iip::iip_codes;
pub use indicators::check_indicators;
pub use industry_codes::{check_gdp_codes, gdp_codes};
pub use investments::check_investments;
pub use naics::{
    check_naics_category, check_naics_industry, check_naics_sectors, check_naics_subcategory,
    check_naics_subsectors,
};
pub use parameter_values::{
    parameter_value_filtered, parameter_value_from_json, parameter_values_from_file,
    parameter_values_to_json,
};
pub use parameters::{parameter_names, parameters_from_file, parameters_to_json};
pub use queues::inspect_queues;
pub use value_sets::{
    check_fa_codes, check_fa_keys, check_io_codes, check_io_keys, fa_codes, fa_keys, get_fa_codes,
    io_codes, io_keys,
};
pub use values::{
    api_error, requests_exceeded, values_filtered, values_filtered_subset, values_gdp_filtered,
    values_ugdp_filtered,
};
