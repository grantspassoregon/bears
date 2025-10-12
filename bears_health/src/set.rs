use std::collections::{BTreeMap, BTreeSet};

pub trait Set {
    type Duplicate;
    type Histogram;

    fn histogram(&self) -> Self::Histogram;
    fn unique(&self) -> Result<(), Self::Duplicate>;
}

impl<T, U: Ord + Clone> Set for BTreeMap<T, U> {
    type Duplicate = BTreeSet<U>;
    type Histogram = BTreeMap<usize, BTreeSet<U>>;

    fn histogram(&self) -> Self::Histogram {
        let mut counts = BTreeMap::new();
        self.values()
            .map(|v| {
                *counts.entry(v.clone()).or_insert(0) += 1;
            })
            .for_each(drop);
        let mut hist = BTreeMap::<usize, BTreeSet<U>>::new();
        counts
            .iter()
            .map(|(key, value)| {
                if let Some(existing) = hist.get_mut(value) {
                    existing.insert(key.clone());
                } else {
                    let mut new_set = BTreeSet::new();
                    new_set.insert(key.clone());
                    hist.insert(*value, new_set);
                }
            })
            .for_each(drop);
        hist
    }

    fn unique(&self) -> Result<(), Self::Duplicate> {
        let mut unique = BTreeSet::new();
        let mut duplicate = BTreeSet::new();
        self.values()
            .map(|v| {
                if !unique.insert(v.to_owned()) {
                    duplicate.insert(v.to_owned());
                }
            })
            .for_each(drop);

        Ok(())
    }
}
