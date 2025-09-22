use bears_ecology::{bea_data, initial_load, trace_init};
use bears_species::{
    BeaErr, Data, Dataset, InputOutput, InputOutputTable, IoCodes, Naics, write_json,
};

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads InputOutput files, converts them to row and column codes.
/// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
/// `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn get_io_codes() -> Result<IoCodes, BeaErr> {
    trace_init()?;
    let dataset = Dataset::InputOutput;
    let iot = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", iot.len());
    let mut row_codes = std::collections::BTreeSet::new();
    let mut row_types = std::collections::BTreeSet::new();
    let mut column_codes = std::collections::BTreeSet::new();
    let mut column_types = std::collections::BTreeSet::new();
    let mut table_ids = std::collections::BTreeSet::new();
    let mut years = std::collections::BTreeSet::new();
    iot.iter()
        .map(|v| {
            if let Data::InputOutput(data) = v {
                column_codes.append(&mut data.column_codes());
                column_types.append(&mut data.column_types());
                row_codes.append(&mut data.row_codes());
                row_types.append(&mut data.row_types());
                table_ids.append(&mut data.table_ids());
                years.append(&mut data.years());
            }
        })
        .for_each(drop);

    let result = IoCodes::new(
        column_codes,
        column_types,
        row_codes,
        row_types,
        table_ids,
        years,
    );
    Ok(result)
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads InputOutput files, converts them to row and column codes.
/// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
/// `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn io_codes() -> Result<(), BeaErr> {
    let codes = get_io_codes().await?;
    let path = bea_data()?;
    let column_path = path.join("InputOutput_ColumnCode.json");
    write_json(codes.column_codes(), column_path)?;
    let ctype_path = path.join("InputOutput_ColumnType.json");
    write_json(codes.column_types(), ctype_path)?;
    let row_path = path.join("InputOutput_RowCode.json");
    write_json(codes.row_codes(), row_path)?;
    let rtype_path = path.join("InputOutput_RowType.json");
    write_json(codes.row_types(), rtype_path)?;
    let table_path = path.join("InputOutput_TableId.json");
    write_json(codes.table_ids(), table_path)?;
    let year_path = path.join("InputOutput_Year.json");
    write_json(codes.years(), year_path)?;
    Ok(())
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads InputOutput files, converts them to row and column codes.
/// Checks that all row and column codes are present as variants of the Naics::InputOutput enum.
#[tracing::instrument]
pub async fn check_io_codes() -> Result<(), BeaErr> {
    let dataset = Dataset::InputOutput;
    let codes = get_io_codes().await?;
    // build struct field sets
    // set of variants in the Naics type
    let naics = Naics::variants();
    // print missing column and row codes to bea_data
    let source_columns = codes.column_codes().to_owned();
    let missing_columns = source_columns
        .difference(&naics)
        .collect::<std::collections::BTreeSet<&Naics>>();
    if !missing_columns.is_empty() {
        let path = bea_data()?;
        let target = path.join(format!("{dataset}_ColumnCode_Missing.json"));
        write_json(&missing_columns, target)?;
        tracing::error!("Missing ColumnCode variants in {dataset} printed to BEA_DATA directory.");
    } else {
        tracing::info!("All source column codes in {dataset} are variants of the Naics type.");
    }

    let source_rows = codes.row_codes().to_owned();
    let missing_rows = source_rows
        .difference(&naics)
        .collect::<std::collections::BTreeSet<&Naics>>();
    if !missing_rows.is_empty() {
        let path = bea_data()?;
        let target = path.join(format!("{dataset}_RowCode_Missing.json"));
        write_json(&missing_rows, target)?;
        tracing::error!("Missing RowCode variants in {dataset} printed to BEA_DATA directory.");
    } else {
        tracing::info!("All source row codes in {dataset} are variants of the Naics type.");
    }
    Ok(())
}

#[tracing::instrument]
fn get_io_keys<P: AsRef<std::path::Path> + std::fmt::Debug>(
    path: P,
) -> Result<
    (
        std::collections::BTreeSet<InputOutputTable>,
        std::collections::BTreeSet<jiff::civil::Date>,
    ),
    BeaErr,
> {
    let path = path.as_ref().to_owned();
    let data = InputOutput::try_from(&path)?;
    let ids = data.table_ids();
    let years = data.years();
    Ok((ids, years))
}

#[tracing::instrument]
pub fn io_keys() -> Result<(), BeaErr> {
    let path = bea_data()?;
    let (ids, years) = get_io_keys(&path)?;
    let target = path.join("InputOutput_TableId_Params.json");
    write_json(&ids, target)?;
    let target = path.join("InputOutput_Year_Params.json");
    write_json(&years, target)?;
    Ok(())
}

#[tracing::instrument]
pub async fn check_io_keys() -> Result<(), BeaErr> {
    let path = bea_data()?;
    let dataset = Dataset::InputOutput;
    // BEA provided paramater name keys for table id and year
    let (ids, years) = get_io_keys(&path)?;
    // load source data
    let codes = get_io_codes().await?;
    // ids observed in source data
    let obs_ids = codes.table_ids();
    // years observed in source data
    let obs_years = codes.years();

    // Are all parameter keys contained in source data?
    let unused_ids = ids
        .difference(obs_ids)
        .collect::<std::collections::BTreeSet<&InputOutputTable>>();
    let unused_years = years
        .difference(obs_years)
        .collect::<std::collections::BTreeSet<&jiff::civil::Date>>();
    // Print unused keys to bea_data directory
    if !unused_ids.is_empty() {
        let target = path.join(format!("{dataset}_TableId_UnusedParam.json"));
        write_json(&unused_ids, target)?;
        tracing::error!(
            "Unused parameter keys for table ids in {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!(
            "All parameter keys for tables in {dataset} are present in the source data."
        );
    }
    if !unused_years.is_empty() {
        let target = path.join(format!("{dataset}_Years_UnusedParam.json"));
        write_json(&unused_years, target)?;
        tracing::error!(
            "Unused parameter keys for years in {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!("All parameter keys for years in {dataset} are present in the source data.");
    }
    // Are all unique source data values represented by a parameter key?
    let missing_ids = obs_ids
        .difference(&ids)
        .collect::<std::collections::BTreeSet<&InputOutputTable>>();
    let missing_years = obs_years
        .difference(&years)
        .collect::<std::collections::BTreeSet<&jiff::civil::Date>>();
    // Print missing keys to bea_data directory
    if !missing_ids.is_empty() {
        let target = path.join(format!("{dataset}_TableId_MissingParam.json"));
        write_json(&missing_ids, target)?;
        tracing::error!(
            "Missing parameter keys for table id in source data from {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!(
            "All values of table id in source data from {dataset} are contained in the parameter keys."
        );
    }
    if !missing_years.is_empty() {
        let target = path.join(format!("{dataset}_Years_MissingParam.json"));
        write_json(&missing_years, target)?;
        tracing::error!(
            "Missing parameter keys for years in source data from {dataset} printed to BEA_DATA directory."
        );
    } else {
        tracing::info!(
            "All values of years in source data from {dataset} are contained in the parameter keys."
        );
    }
    Ok(())
}
