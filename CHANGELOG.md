# Changelog

All notable changes to this project will be documented in this file.

## [0.1.13] - 2025-09-29

### 🚀 Features

- `Measure` enum added for variants of the Classification Unit parameter in the FixedAssets and Iip datasets.
- `IipDatum` updated to use the `Measure` type for the cl_unit field.
- `FixedAssetDatum` updated to use the `Measure` type in the *cl_unit* field.
- Methods added to `FixedAssedData` for retrieving value sets for each field in the struct.
- `FixedAssetCodes` type added to facilitate testing intersection of value sets between source data and target parameters.
- `params` and `new` methods added to `FixedAssets`, and a TryFrom impl added from `ParameterValueTable`.
- The *table_name* field is now a `FixedAssetTable` instead of a `TableName` for cleaner comparison between keys and values.
- `GdpTables` iterator introduced for simpler code and clearer intent, and linked to the `iter_tables` method in `GdpByIndustry`.  Swapped `iter_tables` for `iter` at call site in the *request.rs*.
- Download option added to CLI.
- `Classification` type added for the *Classification* parameter in the MNE dataset.
- `FromStr` and `Display` impls added to `Footnotes` types. A `description` method added linking the variant to the BEA description.
- Methods *description*, *key* and *source* added to `DirectionKind` type to enable it to serve independently to represent the *DirectionOfInvestment* parameter in the MNE dataset.
- Method *params* added to `OwnershipLevel` and `AffiliateLevel` to streamline API call generation in the MNE dataset.
- `MneDoi` type replaced with `Classification` for the *Classification* parameter. `Footnotes` type replaces `BoolOptions` for the *GetFootnotes* parameter in the MNE dataset.  `MneIterator` type replaced by separate `MneIter` and `AmneIter` types.
- `bears_health` added as dependecy to `bears` cli.
- `Queue` variant added to `Action` enum for the user CLI.
- Queue inspection added to CLI, allowing the user to dump the queue contents to a JSON file in the BEA_DATA directory.
- Variant added to `NaicsInputOutput` type to accommodate Hospitals and Nursing and Residential Care Facilities (622HO) in the GdpByIndustry dataset.
- `GdpByIndustry` now maps industries to the `Naics` type.  Methods *industries* and *years* added to produce value sets of the *industry* and *year* fields.
- Migrate the `write_json` function from `bears_health` to `bears_species`.  Dependency of `bears` on `bear_health` removed.
- HashMap updated to BTreeMap in `GdpByIndustry`.
- Dependencies `derive-getters` and `derive-new` added to bears_health.
- `UnderlyingGdpByIndustry` removed and `GdpByIndustry` generalized to handle both cases by adding the `dataset` field to track the origin dataset.
- `init_queue` function updated to use the `GdpByIndustry` type for the `UnderlyingGDPbyIndustry` dataset.
- `ValueSet` updated to use the `GdpByIndustry` type for the `UnderlyingGDPbyIndustry` dataset.
- Variants added to `ParameterName` to accommodate parameter names in the `value_keys` testing suite.
- `Integer` updated to wrap the *i64* type.
- `GdpByIndustry::table_ids` method updated to use the i64 type.
- `IoCodes` type removed from `bears_species` crate.
- `FixedAssetCodes` removed from the `bears_species` crate.
- Value set methods added to `Iip` for the `value_sets` testing suite.
- Files in the `key_sets` module updated follwing the `_set` naming convention.
- `FixedAssetKeys` added to facilitate testing of key sets for the `FixedAssets` dataset.
- `GdpKeys` type added to facilitate testing of the `GdpByIndustry` and `UnderlyingGdpByIndustry` datasets.
- 'value_sets' module renamed to 'key_sets' for greater clarity of intent.
- `IipKeys` type added to facilitate testing of key sets in the `Iip` dataset.
- `IoKeys` added to facilitate testing of key sets in the `InputOutput` dataset.
- Tests for the `Iip` dataset moved to `key_sets` module.
- `difference` and `params` functions added to `helpers` module for comparing key sets.
- Testing suite updated with methods for comparing key sets.
- Obligation to call `dotenvy` moved outside the `trace_init` method.
- The `table_id` field now accepts the i64 type instead of i32/

### 🐛 Bug Fixes

