use crate::{Set, difference, params};
use bears_ecology::initial_load;
use bears_species::{
    BeaErr, Data, Dataset, FixedAssetLine, FixedAssetTable, FixedAssets, Measure, Metric,
    NipaRanges, Note, ParameterName, Scale,
};
use strum::IntoEnumIterator;

#[allow(clippy::too_many_arguments)]
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
/// Contains the value sets for each field in [`FixedAssetDatum`] contained in a [`FixedAssetData`].
pub struct FixedAssetKeys {
    cl_units: std::collections::BTreeSet<Measure>,
    lines: std::collections::BTreeSet<FixedAssetLine>,
    line_descriptions: std::collections::BTreeMap<String, std::collections::BTreeSet<i64>>,
    metric_names: std::collections::BTreeSet<Metric>,
    notes: std::collections::BTreeSet<Note>,
    series_codes: std::collections::BTreeSet<String>,
    series: std::collections::BTreeMap<String, std::collections::BTreeSet<FixedAssetTable>>,
    series_inverse: std::collections::BTreeMap<FixedAssetTable, std::collections::BTreeSet<String>>,
    series2: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    series_inverse2: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    table_names: std::collections::BTreeSet<FixedAssetTable>,
    time_periods: std::collections::BTreeSet<jiff::civil::Date>,
    unit_mults: std::collections::BTreeSet<Scale>,
}

impl FixedAssetKeys {
    /// Attempts to load all files in the download [`History`], without respect to the load `History`.
    /// Loads InputOutput files, converts them to row and column codes.
    /// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
    /// `BEA_DATA` directory.
    #[tracing::instrument]
    async fn observed() -> Result<Self, BeaErr> {
        let dataset = Dataset::FixedAssets;
        let data = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded.", data.len());
        let mut cl_units = std::collections::BTreeSet::new();
        let mut lines = std::collections::BTreeSet::new();
        let mut line_descriptions = std::collections::BTreeMap::new();
        let mut metric_names = std::collections::BTreeSet::new();
        let mut notes = std::collections::BTreeSet::new();
        let mut series_codes = std::collections::BTreeSet::new();
        let mut series = std::collections::BTreeMap::new();
        let mut series_inverse = std::collections::BTreeMap::new();
        let mut series2 = std::collections::BTreeMap::new();
        let mut series_inverse2 = std::collections::BTreeMap::new();
        let mut table_names = std::collections::BTreeSet::new();
        let mut time_periods = std::collections::BTreeSet::new();
        let mut unit_mults = std::collections::BTreeSet::new();
        data.iter()
            .map(|v| {
                if let Data::FixedAssets(data) = v {
                    cl_units.append(&mut data.cl_units());
                    lines.append(&mut data.lines());
                    line_descriptions.append(&mut data.descriptions());
                    metric_names.append(&mut data.metric_names());
                    notes.append(&mut data.notes());
                    series_codes.append(&mut data.series_codes());
                    series.append(&mut data.series());
                    series_inverse.append(&mut data.series_inverse());
                    series2.append(&mut data.series2());
                    series_inverse2.append(&mut data.series_inverse2());
                    table_names.append(&mut data.table_names());
                    time_periods.append(&mut data.time_periods());
                    unit_mults.append(&mut data.unit_mults());
                }
            })
            .for_each(drop);

        let result = Self::new(
            cl_units,
            lines,
            line_descriptions,
            metric_names,
            notes,
            series_codes,
            series,
            series_inverse,
            series2,
            series_inverse2,
            table_names,
            time_periods,
            unit_mults,
        );
        Ok(result)
    }

