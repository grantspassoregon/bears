use bears_ecology::{bea_data, initial_load, trace_init};
use bears_species::{BeaErr, Data, Dataset, Frequency, GdpCodes, Naics, Set, write_json};
use std::collections::BTreeSet;
use strum::IntoEnumIterator;

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads GDPbyIndustry files, converts them to struct field sets.
#[tracing::instrument(skip_all)]
pub async fn get_gdp_codes(dataset: Dataset) -> Result<GdpCodes, BeaErr> {
    trace_init()?;
    let data = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", data.len());
    let mut frequencies = BTreeSet::new();
    let mut industries = BTreeSet::new();
    let mut table_ids = BTreeSet::new();
    let mut years = BTreeSet::new();
    data.iter()
        .map(|v| {
            if let Data::Gdp(data) = v {
                frequencies.append(&mut data.frequencies());
                industries.append(&mut data.industries());
                table_ids.append(&mut data.table_ids());
                years.append(&mut data.years());
            }
            if let Data::UnderlyingGdp(data) = v {
                frequencies.append(&mut data.frequencies());
                industries.append(&mut data.industries());
                table_ids.append(&mut data.table_ids());
                years.append(&mut data.years());
            }
        })
        .for_each(drop);
    if frequencies.is_empty() || industries.is_empty() || table_ids.is_empty() || years.is_empty() {
        let error = Set::Empty;
        Err(error.into())
    } else {
        let codes = GdpCodes::new(frequencies, industries, table_ids, years);
        Ok(codes)
    }
}

/// Prints set members of type struct fields from source data to the `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn gdp_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::GDPbyIndustry, Dataset::UnderlyingGDPbyIndustry];
    for dataset in datasets {
        let data = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded.", data.len());
        let codes = get_gdp_codes(dataset).await?;

        let path = bea_data()?;
        let target = path.join(format!("{dataset}_Frequency.json"));
        write_json(codes.frequencies(), target)?;
        let target = path.join(format!("{dataset}_IndustryCode.json"));
        write_json(codes.industries(), target)?;
        let target = path.join(format!("{dataset}_TableId.json"));
        write_json(codes.table_ids(), target)?;
        let target = path.join(format!("{dataset}_Year.json"));
        write_json(codes.years(), target)?;
    }
    Ok(())
}

/// Checks that all struct field values in source data are present as variants of their corresponding enum.
/// Also warns on unused variants in the associated enum.
#[tracing::instrument(skip_all)]
pub async fn check_gdp_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::GDPbyIndustry, Dataset::UnderlyingGDPbyIndustry];
    for dataset in datasets {
        let data = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded.", data.len());
        // sets of codes within source data
        let codes = get_gdp_codes(dataset).await?;
        // set of Frequency variants
        let freq = Frequency::iter().collect::<std::collections::BTreeSet<Frequency>>();
        // set of variants in the Naics type
        let naics = Naics::variants();
        // for each struct field
        // Is the source code a member of the corresponding enum?
        // Print source codes not in Naics or Frequency to bea_data "Missing"
        let missing_freq = codes
            .frequencies()
            .difference(&freq)
            .collect::<std::collections::BTreeSet<&Frequency>>();
        if !missing_freq.is_empty() {
            tracing::error!(
                "Missing Frequency variants in {dataset} printed to BEA_DATA directory."
            );
            let path = bea_data()?;
            let target = path.join(format!("{dataset}_Frequency_Missing.json"));
            write_json(&missing_freq, target)?;
        } else {
            tracing::info!(
                "All source frequencies in {dataset} are variants of the Frequency type."
            );
        }

        let source_naics = codes.industries().to_owned();
        let missing_naics = source_naics
            .difference(&naics)
            .collect::<std::collections::BTreeSet<&Naics>>();
        if !missing_naics.is_empty() {
            let path = bea_data()?;
            let target = path.join(format!("{dataset}_Industry_Missing.json"));
            write_json(&missing_naics, target)?;
            tracing::error!(
                "Missing Industry variants in {dataset} printed to BEA_DATA directory."
            );
        } else {
            tracing::info!("All source industries in {dataset} are variants of the Naics type.");
        }
        // Print unused variants of Naics or Frequency to bea_data "Unused"
        let unused_freq = freq
            .difference(codes.frequencies())
            .collect::<std::collections::BTreeSet<&Frequency>>();
        if !unused_freq.is_empty() {
            let path = bea_data()?;
            let target = path.join(format!("{dataset}_Frequency_Unused.json"));
            write_json(&unused_freq, target)?;
            tracing::error!(
                "Unused Frequency variants in {dataset} printed to BEA_DATA directory."
            );
        } else {
            tracing::info!("All variants of the Frequency type are in use in {dataset}.");
        }
        let unused_naics = naics
            .difference(codes.industries())
            .collect::<std::collections::BTreeSet<&Naics>>();
        if !unused_naics.is_empty() {
            let path = bea_data()?;
            let target = path.join(format!("{dataset}_Industry_Unused.json"));
            write_json(&unused_naics, target)?;
            tracing::warn!("Unused Naics variants printed to BEA_DATA directory.");
        } else {
            tracing::info!("All variants of the Naics type are in use in {dataset}.");
        }
    }
    Ok(())
}