- Dev-dependency reference removed from workspace.
- Key for all values corrected in the *Frequency* parameter of the MNE dataset.
- Query fix for calls the `Iip` dataset.

### 🚜 Refactor

- Unused `YearSelection` type removed.
- Iteration of the `GdpByIndustry` type moved to `iter_tables` method.  Previous implementation of Iterator for GdpByIndustry removed.
- The *iter_mne* and *iter_amne* methods have replaced the *iter* method for generating the API call queue of the MNE dataset.
- `Cli` struct moved to `command` module.  Details of CLI actions moved to `Action` method *act*.  `bears_health` removed as dependency for `bears` cli.
- Value set tests moved to the `value_sets` module.
- Contents of `industry_codes` module moved to `gdp_codes` in `value_sets`.

### 📚 Documentation

- Doc comments added to undocumented methods.

### 🧪 Testing

- Value set tests added for the `FixedAssets` dataset.
- Tests for value sets updated to use `FixedAssetTable` type instead of `TableName`.
- Testing suites updated to use the `GdpByIndustry` type for the `UnderlyingGDPbyIndustry` dataset.

### ⚙️ Miscellaneous Tasks

- Updates to dependencies `bytesize`, `console`, `serde` and `serde_json`.
- Tracing instrumentation added to `FixedAssets` methods.
- Spacing added for code clarity.
- Version updated to 0.1.13.
- Patch updates to `anyhow`, `clap` and `serde` dependencies.
- Public visibility added to the `Classification` type.
- Visibility adjusted, adding `MneIter` and `AmneIter` types, and removing unused iterator types.
- Visibility adjustments, adding new types to the root level and removing unused types.
- Visibility updates.
- Patch update to the `serde` dependency.
- Patch update to the `serde` dependency.
- Visibility updated to remove deleted types.
- Mod file updated with new types.
- Visibility updated for new types and deleted functions.
- Updates to justfile, trying out --workspace.

## [0.1.12] - 2025-09-08

### 🚀 Features

- `Affiliation` type added for the IntlServTrade dataset.
- `Service` type added for the IntlServTrade dataset.
- `Trade` enum added for the IntlServTrade dataset.
- `Display` implemented for `Frequency` enum.
- Methods added to `GdpData` and `UnderlyingGdpData` types to produce sets of unique values amongs fields within its wrapped vector of data.
- Methods added to `IipData` type for producing sets of unique field values in the wrapped vector.
- `cl_units` and `components` methods added to `IipData` to produce unique sets of values from the wrapped vector.
- `FromStr` implemented for `Naics`.  A `variants` method added to create a vector of all variants.
- `into_inner` method added to `InputOutputCode` to facilitate accessing the wrapped `Naics` value inside map closures.
- Methods added to `InputOutput` and `InputOutputData` to produce sets of unique variants for each struct field of the contained datum.
- Additional methods added to `Iip` and `IipData` types for producing unique sets of values contained in the wrapped struct fields.  HashMaps and Sets converted to BTreeSets for sorting.
- `Debug` impl bound added to *path* parameter to enable tracing instrumentation.
- `frequencies` and `table_ids` methods added to `GdpData` for retrieving the value sets of the wrapped datum.
- `GdpCodes` type added to facilitate unit testing.
- Testing endpoints added for `Iip`, `InputOutput` value sets, `health_check` method added as a convenience testing suite.
- `jiff` dependency added, `anyhow` updated.
- `params` convenience method added to `Investment` type for producing API calls.
- `params` method added to `TableName` type to facilitate generating API calls from the types.
- `FixedAssetTable` type added to represent fixed assets table names in the `FixedAssets` dataset.
- `table_names` method added to `FixedAssets` for producing the value set of table names.
- `Command` type updated with Load and NextError variants.
- Load and NextError commands added to CLI.

### 🚜 Refactor

- `NipaDatum` and `NipaData` moved from `data` module to `nipa` module in `key_sets`.
- Tracing instrumentation added to value set methods.
- HashMap and Sets replaced with BTreeSet for ordering, and easier comparison.
- Noisy log lowered to `TRACE` level.
- `IipIterator` replaced with `IipInvestment` type to simply the code and convey clearer intent.
- `FixedAssetsIterator` replaced with `FixedAssetsTables` type to clarify intent and simplify design.
- `FixedAssetDatum` and `FixedAssetData` types moved to `fixed_assets` within the `key_sets` module.
- Minor polish.

