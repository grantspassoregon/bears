use crate::{difference, params};
use bears_ecology::initial_load;
use bears_species::{
    BeaErr, Data, Dataset, FixedAssetTable, FixedAssets, Measure, NipaRanges, ParameterName,
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
    line_descriptions: std::collections::BTreeSet<String>,
    line_numbers: std::collections::BTreeSet<i64>,
    metric_names: std::collections::BTreeSet<String>,
    series_codes: std::collections::BTreeSet<String>,
    table_names: std::collections::BTreeSet<FixedAssetTable>,
    time_periods: std::collections::BTreeSet<jiff::civil::Date>,
    unit_mults: std::collections::BTreeSet<i64>,
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

        let result = Self::new(
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
    pub async fn print_observed<P: AsRef<std::path::Path>>(path: P) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::FixedAssets;
        let codes = Self::observed().await?;
        let kind = "Observed";
        let name = ParameterName::ClUnit;
        params(codes.cl_units(), path, dataset, name, kind)?;
        let name = ParameterName::LineDescription;
        params(codes.line_descriptions(), path, dataset, name, kind)?;
        let name = ParameterName::LineNumber;
        params(codes.line_numbers(), path, dataset, name, kind)?;
        let name = ParameterName::MetricName;
        params(codes.metric_names(), path, dataset, name, kind)?;
        let name = ParameterName::SeriesCode;
        params(codes.series_codes(), path, dataset, name, kind)?;
        let name = ParameterName::TableName;
        params(codes.table_names(), path, dataset, name, kind)?;
        let name = ParameterName::TimePeriod;
        params(codes.time_periods(), path, dataset, name, kind)?;
        let name = ParameterName::UnitMult;
        params(codes.unit_mults(), path, dataset, name, kind)?;
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
