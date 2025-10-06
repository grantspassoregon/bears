use crate::{difference, params};
use bears_ecology::initial_load;
use bears_species::{
    BeaErr, Data, Dataset, DatasetMissing, Frequency, GdpByIndustry, Naics, Note, ParameterName,
    Set,
};
use std::collections::BTreeSet;
use strum::IntoEnumIterator;

/// Holds unique values from each field of [`GdpDatum`](bears_species::GdpDatum) in a [`BTreeSet`]
/// for comparison between expected and observed values in BEA parameters and responses.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
    derive_new::new,
)]
pub struct GdpKeys {
    frequencies: BTreeSet<Frequency>,
    industries: BTreeSet<Naics>,
    notes: Option<BTreeSet<Note>>,
    quarters: Option<BTreeSet<jiff::civil::Date>>,
    tables: BTreeSet<i64>,
    years: BTreeSet<jiff::civil::Date>,
}

impl GdpKeys {
    #[tracing::instrument(skip_all)]
    pub fn append_quarters(
        record: &mut Option<BTreeSet<jiff::civil::Date>>,
        addition: &mut Option<BTreeSet<jiff::civil::Date>>,
    ) {
        if let Some(record) = record
            && let Some(new) = addition
        {
            record.append(new);
        }
    }
    /// Retrieve the value sets from each struct field in the source data.
    #[tracing::instrument]
    fn expected<'a, P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
        dataset: Dataset,
    ) -> Result<GdpKeys, BeaErr> {
        let path = path.as_ref().to_owned();
        match dataset {
            Dataset::GDPbyIndustry => {
                let data = GdpByIndustry::try_from((&path, dataset))?;
                let freqs = GdpByIndustry::frequencies(dataset)
                    .iter()
                    .cloned()
                    .collect::<std::collections::BTreeSet<Frequency>>();
                let industries = data.industries();
                let notes = None;
                let quarters = None;
                let tables = data.table_ids();
                let years = data.years();
                Ok(GdpKeys::new(
                    freqs, industries, notes, quarters, tables, years,
                ))
            }
            Dataset::UnderlyingGDPbyIndustry => {
                let data = GdpByIndustry::try_from((&path, dataset))?;
                let freqs = GdpByIndustry::frequencies(dataset)
                    .iter()
                    .cloned()
                    .collect::<std::collections::BTreeSet<Frequency>>();
                let industries = data.industries();
                let notes = None;
                let quarters = None;
                let tables = data.table_ids();
                let years = data.years();
                Ok(GdpKeys::new(
                    freqs, industries, notes, quarters, tables, years,
                ))
            }
            _ => {
                tracing::error!(
                    "Invalid dataset: {dataset}.  GDPbyIndustry or UnderlyingGDPbyIndustry required."
                );
                let error = DatasetMissing::new(dataset.to_string(), line!(), file!().to_owned());
                Err(error.into())
            }
        }
    }

    /// Print the value sets from each struct field in the source data to the BEA_DATA directory.
    #[tracing::instrument]
    pub fn print_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
        dataset: Dataset,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let kind = "Expected";
        let data = Self::expected(path, dataset)?;
        let name = ParameterName::Frequency;
        params(data.frequencies(), path, dataset, name, kind)?;
        let name = ParameterName::Industry;
        params(data.industries(), path, dataset, name, kind)?;
        let name = ParameterName::TableID;
        params(data.tables(), path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(data.years(), path, dataset, name, kind)?;
        Ok(())
    }

    /// Compare expected values to enum representation.  Print missing and unused parameters to the
    /// BEA_DATA directory.
    #[tracing::instrument]
    pub fn check_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
        dataset: Dataset,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        // BEA provided paramater name keys for table id and year
        let keys = Self::expected(path, dataset)?;
        // compare value sets to campare against response data
        let freq = keys.frequencies();
        let ind = keys.industries();

        let freqs = Frequency::iter().collect::<std::collections::BTreeSet<Frequency>>();
        let inds = Naics::variants();

        let unused = "UnusedParams";
        let missing = "MissingParams";
        let name = ParameterName::Frequency;
        difference(freq, &freqs, path, dataset, name, unused)?;
        difference(&freqs, freq, path, dataset, name, missing)?;
        let name = ParameterName::Industry;
        difference(ind, &inds, path, dataset, name, unused)?;
        difference(&inds, ind, path, dataset, name, missing)?;
        Ok(())
    }

    /// Attempts to load all files in the download [`History`], without respect to the load `History`.
    /// Loads GDPbyIndustry files, converts them to struct field sets.
    #[tracing::instrument(skip_all)]
    async fn observed(dataset: Dataset) -> Result<GdpKeys, BeaErr> {
        let obs = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded.", obs.len());
        let mut frequencies = BTreeSet::new();
        let mut industries = BTreeSet::new();
        let mut notes = BTreeSet::new();
        let mut quarters = Some(BTreeSet::new());
        let mut table_ids = BTreeSet::new();
        let mut years = BTreeSet::new();
        obs.iter()
            .map(|v| {
                if let Data::Gdp(data) = v {
                    frequencies.append(&mut data.frequencies());
                    industries.append(&mut data.industries());
                    notes.append(&mut data.notes());
                    GdpKeys::append_quarters(&mut quarters, &mut data.quarters());
                    table_ids.append(&mut data.table_ids());
                    years.append(&mut data.years());
                }
            })
            .for_each(drop);
        if frequencies.is_empty()
            || industries.is_empty()
            || table_ids.is_empty()
            || years.is_empty()
        {
            let error = Set::Empty;
            Err(error.into())
        } else {
            let quarters = if let Some(values) = &quarters {
                if values.is_empty() { None } else { quarters }
            } else {
                None
            };
            let notes = if notes.is_empty() { None } else { Some(notes) };
            let codes = GdpKeys::new(frequencies, industries, notes, quarters, table_ids, years);
            Ok(codes)
        }
    }

    /// Prints set members of type struct fields from source data to the `BEA_DATA` directory.
    #[tracing::instrument(skip_all)]
    pub async fn print_observed<P: AsRef<std::path::Path>>(
        path: P,
        dataset: Dataset,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let kind = "Observed";
        let obs = Self::observed(dataset).await?;

        let name = ParameterName::Frequency;
        params(obs.frequencies(), path, dataset, name, kind)?;
        let name = ParameterName::IndustryCode;
        params(obs.industries(), path, dataset, name, kind)?;
        let name = ParameterName::Notes;
        params(obs.notes(), path, dataset, name, kind)?;
        let name = ParameterName::Quarter;
        params(obs.quarters(), path, dataset, name, kind)?;
        let name = ParameterName::TableID;
        params(obs.tables(), path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(obs.years(), path, dataset, name, kind)?;
        Ok(())
    }

    /// Checks that all struct field values in source data are present as variants of their corresponding enum.
    /// Also warns on unused variants in the associated enum.
    #[tracing::instrument(skip_all)]
    pub async fn check_observed<P: AsRef<std::path::Path>>(
        path: P,
        dataset: Dataset,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let exp = Self::expected(path, dataset)?;
        // sets of codes within source data
        let obs = Self::observed(dataset).await?;
        // set of Frequency variants
        // for each struct field
        // Is the source code a member of the corresponding enum?
        // Print source codes not in Naics or Frequency to bea_data "Missing"
        let unused = "Unused";
        let missing = "Missing";
        let name = ParameterName::Frequency;
        let left = exp.frequencies();
        let right = obs.frequencies();
        difference(left, right, path, dataset, name, unused)?;
        difference(right, left, path, dataset, name, missing)?;
        let left = exp.industries();
        let right = obs.industries();
        let name = ParameterName::Industry;
        difference(left, right, path, dataset, name, unused)?;
        difference(right, left, path, dataset, name, missing)?;
        let left = exp.tables();
        let right = obs.tables();
        let name = ParameterName::TableID;
        difference(left, right, path, dataset, name, unused)?;
        difference(right, left, path, dataset, name, missing)?;
        let left = exp.years();
        let right = obs.years();
        let name = ParameterName::Year;
        difference(left, right, path, dataset, name, unused)?;
        difference(right, left, path, dataset, name, missing)?;
        Ok(())
    }
}