### 📚 Documentation

- Justfile updated with CLI test suite for loading datasets.

### 🧪 Testing

- Units tests added for `Iip` data comparing unique values in source data to variants of parameter keys or classification enums.
- Updateds to value set test suites for the `GDPbyIndustry` and `UnderlyingGDPbyIndustry` datasets.
- Value set comparisons for the `InputOutput` dataset updated to compare parameter keys, as well as additional struct fields within the wrapped data.

### ⚙️ Miscellaneous Tasks

- Version incremented to 0.1.12.  Patch update to `clap` dependency.
- Visibility updated for `Affiliation`, `Trade` and `Service` enums.
- Cargo lock updated.
- Visibility updated for new types and some restructuring.
- Visibility updated for new tests.
- Dist init run for new version of cargo dist.

## [0.1.11] - 2025-09-02

### 🚀 Features

- Variants added to `AreaOrCountry` type to account for RowCode numbers.  A `from_code` method added to facilitate conversion.
- `NaicsSupplement` type added to account for international MNE row codes that do not conform to 2022 NAICS codes.  A corresponding `Supplement` variant added to `Naics` type.
- `RowCode` now maps to `Naics` type instead of i64, and `AreaOrCountry` instead of String.
- Variants added to the `NaicsSupplement` type to accommodate MNE data.
- `from_code` method added to `StateKind` type to facilitate conversion to the `RowCode` type.
- `NaicsInputOutput` type added to accommodate InputOutput data, as well as the InputOutput variant of the `Naics` enum.
- `State` variant added to the `RowCode` enum to facilitate MNE data conversion.
- `params` method added to the `InputOutputTable` type for producing parameters for BEA API calls.
- `InputOutputCode` type added to represent row and column codes in the InputOutput dataset.
- Default implemented where missing, even if arbitrarily.
- Variants added to `AreaOrCountry` enum to accommodate MNE data conversion.
- The *investment* field of the IIP dataset now maps to the `Investment` type.
- `InputOutputIterator`, `InputOutputDatum` and `InputOutputData` types added for the InputOutput dataset.
- `Iip` and `InputOutput` variants added to the `Data` type for their eponymous datasets.
- `Iip` and `InputOutput` variants added to match statements for parsing `Results` from json.
- `Iip`, `InputOutput` and `UnderlyingGDPbyIndustry` variants added to match statement in the `App::destination` method.
- `Iip`, `InputOutput` and `UnderlyingGDPbyIndustry` variants added to queue construction in the `Request` type.
- Readme updated with `Iip`, `InputOutput` and new `MNE` totals.
- `UnderlyingGdpDatum` and `UnderlyingGdpData` types added to the *gdp_by_industry* file in the `key_sets` module.
- `UnderlyingGdpData` placed in `UnderlyingGdp` variant of `Data` enum.
- `UnderlyingGDPbyIndustry` variant of `Dataset` added to parsing options for the `Results` type.
- Arbitrary default added to the `NaicsSector` type.
- Variants added to the `NaicsInputOutput` type to facilitate conversion of UnderlyingGDPbyIndustry data.
- Industry codes converted from integers to `Naics` type for the GDPbyIndustry and UnderlyingG
- `write_json` utility function added to write longer outputs to file in JSON format.
- Variants added to `NaicsSupplement` for conversion of UnderlyingGDPbyIndustry data.
- Variants added to `NaicsInputOutput` type for conversion of UnderlyingGDPbyIndustry data.

### 💼 Other

- Dataset name rotation for active testing.  Eventually need to move some of this to the CLI.

### 🚜 Refactor

- Call to `dotenv` moved to within the `bea_data` function to reduce code duplication.
- `GdpDatum` and `GdpData` types moved to `gdp_by_industry` file in `key_sets` module.
- Tests updated to use the `write_json` method, reducing code duplication.
- Industry codes in `UnderlyingGdpDatum` mapped to `Naics` type instead of integers.

### 📚 Documentation

- Tables in readme.md updated with UnderlyingGDPbyIndustry data.

### 🧪 Testing