    pub fn discriminate<T, U, V, W>(
        target: &std::collections::BTreeMap<T, U>,
        discriminant: &std::collections::BTreeMap<V, W>,
    ) {
        let targ = target.len();
        tracing::info!("Target size: {targ}");
        let disc = discriminant.len();
        tracing::info!("Discriminant size: {disc}");
        if disc < targ {
            tracing::error!(
                "Discriminants relation to target is one to many.  The set of discriminants is insufficient to identify the target."
            );
        } else if disc == targ {
            tracing::info!(
                "Discriminants relation to target is one to one.  The set of discriminants is ideal to identify the target."
            );
        } else if disc > targ {
            tracing::info!(
                "Discriminants relation to target is many to one.  The set of discriminants is sufficient to identify the target, but may be inefficient."
            );
        }
    }

    /// Attempts to load all files in the download [`History`], without respect to the load `History`.
    /// Loads InputOutput files, converts them to row and column codes.
    /// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
    /// `BEA_DATA` directory.
    #[tracing::instrument(skip_all)]
    pub async fn print_observed<P: AsRef<std::path::Path>>(path: P) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::FixedAssets;
        let obs = Self::observed().await?;
        let kind = "Observed";
        let name = ParameterName::ClUnit;
        params(obs.cl_units(), path, dataset, name, kind)?;
        let name = ParameterName::LineNumber;
        params(obs.lines(), path, dataset, name, kind)?;
        let name = ParameterName::LineDescription;
        params(obs.line_descriptions(), path, dataset, name, kind)?;
        let name = ParameterName::MetricName;
        params(obs.metric_names(), path, dataset, name, kind)?;
        let name = ParameterName::Notes;
        params(obs.notes(), path, dataset, name, kind)?;
        let disc = obs.series_inverse2();
        let hist = disc.histogram();
        tracing::info!("| Count | Values |");
        tracing::info!("==================");
        for (count, set) in hist {
            tracing::info!("| {count} | {} |", set.len())
        }

        if let Err(dupe) = disc.unique() {
            tracing::error!("{dupe:?}");
        } else {
            tracing::info!("✅ Success: discriminant is disjoint for target.");
        }

        let name = ParameterName::SeriesCode;
        params(disc, path, dataset, name, kind)?;
        let name = ParameterName::TableName;
        params(obs.table_names(), path, dataset, name, kind)?;
        let name = ParameterName::TimePeriod;
        params(obs.time_periods(), path, dataset, name, kind)?;
        let name = ParameterName::UnitMult;
        params(obs.unit_mults(), path, dataset, name, kind)?;
        Ok(())
    }

    /// Attempts to load all files in the download [`History`], without respect to the load `History`.
    /// Loads FixedAssets files, converts them to value sets.
    /// Checks that all tables are present as variants of the FixedAssetTable enum.
    #[tracing::instrument]
    pub async fn check_observed<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::InputOutput;
        let (exp, _years) = Self::expected(path)?;
        let obs = Self::observed().await?;

        let unused = "Unused";
        let missing = "Missing";
        let name = ParameterName::TableName;
        difference(&exp, obs.table_names(), path, dataset, name, unused)?;
        difference(obs.table_names(), &exp, path, dataset, name, missing)?;

        Ok(())
    }

    /// Retrieve the value sets from each struct field in the source data.
    #[tracing::instrument]
    fn expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
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
    pub fn print_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::FixedAssets;
        let kind = "Expected";
        let (names, years) = Self::expected(path)?;
        let name = ParameterName::TableName;
        params(&names, path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(&years, path, dataset, name, kind)?;
        Ok(())
    }

    /// Compare expected values to enum representation.  Print missing and unused parameters to the
    /// BEA_DATA directory.
    #[tracing::instrument]
    pub async fn check_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::FixedAssets;
        // BEA provided paramater name keys for table id and year
        let (names, _years) = Self::expected(path)?;
        let tables =
            FixedAssetTable::iter().collect::<std::collections::BTreeSet<FixedAssetTable>>();

        // Are all parameter keys contained in source data?
        let unused = "UnusedParams";
        let missing = "MissingParams";
        let name = ParameterName::TableName;
        difference(&names, &tables, path, dataset, name, unused)?;
        difference(&tables, &names, path, dataset, name, missing)?;
        Ok(())
    }
}
