use bears_species::{BeaErr, Dataset, IoError, ParameterName, write_json};
use std::collections::BTreeSet;

#[tracing::instrument(skip_all)]
pub fn params<T: Ord + serde::Serialize, P: AsRef<std::path::Path>, N: std::fmt::Display>(
    data: &T,
    path: P,
    dataset: Dataset,
    name: N,
    kind: &str,
) -> Result<(), BeaErr> {
    let path = path.as_ref();
    let path = path.join("key_sets");
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("Key sets directory created.");
    }
    let target = path.join(format!("{dataset}_{name}_{kind}.json"));
    write_json(data, target)?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn difference<T: Ord + serde::Serialize, P: AsRef<std::path::Path>>(
    left: &BTreeSet<T>,
    right: &BTreeSet<T>,
    path: P,
    dataset: Dataset,
    name: ParameterName,
    kind: &str,
) -> Result<(), BeaErr> {
    let path = path.as_ref();
    let path = path.join("key_sets");
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("Key sets directory created.");
    }
    let differ = left
        .difference(right)
        .collect::<std::collections::BTreeSet<&T>>();
    if !differ.is_empty() {
        let target = path.join(format!("{dataset}_{name}_{kind}.json"));
        write_json(&differ, target)?;
        tracing::error!(
            "{kind} parameter keys for {name} in {dataset} printed to {}.",
            path.display()
        );
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn unused<T: Ord + serde::Serialize, P: AsRef<std::path::Path>>(
    expected: &BTreeSet<T>,
    observed: &BTreeSet<T>,
    path: P,
    dataset: Dataset,
    name: ParameterName,
) -> Result<(), BeaErr> {
    let path = path.as_ref();
    let path = path.join("key_sets");
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("Key sets directory created.");
    }
    let unused = expected
        .difference(observed)
        .collect::<std::collections::BTreeSet<&T>>();
    if !unused.is_empty() {
        let target = path.join(format!("{dataset}_{name}_Unused.json"));
        write_json(&unused, target)?;
        tracing::error!(
            "Unused parameter keys for {name} in {dataset} printed to BEA_DATA directory."
        );
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn missing<T: Ord + serde::Serialize, P: AsRef<std::path::Path>>(
    expected: &BTreeSet<T>,
    observed: &BTreeSet<T>,
    path: P,
    dataset: Dataset,
    name: ParameterName,
) -> Result<(), BeaErr> {
    let path = path.as_ref();
    let path = path.join("key_sets");
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("Key sets directory created.");
    }
    let missing = observed
        .difference(expected)
        .collect::<std::collections::BTreeSet<&T>>();
    if !missing.is_empty() {
        let target = path.join(format!("{dataset}_{name}_Missing.json"));
        write_json(&missing, target)?;
        tracing::error!(
            "Missing parameter keys for {name} in {dataset} printed to BEA_DATA directory."
        );
    }
    Ok(())
}