- Test added to pull row and column codes from InputOutput data.
- Test added to verify that each field of json data containing parameter values has a corresponding variant in the `Investment` enum.
- `Iip` and `InputOutput` datasets added to the test rotation.
- Tests added to expose new additions to the testing harness.
- `industry_codes` test added to produce key:value pairs of industry codes and names from the `GDPbyIndustry` and `UnderlyingGDPbyIndustry` datasets.
- `check_industry_codes` test added to verify that industry codes in the `GDPbyIndustry` and `UnderlyingGDPbyIndustry` datasets are present as variants in the `Naics` enum.
- List of datasets updated in unit tests, functionality that should move to the CLI.

### ⚙️ Miscellaneous Tasks

- Version incremented to 0.1.11.
- `clap` dependency updated.
- Patch updates to dependencies `serde_json` and `tokio`.
- RowCode traces update to DEBUG level for work on the `NaicsSupplement` type.
- Clippy fixes for Rust 1.89 update.
- Updates to `clap`, `reqwest` and other dependencies.
- Patch updates to dependencies `clap`, `serde_json` and `url`.
- Clippy fixes for implied Iterator lifetimes.
- Visibility updates, adding new types and incorporating a name change.
- Info-level tracing downgraded to trace-level due to established stability.
- Mod file updated with new types for visibility.
- Lib file updated with new modules and types.  Visibility for all library types is at the root level (crate::type).
- Lib file udpated with new module names and types.
- `UnderlyingGdpDatum` and `UnderlyingGdpData` types given public visibility at the root level of the crate.
- Patch update to dependency `tracing-subscriber`.
- Cargo.toml for `bears_ecology` updated to include `serde` for use of the `Serialize` trait in the `write_json` function.

## [0.1.10] - 2025-07-30

### 🚀 Features

- `Scope` and `Overwrite` types added to module to represent parameter options for `Queue` methods.
- `Overwrite` and `Scope` added to parameter arguments for `Queue` methods.
- `next_mne_error` added to testing suite to retry to first of a presumably long list of errors, intended for use on TRACE.
- `Naics` type added to module.

### 🐛 Bug Fixes

- Clippy fixes and some commented out WIP.
- Temporary down-throttle of the rate limiting for testing.
- Corrections to 2022 NAICS codes.
- Edge case handled where a single item causes the list to serialize as a `serde_json::Value::Object` instead of a `serde_json::Value::Array`.
- Corrections to 2022 Naics codes incorporated into Naics types.
- Special cases added for MNE tables.

### 📚 Documentation

- Doc comments added to justfile.

### 🎨 Styling

- Clarify reporting on the `FromStrError` type.
- Minor refactor of tracing subscriber filter.

### 🧪 Testing

- Flux in unit tests related to bulk dataset downloads.
- Queue tests updated with `Overwrite` and `Scope` arguments.

### ⚙️ Miscellaneous Tasks

- Increment version to 0.1.10.
- Patch update to derive_setters dependency.
- Dependency updates for clap, console, jiff, reqwest and tokio.
- Patch updates to dependency `clap` and dev-dependency `anyhow`.
- `rand` added as dependency.  Patch updates to `clap`, `serde_json`, `strum` and `tokio`.
- `rand` added as dependency.
- Types updated for public visibility.
- Naics codes table added back in for backwards compability until the transition to full Naics types is complete.
- Cargo-dist update.

## [0.1.9] - 2025-05-12

### 🚀 Features

- NaicsIndustry enum added to represent the full six-digit NAICS codes.

### 🐛 Bug Fixes

- Typo corrections in the reference file for NAICS codes.

### 🧪 Testing

- Check_naics_industry added to check_naics test suite.
- Check_naics_industry added to check_naics test suite.

### ⚙️ Miscellaneous Tasks

- Version incremented to 0.1.9.
- NaicsIndustry enum added to public visibility in lib.rs and mod.rs.
- Minor update to tokio and patch updates to clap and jiff dependencies.
- Locked flag added to cargo-dist and cargo-release installs.
- `git-cliff` added to justfile.

## [0.1.8] - 2025-04-14

### 🚀 Features

- The NaicsSector::from_key method is now NaicsSector::from_code.
- Added the `NaicsSubcategory` type to represent Naics industry codes with a length of five digits.

### 🐛 Bug Fixes

- NaicsSubcategory variants and method values corrected in response to unit testing.
- `NaicsCategory` variants and methods corrected in response to unit testing.

### 🚜 Refactor

