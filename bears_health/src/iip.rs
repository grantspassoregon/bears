use bears_ecology::{bea_data, initial_load, trace_init};
use bears_species::{BeaErr, Data, Dataset, write_json};

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads IIP files, converts struct fields to BTree hash maps or sets.
/// Serializes the results to the `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn iip_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let dataset = Dataset::Iip;
    let iot = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", iot.len());
    let mut cl_units = std::collections::BTreeSet::new();
    let mut components = std::collections::BTreeSet::new();
    let mut frequencies = std::collections::BTreeSet::new();
    let mut time_periods = std::collections::BTreeSet::new();
    let mut time_series_codes = std::collections::BTreeMap::new();
    let mut investments = std::collections::BTreeSet::new();
    let mut unit_multipliers = std::collections::BTreeSet::new();
    let mut years = std::collections::BTreeSet::new();
    iot.iter()
        .map(|v| {
            if let Data::Iip(data) = v {
                cl_units.append(&mut data.cl_units());
                components.append(&mut data.components());
                frequencies.append(&mut data.frequencies());
                time_periods.append(&mut data.time_periods());
                time_series_codes.append(&mut data.time_series_codes());
                investments.append(&mut data.investments());
                unit_multipliers.append(&mut data.unit_multipliers());
                years.append(&mut data.years());
            }
        })
        .for_each(drop);

    let path = bea_data()?;
    let cl_unit_path = path.join("Iip_ClUnit.json");
    write_json(&cl_units, cl_unit_path)?;
    let component_path = path.join("Iip_Component.json");
    write_json(&components, component_path)?;
    let target = path.join("Iip_Frequency.json");
    write_json(&frequencies, target)?;
    let target = path.join("Iip_TimePeriod.json");
    write_json(&time_periods, target)?;
    let target = path.join("Iip_TimeSeriesCode.json");
    write_json(&time_series_codes, target)?;
    let target = path.join("Iip_Investment.json");
    write_json(&investments, target)?;
    let target = path.join("Iip_UnitMult.json");
    write_json(&unit_multipliers, target)?;
    let year_path = path.join("InputOutput_Year.json");
    write_json(&years, year_path)?;
    Ok(())
}
