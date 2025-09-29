use bears_ecology::{bea_data, trace_init};
use bears_species::Dataset;

#[test]
fn parameter_names() -> anyhow::Result<()> {
    bears_health::parameter_names()?;
    Ok(())
}

#[tokio::test]
async fn load_datasets() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    bears_health::datasets_to_json().await?;
    #[cfg(feature = "api")]
    tracing::info!("Response received from BEA API.");
    bears_health::datasets_from_file()?;
    tracing::info!("Datasets deserialized from file.");
    bears_health::check_datasets()?;
    tracing::info!("Checked against dataset variants.");
    Ok(())
}

#[tokio::test]
async fn parameters() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    bears_health::parameters_to_json().await?;
    #[cfg(feature = "api")]
    tracing::info!("Response received from BEA API.");
    bears_health::parameters_from_file()?;
    tracing::info!("Parameters deserialized from file.");
    Ok(())
}

#[tokio::test]
async fn parameter_values() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    bears_health::parameter_values_to_json().await?;
    #[cfg(feature = "api")]
    tracing::info!("Response received from BEA API.");
    bears_health::parameter_values_from_file()?;
    tracing::info!("Parameter values deserialized from file.");
    Ok(())
}

// try to retrieve parameter values for all datasets and all parameters
#[tokio::test]
#[cfg(feature = "api")]
async fn values_filtered_search() -> anyhow::Result<()> {
    bears_health::values_filtered().await?;
    Ok(())
}

// retreive parameter values with known valid responses
#[tokio::test]
#[cfg(feature = "api")]
async fn values_filtered_subset() -> anyhow::Result<()> {
    bears_health::values_filtered_subset().await?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "api")]
async fn values_gdp_filtered() -> anyhow::Result<()> {
    // bears_health::values_gdp_filtered().await?;
    // tracing::info!("Filtered values for GDP read.");
    bears_health::values_ugdp_filtered().await?;
    tracing::info!("Filtered values for Underlying GDP read.");
    Ok(())
}

#[test]
fn health_check() -> anyhow::Result<()> {
    check_indicators()?;
    check_investments()?;
    check_components()?;
    check_aoc_sta()?;
    check_naics()?;
    check_expected_keys()?;
    check_observed_keys()?;
    Ok(())
}

#[test]
fn check_indicators() -> anyhow::Result<()> {
    bears_health::check_indicators()?;
    Ok(())
}

#[test]
fn check_investments() -> anyhow::Result<()> {
    bears_health::check_investments()?;
    Ok(())
}

#[test]
fn check_components() -> anyhow::Result<()> {
    bears_health::check_components()?;
    Ok(())
}

#[test]
fn check_aoc_sta() -> anyhow::Result<()> {
    bears_health::check_aoc_sta()?;
    Ok(())
}

#[test]
fn check_naics() -> anyhow::Result<()> {
    bears_health::check_naics_sectors()?;
    bears_health::check_naics_subsectors()?;
    bears_health::check_naics_category()?;
    bears_health::check_naics_subcategory()?;
    bears_health::check_naics_industry()?;
    Ok(())
}

#[test]
fn print_expected_keys() -> anyhow::Result<()> {
    trace_init()?;
    let path = bea_data()?;
    bears_health::FixedAssetKeys::print_expected(&path)?;
    bears_health::GdpKeys::print_expected(&path, Dataset::GDPbyIndustry)?;
    bears_health::GdpKeys::print_expected(&path, Dataset::UnderlyingGDPbyIndustry)?;
    bears_health::IipKeys::print_expected(&path)?;
    bears_health::IoKeys::print_expected(&path)?;
    Ok(())
}

#[tokio::test]
async fn print_observed_keys() -> anyhow::Result<()> {
    trace_init()?;
    let path = bea_data()?;
    bears_health::FixedAssetKeys::print_observed(&path).await?;
    bears_health::GdpKeys::print_observed(&path, Dataset::GDPbyIndustry).await?;
    bears_health::GdpKeys::print_observed(&path, Dataset::UnderlyingGDPbyIndustry).await?;
    bears_health::IipKeys::print_observed(&path).await?;
    bears_health::IoKeys::print_observed(&path).await?;
    Ok(())
}

#[tokio::test]
async fn check_expected_keys() -> anyhow::Result<()> {
    trace_init()?;
    let path = bea_data()?;
    bears_health::FixedAssetKeys::check_expected(&path).await?;
    bears_health::GdpKeys::check_expected(&path, Dataset::GDPbyIndustry)?;
    bears_health::GdpKeys::check_expected(&path, Dataset::UnderlyingGDPbyIndustry)?;
    bears_health::IipKeys::check_expected(&path)?;
    bears_health::IoKeys::check_expected(&path)?;
    Ok(())
}

#[tokio::test]
async fn check_observed_keys() -> anyhow::Result<()> {
    trace_init()?;
    let path = bea_data()?;
    bears_health::FixedAssetKeys::check_observed(&path).await?;
    bears_health::GdpKeys::check_observed(&path, Dataset::GDPbyIndustry).await?;
    bears_health::GdpKeys::check_observed(&path, Dataset::UnderlyingGDPbyIndustry).await?;
    bears_health::IipKeys::check_observed(&path).await?;
    bears_health::IoKeys::check_observed(&path).await?;
    Ok(())
}

#[tokio::test]
async fn api_error() -> anyhow::Result<()> {
    bears_health::api_error()?;
    bears_health::requests_exceeded()?;
    Ok(())
}

#[tokio::test]
async fn inspect_queues() -> anyhow::Result<()> {
    bears_health::inspect_queues().await?;
    Ok(())
}

#[test]
fn debug_gdpbyindustry() -> anyhow::Result<()> {
    bears_health::debug_gdpbyindustry()?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "api")]
async fn datasets_download_initial() -> anyhow::Result<()> {
    bears_health::datasets_download_initial().await?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "api")]
async fn datasets_download_mne_initial() -> anyhow::Result<()> {
    bears_health::datasets_download_mne_initial().await?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "api")]
async fn datasets_download_with_history() -> anyhow::Result<()> {
    bears_health::datasets_download_with_history().await?;
    Ok(())
}

#[tokio::test]
async fn datasets_initial_load_start() -> anyhow::Result<()> {
    bears_health::datasets_initial_load().await?;
    Ok(())
}

#[tokio::test]
async fn datasets_initial_load_continued() -> anyhow::Result<()> {
    bears_health::datasets_initial_load_continued().await?;
    Ok(())
}

#[tokio::test]
async fn datasets_retry_load() -> anyhow::Result<()> {
    bears_health::datasets_retry_load().await?;
    Ok(())
}

#[test]
fn next_mne_error() -> anyhow::Result<()> {
    bears_health::next_mne_error()?;
    Ok(())
}

#[tokio::test]
async fn data_to_json() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    bears_health::data_to_json().await?;
    Ok(())
}

#[tokio::test]
async fn data_from_json() -> anyhow::Result<()> {
    bears_health::data_from_json().await?;
    Ok(())
}

#[test]
fn download_history() -> anyhow::Result<()> {
    bears_health::download_history()?;
    Ok(())
}

#[test]
fn download_summary() -> anyhow::Result<()> {
    bears_health::download_summary()?;
    Ok(())
}