- Naics.rs and naics_codes.csv removed.
- Naics codes by classification added to the `cave` directory of `bears_health`.
- NaicsItem moved to the `keys` module of `bears_species`.
- Data module updated to use the new name for NaicsItems.

### 🧪 Testing

- Verification testings for Naics types added validating variant names, description and codes against the .csv files in the cave.

### ⚙️ Miscellaneous Tasks

- Increment version to 0.1.8.
- Rust fmt changes.
- Naics types added to mod and lib files.
- Clippy corrected `manual implementation of ok` corrected to use the ok method.

## [0.1.7] - 2025-04-07

### 🚀 Features

- Component enum added for the IIP dataset.
- InputOutputTable type added to keys module of bears_species to provide variants for keys in the InputOutput dataset.
- `AocSta` type added for AreaOrCountry keys in the IntlServSTA dataset.
- Added the `Channel` enum to represent valid keys for the Channel parameter of the IntlServSTA dataset.
- Added the `IipIndustry` enum to represent valid keys for the Industry parameter of the IntlServSTA dataset.
- Added the `NaicsSector`, `NaicsSubsector` and `NaicsCategory` enums to represent different levels of NAICS code categories.
- Command, ValueSet and ValueSets types stubbed out for future use.

### 🐛 Bug Fixes

- Explicit version numbers added to workspace members.

### 🚜 Refactor

- Library structure changed from crate to workspace.
- Library types moved to bears_species crate.
- Core library functionality moved to the bears_ecology crate.
- Tests moved to the bears_health crate.
- Stub cli moved to the eponymous bears crate.
- Tracker type moved from queue to tracker module.
- Iip key set moved to the key_sets module of the bears_species crate.
- ApiMetada moved to the key_sets module of the bears_species crate.
- InputOutput key set moved to the key_sets module of the bears_ecology crate.
- `IntlServSta`, `IntlServTrade` and `Regional` key sets moved to the key_sets module of the bears_species crate.

### 📚 Documentation

- Descriptions added for member crates.

### 🧪 Testing

- Unused tests for json removed.
- Tests added for `Component` and `AocSta` types validating enum variants against the BEA response.

### ⚙️ Miscellaneous Tasks

- Version incremented to 0.1.7.
- Patch update to the `clap` dependency.
- Unused json and validate modules removed.
- Patch update to the `tokio` dependency.
- Recipe marked as invalid for workspace in justfile.

## [0.1.6] - 2025-03-31

### 🚀 Features

- `ParameterNameMissing` error moved to error location, some metrics added to the type.
- `NipaTableName` enum added with variants for NIPA table name keys.

### 🚜 Refactor

- Location of dataset keys moved from `value` folder to `keys` and `key_sets`.
- Local errors moved from `error` module to the location of origin.
- Local errors moved from the `error` module to the `free` module.
- Local errors moved from the `error` module to the `app` module.
- Dataset key types moved to the `keys` module.
- Dataset key sets moved to the `key_sets` module.
- `lib.rs` updated to reflect name and module changes.

### 🧪 Testing

- Refactor for changes to Nipa key set.

### ⚙️ Miscellaneous Tasks

- Patch update to the `clap` dependency.
- Justfile updated for publishing workflow.
- Version incremented to 0.1.6.

## [0.1.5] - 2025-03-23

### 🚀 Features

- `Indicator` enum added with variants corresponding to valid values for the Indicator parameter.
- `AreaOrCountry` enum added with variants for valid values of the "AreaOrCountry" parameter in BEA requests.
- `ItaFrequency` and `ItaFrequencies` types added to represent valid values for the "Frequency" parameter of the ITA dataset in BEA requests.
- `SelectionSet` enum added as an alternative abstraction to the `SelectionKind` enum.
- `Frequency::all` method added to generate valid request parameters for all frequencies.  Only implemented for a subset of datasets.
- `ItaDatum` and `ItaData` types added to accomodate data responses for the ITA dataset.  Iterator implemented for the `Ita` type.  `Ita::queue` and `Ita::iter` methods added.
- `ITA` variant added to the TryFrom impl for `Results`.
- Convenience method `ParameterValueTable::parameter_fields` added to provide access to the value contained within the variant.
- Added the `Queue::load_par` method to enable parallel loading through the `rayon` library.
- The `Dataset::initial_load_par` method implements parallel queue loading for benchmarking purposes.

