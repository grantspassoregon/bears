use crate::{Set, difference, params};
use bears_ecology::initial_load;
use bears_species::{
    BeaErr, Data, Dataset, FixedAssetLine, FixedAssetTable, FixedAssets, Measure, Metric,
    NipaRanges, Note, ParameterName, Scale,
};
use std::collections::{BTreeMap, BTreeSet};
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
    cl_units: BTreeSet<Measure>,
    lines: BTreeSet<FixedAssetLine>,
    line_descriptions: BTreeMap<String, BTreeSet<i64>>,
    metric_names: BTreeSet<Metric>,
    notes: BTreeSet<Note>,
    series_codes: BTreeSet<String>,
    series_by_line: BTreeMap<FixedAssetLine, BTreeSet<String>>,
    series_by_number: BTreeMap<i64, BTreeSet<String>>,
    series_by_metric: BTreeMap<Metric, BTreeSet<String>>,
    series_by_table: BTreeMap<FixedAssetTable, BTreeSet<String>>,
    series_by_year: BTreeMap<jiff::civil::Date, BTreeSet<String>>,
    series_by_line_and_number: BTreeMap<String, BTreeSet<String>>,
    series_by_line_number_and_metric: BTreeMap<String, BTreeSet<String>>,
    series_by_table_and_line: BTreeMap<String, BTreeSet<String>>,
    series_by_table_and_number: BTreeMap<String, BTreeSet<String>>,
    series_by_table_line_and_number: BTreeMap<String, BTreeSet<String>>,
    table_names: BTreeSet<FixedAssetTable>,
    time_periods: BTreeSet<jiff::civil::Date>,
    unit_mults: BTreeSet<Scale>,
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
        let mut cl_units = BTreeSet::new();
        let mut lines = BTreeSet::new();
        let mut line_descriptions = BTreeMap::new();
        let mut metric_names = BTreeSet::new();
        let mut notes = BTreeSet::new();
        let mut series_codes = BTreeSet::new();
        let mut series_by_line = BTreeMap::new();
        let mut series_by_number = BTreeMap::new();
        let mut series_by_metric = BTreeMap::new();
        let mut series_by_table = BTreeMap::new();
        let mut series_by_year = BTreeMap::new();
        let mut series_by_line_and_number = BTreeMap::new();
        let mut series_by_line_number_and_metric = BTreeMap::new();
        let mut series_by_table_and_line = BTreeMap::new();
        let mut series_by_table_and_number = BTreeMap::new();
        let mut series_by_table_line_and_number = BTreeMap::new();
        let mut table_names = BTreeSet::new();
        let mut time_periods = BTreeSet::new();
        let mut unit_mults = BTreeSet::new();
        data.iter()
            .map(|v| {
                if let Data::FixedAssets(data) = v {
                    data.check_series().unwrap();
                    cl_units.append(&mut data.cl_units());
                    lines.append(&mut data.lines());
                    line_descriptions.append(&mut data.descriptions());
                    metric_names.append(&mut data.metric_names());
                    notes.append(&mut data.notes());
                    series_codes.append(&mut data.series_codes());
                    series_by_line.append(&mut data.series_by_line());
                    series_by_number.append(&mut data.series_by_number());
                    series_by_metric.append(&mut data.series_by_metric());
                    series_by_table.append(&mut data.series_by_table());
                    series_by_year.append(&mut data.series_by_year());
                    series_by_line_and_number.append(&mut data.series_by_line_and_number());
                    series_by_line_number_and_metric
                        .append(&mut data.series_by_line_number_and_metric());
                    series_by_table_and_line.append(&mut data.series_by_table_and_line());
                    series_by_table_and_number.append(&mut data.series_by_table_and_number());
                    series_by_table_line_and_number
                        .append(&mut data.series_by_table_line_and_number());
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
            series_by_line,
            series_by_number,
            series_by_metric,
            series_by_table,
            series_by_year,
            series_by_line_and_number,
            series_by_line_number_and_metric,
            series_by_table_and_line,
            series_by_table_and_number,
            series_by_table_line_and_number,
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
        let name = ParameterName::SeriesCode;
        params(obs.series_codes(), path, dataset, name, kind)?;
        let name = ParameterName::TableName;
        params(obs.table_names(), path, dataset, name, kind)?;
        let name = ParameterName::TimePeriod;
        params(obs.time_periods(), path, dataset, name, kind)?;
        let name = ParameterName::UnitMult;
        params(obs.unit_mults(), path, dataset, name, kind)?;
        let name = "SeriesByLine";
        params(obs.series_by_line(), path, dataset, name, kind)?;
        tracing::info!("Series by Line:");
        Self::report(obs.series_by_line());
        let name = "SeriesByNumber";
        params(obs.series_by_number(), path, dataset, name, kind)?;
        tracing::info!("Series by Number:");
        Self::report(obs.series_by_number());
        let name = "SeriesByMetric";
        params(obs.series_by_metric(), path, dataset, name, kind)?;
        tracing::info!("Series by Metric:");
        Self::report(obs.series_by_metric());
        let name = "SeriesByTable";
        params(obs.series_by_table(), path, dataset, name, kind)?;
        tracing::info!("Series by Table:");
        Self::report(obs.series_by_table());
        let name = "SeriesByYear";
        params(obs.series_by_year(), path, dataset, name, kind)?;
        tracing::info!("Series by Year:");
        Self::report(obs.series_by_year());
        let name = "SeriesByLineAndNumber";
        params(obs.series_by_line_and_number(), path, dataset, name, kind)?;
        tracing::info!("Series by Line and Number:");
        Self::report(obs.series_by_line_and_number());
        let name = "SeriesByLineNumberAndMetric";
        params(
            obs.series_by_line_number_and_metric(),
            path,
            dataset,
            name,
            kind,
        )?;
        tracing::info!("Series by Line, Number And Metric:");
        Self::report(obs.series_by_line_number_and_metric());
        let name = "SeriesByTableAndLine";
        params(obs.series_by_table_and_line(), path, dataset, name, kind)?;
        tracing::info!("Series by Table and Line:");
        Self::report(obs.series_by_table_and_line());
        let name = "SeriesByTableAndNumber";
        params(obs.series_by_table_and_number(), path, dataset, name, kind)?;
        tracing::info!("Series by Table and Number:");
        Self::report(obs.series_by_table_and_number());
        let name = "SeriesByTableLineAndNumber";
        params(
            obs.series_by_table_line_and_number(),
            path,
            dataset,
            name,
            kind,
        )?;
        tracing::info!("Series by Table, Line and Number:");
        Self::report(obs.series_by_table_line_and_number());

        Ok(())
    }

    pub fn report<T, U: Ord + Clone + std::fmt::Debug>(set: &BTreeMap<T, U>) {
        tracing::info!("| Count | Values |");
        tracing::info!("==================");
        for (count, values) in set.histogram() {
            tracing::info!("| {count} | {} |", values.len())
        }

        if let Err(dupe) = set.unique() {
            tracing::error!("{dupe:?}");
        } else {
            tracing::info!("✅ Success: discriminant is disjoint for target.");
        }
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
