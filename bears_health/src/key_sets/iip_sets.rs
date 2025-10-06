use crate::{difference, params};
use bears_ecology::initial_load;
use bears_species::{
    BeaErr, Component, Data, Dataset, Iip, Investment, ItaFrequency, Measure, Note, ParameterName,
    Scale,
};
use std::collections::BTreeSet;
use strum::IntoEnumIterator;

/// Holds unique values from each field of [`IipDatum`](bears_species::IipDatum) in a [`BTreeSet`]
/// for comparison between expected and observed values in BEA parameters and responses.
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
pub struct IipKeys {
    cl_units: BTreeSet<Measure>,
    components: BTreeSet<Component>,
    frequencies: BTreeSet<ItaFrequency>,
    notes: Option<BTreeSet<Note>>,
    time_periods: BTreeSet<jiff::civil::Date>,
    time_series_codes: std::collections::BTreeMap<String, String>,
    investments: BTreeSet<Investment>,
    unit_multipliers: BTreeSet<Scale>,
    years: BTreeSet<jiff::civil::Date>,
}

impl IipKeys {
    /// Retrieve the value sets from each struct field in the source data.
    #[allow(clippy::type_complexity)]
    #[tracing::instrument]
    fn expected<'a, P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<
        (
            BTreeSet<Component>,
            BTreeSet<ItaFrequency>,
            BTreeSet<Investment>,
            BTreeSet<jiff::civil::Date>,
        ),
        BeaErr,
    > {
        let path = path.as_ref().to_owned();
        let data = Iip::try_from(&path)?;
        let comp = data.components();
        let freq = data.frequencies();
        let inv = data.investments();
        let year = data.years();
        Ok((comp, freq, inv, year))
    }

    /// Attempts to load all files in the download [`History`], without respect to the load `History`.
    /// Loads IIP files, converts struct fields to BTree hash maps or sets.
    /// Serializes the results to the `BEA_DATA` directory.
    #[tracing::instrument(skip_all)]
    pub async fn observed() -> Result<Self, BeaErr> {
        let dataset = Dataset::Iip;
        let obs = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded.", obs.len());
        let mut cl_units = BTreeSet::new();
        let mut components = BTreeSet::new();
        let mut frequencies = BTreeSet::new();
        let mut notes = BTreeSet::new();
        let mut time_periods = BTreeSet::new();
        let mut time_series_codes = std::collections::BTreeMap::new();
        let mut investments = BTreeSet::new();
        let mut unit_multipliers = BTreeSet::new();
        let mut years = BTreeSet::new();
        obs.iter()
            .map(|v| {
                if let Data::Iip(data) = v {
                    cl_units.append(&mut data.cl_units());
                    components.append(&mut data.components());
                    frequencies.append(&mut data.frequencies());
                    if let Some(value) = &mut data.notes() {
                        notes.append(value);
                    }
                    time_periods.append(&mut data.time_periods());
                    time_series_codes.append(&mut data.time_series_codes());
                    investments.append(&mut data.investments());
                    unit_multipliers.append(&mut data.unit_multipliers());
                    years.append(&mut data.years());
                }
            })
            .for_each(drop);
        let notes = if notes.is_empty() { None } else { Some(notes) };

        Ok(Self::new(
            cl_units,
            components,
            frequencies,
            notes,
            time_periods,
            time_series_codes,
            investments,
            unit_multipliers,
            years,
        ))
    }

    /// Print the value sets from each struct field in the source data to the BEA_DATA directory.
    #[tracing::instrument]
    pub fn print_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::Iip;
        let kind = "Expected";
        let (comp, freq, inv, year) = Self::expected(path)?;
        let name = ParameterName::Component;
        params(&comp, path, dataset, name, kind)?;
        let name = ParameterName::Frequency;
        params(&freq, path, dataset, name, kind)?;
        let name = ParameterName::Investment;
        params(&inv, path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(&year, path, dataset, name, kind)?;
        Ok(())
    }

    /// Compare expected values to enum representation.  Print missing and unused parameters to the
    /// BEA_DATA directory.
    #[tracing::instrument]
    pub fn check_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::Iip;
        // BEA provided paramater name keys for table id and year
        let (comp, freq, inv, _year) = Self::expected(path)?;
        // load source data
        let comps = Component::iter().collect::<std::collections::BTreeSet<Component>>();
        let freqs = ItaFrequency::iter().collect::<std::collections::BTreeSet<ItaFrequency>>();
        let invs = Investment::iter().collect::<std::collections::BTreeSet<Investment>>();

        let unused = "UnusedParams";
        let missing = "MissingParams";
        let name = ParameterName::Component;
        difference(&comp, &comps, path, dataset, name, unused)?;
        difference(&comps, &comp, path, dataset, name, missing)?;
        let name = ParameterName::Frequency;
        difference(&freq, &freqs, path, dataset, name, unused)?;
        difference(&freqs, &freq, path, dataset, name, missing)?;
        let name = ParameterName::Investment;
        difference(&inv, &invs, path, dataset, name, unused)?;
        difference(&invs, &inv, path, dataset, name, missing)?;
        Ok(())
    }

    /// Prints set members of type struct fields from source data to the `BEA_DATA` directory.
    #[tracing::instrument(skip_all)]
    pub async fn print_observed<P: AsRef<std::path::Path>>(path: P) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::Iip;
        let kind = "Observed";
        let obs = Self::observed().await?;

        let name = ParameterName::ClUnit;
        params(obs.cl_units(), path, dataset, name, kind)?;
        let name = ParameterName::Component;
        params(obs.components(), path, dataset, name, kind)?;
        let name = ParameterName::Frequency;
        params(obs.frequencies(), path, dataset, name, kind)?;
        let name = ParameterName::Notes;
        params(obs.notes(), path, dataset, name, kind)?;
        let name = ParameterName::TimePeriod;
        params(obs.time_periods(), path, dataset, name, kind)?;
        let name = ParameterName::TimeSeriesCode;
        params(obs.time_series_codes(), path, dataset, name, kind)?;
        let name = ParameterName::Investment;
        params(obs.investments(), path, dataset, name, kind)?;
        let name = ParameterName::UnitMult;
        params(obs.unit_multipliers(), path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(obs.years(), path, dataset, name, kind)?;
        Ok(())
    }

    /// Checks that all struct field values in source data are present as variants of their corresponding enum.
    /// Also warns on unused variants in the associated enum.
    #[tracing::instrument(skip_all)]
    pub async fn check_observed<P: AsRef<std::path::Path>>(path: P) -> Result<(), BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::Iip;
        // sets of codes within source data
        let obs = Self::observed().await?;
        let (comp, freq, inv, year) = Self::expected(path)?;
        // Is the source code a member of the corresponding enum?
        // Print source codes not in Naics or Frequency to bea_data "Missing"
        let unused = "Unused";
        let missing = "Missing";
        let name = ParameterName::Component;
        difference(&comp, obs.components(), path, dataset, name, unused)?;
        difference(obs.components(), &comp, path, dataset, name, missing)?;
        let name = ParameterName::Frequency;
        difference(&freq, obs.frequencies(), path, dataset, name, unused)?;
        difference(obs.frequencies(), &freq, path, dataset, name, missing)?;
        let name = ParameterName::Investment;
        difference(&inv, obs.investments(), path, dataset, name, unused)?;
        difference(obs.investments(), &inv, path, dataset, name, missing)?;
        let name = ParameterName::Year;
        difference(&year, obs.years(), path, dataset, name, unused)?;
        difference(obs.years(), &year, path, dataset, name, missing)?;
        Ok(())
    }
}