### 🚜 Refactor

- Internal macro impl_json_to_bea_error! added to facilitate conversion of JSON parsing errors to the umbrella internal error type.
- Convenience function `result_to_data` added to convert a `serde_json::Value` representing a BEA result, to a `serde_json::Value` representing the `Data` portion of the `Result`.  Steps a couple levels down the JSON value tree, preventing rightward drift in the caller.
- `lib.rs` updated with added types.
- Added `check_indicators` to `mod.rs` to provide access to unit tests.
- ITA variant added to the `download_summary` function.

### 📚 Documentation

- Comments added to the `check_datasets` test.
- README.md updated with ITA stats.

### 🧪 Testing

- The `check_indicators` test ensures all values in the BEA response parse to valid Indicator variants.
- Added `check_indicators` to unit tests.
- Data download tests temporarily refocued on ITA for active development.

### ⚙️ Miscellaneous Tasks

- Incremental version to 0.1.4 in Cargo.toml.
- Patch updates to `jiff` and `reqwest` dependencies.
- Noise trimmed from tracing logs.
- `mod.rs` updated to include added types.
- ITA variant for `Dataset` added to path generation handling in `App::destination` method.
- Loading benchmark added to test parallel loading with `rayon`.
- Update to convenience commands in the `justfile`.
- Changelog updated for version 0.1.5.

## [0.1.4] - 2025-03-15

### 🚀 Features

- 'Style' type added to facilitate progress bars drawn to the console.
- Method `from_value` added to `Frequency` to provide a canonical means of interpreting BEA parameter values.
- Added the `roman_numeral_quarter` function mapping Roman Numeral values in the `Quarter` field to the `jiff::civil::Date` type.
- `GdpData` type added for the GDPbyIndustry dataset.  Companion variant `Data::GdpData` added.
- Progress bar added to `Queue` file loading.
- Progress bars added to `History` load and download methods.

### 🚜 Refactor

- Various console logs lowered from Info to Trace, now that the functions of interest are more stable.
- `MneDiData` and `FixedAssetsData` added to the root namespace following the library convention.

### 📚 Documentation

- Description added for the `roman_numeral_quarter` function.
- Descriptions added to `Frequency` methods.
- Module and function level descriptions added the `check` module.
- Progress statistics updated in the root `README.md`.

### 🧪 Testing

- Additional troubleshooting tests added for loading GDPbyIndustry files.
- Duplicate test removed.

### ⚙️ Miscellaneous Tasks

- Changelog updated for version 0.1.3.
- Version incremented to 0.1.4 in Cargo.toml.
- Changelog action added to justfile.
- *(dependency)* Patch updates for dependencies.  Bincode dependency removed.
- Bincode removed from internal tests.
- Benchmarks updated to use the 'Style' type.
- *(dependency)* Patch update applied to `tokio`.  Minor update applied to `uuid`.  No changes required to code.
- Deletion of dead legacy code.
- Changelog updated for version 0.1.4.

## [0.1.3] - 2025-03-03

### 🚀 Features

- Methods `initial_load`, `initial_load_continued`, `retry_load` and `download_with_history` added to the `Dataset` type.
- Method variants for `with_events` added to the Queue and History types to facilitate benchmarking.
- GdpDatum type added to represent return values for the `Dataset::GDPbyIndustry` variant.
- Download support added to GdpByIndustry via the GdpByIndustryIterator type.

### 🐛 Bug Fixes

- Note variant added to the Addendum type.
- The MneError type now recognizes multiple error codes returned in an array.
- The size limiting check no longer prevents users from requesting files larger than the 100MB limit.
- Logical bug patched in the impl for `Chunks` from `History`.  The inner vector will now include the final `Chunk`.
- Off-by-one logical error fixed for generating frequency parameters from a list.

### 🚜 Refactor

- The Error cap has been raised to 29, and the Call cap to 89, since size tracking is now enabled.
- The History::contains method has been removed in favor of calling `contains_key` directly on the inner BTreeMap.
- Streamlined error handling for JsonParseError variants.

### 📚 Documentation

- Method descriptions added to the `dataset` module.
- Descriptions added for `History` methods.

### 🧪 Testing

- Coverage added for the `History` methods `initial_load`, `initial_load_continued`, `retry_load` and `download_with_history`.
- Benchmarking added to the `with_event` family of methods for the Queue and Chunks types.

