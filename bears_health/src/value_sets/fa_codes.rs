use bears_ecology::{bea_data, initial_load, trace_init};
use bears_species::{
    BeaErr, Data, Dataset, FixedAssetCodes, FixedAssetTable, FixedAssets, NipaRanges, write_json,
};
use strum::IntoEnumIterator;

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads InputOutput files, converts them to row and column codes.
/// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
/// `BEA_DATA` directory.
#[tracing::instrument]
pub async fn get_fa_codes() -> Result<FixedAssetCodes, BeaErr> {
    trace_init()?;
    let dataset = Dataset::FixedAssets;
    let data = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", data.len());
    let mut cl_units = std::collections::BTreeSet::new();
    let mut line_descriptions = std::collections::BTreeSet::new();
    let mut line_numbers = std::collections::BTreeSet::new();
    let mut metric_names = std::collections::BTreeSet::new();
    let mut series_codes = std::collections::BTreeSet::new();
    let mut table_names = std::collections::BTreeSet::new();
    let mut time_periods = std::collections::BTreeSet::new();
    let mut unit_mults = std::collections::BTreeSet::new();
    data.iter()
        .map(|v| {
            if let Data::FixedAssets(data) = v {
                cl_units.append(&mut data.cl_units());
                line_descriptions.append(&mut data.line_descriptions());
                line_numbers.append(&mut data.line_numbers());
                metric_names.append(&mut data.metric_names());
                series_codes.append(&mut data.series_codes());
                table_names.append(&mut data.table_names());
                time_periods.append(&mut data.time_periods());
                unit_mults.append(&mut data.unit_mults());
            }
        })
        .for_each(drop);

    let result = FixedAssetCodes::new(
        cl_units,
        line_descriptions,
        line_numbers,
        metric_names,
        series_codes,
        table_names,
        time_periods,
        unit_mults,
    );
    Ok(result)
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads InputOutput files, converts them to row and column codes.
/// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
/// `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn fa_codes() -> Result<(), BeaErr> {
    let codes = get_fa_codes().await?;
    let path = bea_data()?;
    let target = path.join("FixedAssets_ClUnit.json");
    write_json(codes.cl_units(), target)?;
    let target = path.join("FixedAssets_LineDescription.json");
    write_json(codes.line_descriptions(), target)?;
    let target = path.join("FixedAssets_LineNumber.json");
    write_json(codes.line_numbers(), target)?;
    let target = path.join("FixedAssets_MetricName.json");
    write_json(codes.metric_names(), target)?;
    let target = path.join("FixedAssets_SeriesCode.json");
    write_json(codes.series_codes(), target)?;
    let target = path.join("FixedAssets_TableName.json");
    write_json(codes.table_names(), target)?;
    let target = path.join("FixedAssets_TimePeriod.json");
    write_json(codes.time_periods(), target)?;
    let target = path.join("FixedAssets_UnitMult.json");
    write_json(codes.unit_mults(), target)?;
    Ok(())
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads FixedAssets files, converts them to value sets.
/// Checks that all tables are present as variants of the FixedAssetTable enum.
#[tracing::instrument]
pub async fn check_fa_codes() -> Result<(), BeaErr> {
    let dataset = Dataset::InputOutput;
    let codes = get_fa_codes().await?;
    // build struct field sets
    // set of variants for FixedAssetTable
    let tables = FixedAssetTable::iter().collect::<std::collections::BTreeSet<FixedAssetTable>>();
    // print missing tables to bea_data
    let source_tables = codes.table_names();
    let missing_tables = source_tables
        .difference(&tables)
        .collect::<std::collections::BTreeSet<&FixedAssetTable>>();
    if !missing_tables.is_empty() {
        let path = bea_data()?;
        let target = path.join(format!("{dataset}_TableName_Missing.json"));
        write_json(&missing_tables, target)?;
        tracing::error!(
            "Missing FixedAssetTable variants in {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!("All source tables in {dataset} are variants of the FixedAssetTable type.");
    }
    // Print unused variants of FixedAssetTable to bea_data "Unused"
    let unused_tables = tables
        .difference(codes.table_names())
        .collect::<std::collections::BTreeSet<&FixedAssetTable>>();
    if !unused_tables.is_empty() {
        let path = bea_data()?;
        let target = path.join(format!("{dataset}_TableName_Unused.json"));
        write_json(&unused_tables, target)?;
        tracing::error!(
            "Unused FixedAssetTable variants in {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!("All variants of the FixedAssetTable type are in use in {dataset}.");
    }

    Ok(())
}

/// Retrieve the value sets from each struct field in the source data.
#[tracing::instrument]
fn get_fa_keys<P: AsRef<std::path::Path> + std::fmt::Debug>(
    path: P,
) -> Result<(std::collections::BTreeSet<FixedAssetTable>, NipaRanges), BeaErr> {
    let path = path.as_ref().to_owned();
    let data = FixedAssets::try_from(&path)?;
    let ids = data.table_names();
    let years = data.year().to_owned();
    Ok((ids, years))
}

/// Print the value sets from each struct field in the source data to the BEA_DATA directory.
#[tracing::instrument]
pub fn fa_keys() -> Result<(), BeaErr> {
    let path = bea_data()?;
    let (names, years) = get_fa_keys(&path)?;
    let target = path.join("FixedAssets_TableName_Params.json");
    write_json(&names, target)?;
    let target = path.join("FixedAssets_Year_Params.json");
    write_json(&years, target)?;
    Ok(())
}

#[tracing::instrument]
pub async fn check_fa_keys() -> Result<(), BeaErr> {
    let path = bea_data()?;
    let dataset = Dataset::InputOutput;
    // BEA provided paramater name keys for table id and year
    let (names, _years) = get_fa_keys(&path)?;
    // load source data
    let codes = get_fa_codes().await?;
    // names observed in source data
    let obs_names = codes.table_names();
    // years observed in source data
    let _obs_years = codes.time_periods();

    // Are all parameter keys contained in source data?
    let unused_names = names
        .difference(obs_names)
        .collect::<std::collections::BTreeSet<&FixedAssetTable>>();
    // let unused_years = years
    //     .difference(obs_years)
    //     .collect::<std::collections::BTreeSet<&jiff::civil::Date>>();
    // Print unused keys to bea_data directory
    if !unused_names.is_empty() {
        let target = path.join(format!("{dataset}_TableName_Unused.json"));
        write_json(&unused_names, target)?;
        tracing::error!(
            "Unused parameter keys for table names in {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!(
            "All parameter keys for table names in {dataset} are present in the source data."
        );
    }
    // if !unused_years.is_empty() {
    //     let target = path.join(format!("{dataset}_Years_UnusedParam.json"));
    //     write_json(&unused_years, target)?;
    //     tracing::error!(
    //         "Unused parameter keys for years in {dataset} printed to BEA_DATA directory."
    //     );
    // } else {
    //     tracing::info!("All parameter keys for years in {dataset} are present in the source data.");
    // }
    // Are all unique source data values represented by a parameter key?
    let missing_names = obs_names
        .difference(&names)
        .collect::<std::collections::BTreeSet<&FixedAssetTable>>();
    // let missing_years = obs_years
    //     .difference(&years)
    //     .collect::<std::collections::BTreeSet<&jiff::civil::Date>>();
    // // Print missing keys to bea_data directory
    if !missing_names.is_empty() {
        let target = path.join(format!("{dataset}_TableName_Missing.json"));
        write_json(&missing_names, target)?;
        tracing::error!(
            "Missing parameter keys for table name in source data from {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!(
            "All values of table name in source data from {dataset} are contained in the parameter keys."
        );
    }
    // if !missing_years.is_empty() {
    //     let target = path.join(format!("{dataset}_Years_MissingParam.json"));
    //     write_json(&missing_years, target)?;
    //     tracing::error!(
    //         "Missing parameter keys for years in source data from {dataset} printed to BEA_DATA directory."
    //     );
    // } else {
    //     tracing::info!(
    //         "All values of years in source data from {dataset} are contained in the parameter keys."
    //     );
    // }
    Ok(())
}
