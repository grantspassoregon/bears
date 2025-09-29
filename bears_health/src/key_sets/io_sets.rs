use crate::{difference, params};
use bears_ecology::initial_load;
use bears_species::{BeaErr, Data, Dataset, InputOutput, InputOutputTable, Naics, ParameterName};
use std::collections::BTreeSet;
use strum::IntoEnumIterator;

/// Holds unique values from each field of [`InputOutputDatum`](bears_species::InputOutputDatum) in a [`BTreeSet`]
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
pub struct IoKeys {
    column_codes: BTreeSet<Naics>,
    column_types: BTreeSet<String>,
    row_codes: BTreeSet<Naics>,
    row_types: BTreeSet<String>,
    table_ids: BTreeSet<InputOutputTable>,
    years: BTreeSet<jiff::civil::Date>,
}

impl IoKeys {
    /// Attempts to load all files in the download [`History`], without respect to the load `History`.
    /// Loads InputOutput files, converts them to row and column codes.
    /// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
    /// `BEA_DATA` directory.
    #[tracing::instrument(skip_all)]
    async fn observed() -> Result<IoKeys, BeaErr> {
        let dataset = Dataset::InputOutput;
        let iot = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded.", iot.len());
        let mut row_codes = BTreeSet::new();
        let mut row_types = BTreeSet::new();
        let mut column_codes = BTreeSet::new();
        let mut column_types = BTreeSet::new();
        let mut table_ids = BTreeSet::new();
        let mut years = BTreeSet::new();
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

        let result = IoKeys::new(
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
    #[tracing::instrument]
    pub async fn print_observed<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::InputOutput;
        let kind = "Observed";
        let codes = Self::observed().await?;
        let name = ParameterName::ColumnCode;
        params(codes.column_codes(), path, dataset, name, kind)?;
        let name = ParameterName::ColumnType;
        params(codes.column_types(), path, dataset, name, kind)?;
        let name = ParameterName::RowCode;
        params(codes.row_codes(), path, dataset, name, kind)?;
        let name = ParameterName::RowType;
        params(codes.row_types(), path, dataset, name, kind)?;
        let name = ParameterName::TableID;
        params(codes.table_ids(), path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(codes.years(), path, dataset, name, kind)?;
        Ok(())
    }

    /// Attempts to load all files in the download [`History`], without respect to the load `History`.
    /// Loads InputOutput files, converts them to row and column codes.
    /// Checks that all row and column codes are present as variants of the Naics::InputOutput enum.
    #[tracing::instrument]
    pub async fn check_observed<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::InputOutput;
        let obs = Self::observed().await?;
        // build struct field sets
        // set of variants in the Naics type
        let naics = Naics::variants();
        // print missing column and row codes to bea_data
        let unused = "Unused";
        let missing = "Missing";
        let name = ParameterName::ColumnCode;
        difference(&naics, obs.column_codes(), path, dataset, name, unused)?;
        difference(obs.column_codes(), &naics, path, dataset, name, missing)?;

        let name = ParameterName::RowCode;
        difference(&naics, obs.row_codes(), path, dataset, name, unused)?;
        difference(obs.row_codes(), &naics, path, dataset, name, missing)?;
        Ok(())
    }

    #[tracing::instrument]
    fn expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
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
    pub fn print_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::InputOutput;
        let kind = "Expected";
        let (ids, years) = Self::expected(path)?;
        let name = ParameterName::TableID;
        params(&ids, path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(&years, path, dataset, name, kind)?;
        Ok(())
    }

    /// Compare expected values to enum representation.  Print missing and unused parameters to the
    /// BEA_DATA directory.
    #[tracing::instrument]
    pub fn check_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::InputOutput;
        // BEA provided paramater name keys for table id and year
        let (ids, _years) = Self::expected(path)?;
        // ids observed in source data
        let id_set =
            InputOutputTable::iter().collect::<std::collections::BTreeSet<InputOutputTable>>();

        // Are all parameter keys contained in source data?
        let unused = "UnusedParams";
        let missing = "MissingParams";
        let name = ParameterName::TableID;
        difference(&ids, &id_set, path, dataset, name, unused)?;
        difference(&id_set, &ids, path, dataset, name, missing)?;
        Ok(())
    }
}