### ⚙️ Miscellaneous Tasks

- Increment version to 0.1.3 in Cargo.toml.

## [0.1.2] - 2025-02-23

### 🐛 Bug Fixes

- Missing annotations added to Annotation type.  Missing row codes added to row_code module.

### 🚜 Refactor

- ParameterKind added to organize parameter headers.
- BTreeKeyMissing error added for cases when the expected key is not present in internal BTreeMap structures, such as `Options` for `App` types.
- Tests for data load and download in the `check` module remain a work in progress.

### 📚 Documentation

- Description added to Options type.
- Link to playground added.  Examples do not yet work, as `bears` is not a recognized library.

### ⚙️ Miscellaneous Tasks

- Cargo-audit and OmniBOR supply chain security tooling added to release build artifacts.
- Migrate to 2024 Edition.  Patch updates for dependencies.
- Patch update for dependency uuid.
- Changelog updated for version 0.1.2.
- Incremented version to 0.1.2.
- Cargo.lock updated.

## [0.1.1] - 2025-02-17

### 🚀 Features

- TryFrom<(Dataset, Mode)> implemented for History.  This is an internal convenience function used to direct the program to the correct log file based on the given context.
- Support added for deriving a Queue from a log history.
- Size tracking from History added to App and Queue methods.
- Load and download methods added to Chunks, wrapping the load and download methods from Queue.
- The path of NIPA datasets now indicate whether ShowMillions is present appending `_millions` to the file path.
- The History::summary method now includes the total size of successful downloads.
- The bea_data function has replaced use of the EnvError::from_env method, which has been removed.
- Added the impl_json_parse_error! macro to streamline construction of the JsonParseError type.

### 🐛 Bug Fixes

- The map_to_int method now strips commas from strings.

### 🚜 Refactor

- Load and download logs now divert to the `history` folder in the `BEA_DATA` directory.
- NipaIterator updated to use the SelectionKind type.
- The successes and errors methods have been reworked.  The `from_file` method has been renamed to `from_env`.
- The History::from_env method now looks for the `history.log` file in the `history` folder of the `BEA_DATA` directory.
- Clearer flow control within iterators for Nipa and NiUnderlyingDetail.
- Tracing statements added to the read_json method to aid in debugging during deserialization.
- Control flow improved for MneIterator and FixedAssetIterator.  The YearSelection type has been removed in favor of the SelectionKind enum.  The date_by_period function has replaced the quarter function for mapping time periods to the jiff::civil::Date type.
- The standard App::with_options method generated by derive_getters has been removed.  The App::add_options function has been renamed to with_options. Now only one way to set the value for options, and it ensures the query parameters have been properly updated, relieving the burden from the user.
- Pollster removed from dev-dependencies.

### 📚 Documentation

- Bears logo art added to README.
- Can't quite figure out the changelog tooling.
- Preamble and methods documentation added for the App type.

### ⚙️ Miscellaneous Tasks

- Incremented version to 0.1.1.
- Incremented version to 0.1.1.
- Tests updated to reflect API changes.
- Strum dependency updated to 0.27.0.
- Changelog synced to proper release tag.
- Jiff updated to 0.2.0.  Patch updates for other dependencies.
- Cargo dist tool added to CI.
- Changelog updated for version 0.1.1.

## [0.1.0] - 2025-02-08

### 🚀 Features

- Download queue
- Impl TryFrom<&PathBuf> for NipaData
- Async implementations of the load and download methods for Queue. feat: Iterator implementation for the MNE dataset. feat: Structured logging added to loading and downloading actions.
- Configuration file for git-cliff added. chore: Dependency updates.
- README added to the Github landing page.
- Markdown lint file added to suppress spurious warnings in doc markdown.

### 🚜 Refactor

- Remove dead code from prototype period.
- Match statements for error constructions updated to use the map_err method instead.
- The error_info! macro has been removed, leaving no macros in the public API.
- Amendment to error_info! removal.
- Trimmed unused dependencies.

### 📚 Documentation

- Bears logo art added to data folder.
- Crate metadata added to Cargo.toml.
- Category slugs adjusted in Cargo.toml.
- Keywords adjusted in Cargo.toml.

### ⚙️ Miscellaneous Tasks

- Dependency updates.

<!-- generated by git-cliff -->
