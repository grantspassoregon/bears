use crate::{
    NaicsCategory, NaicsIndustry, NaicsInputOutput, NaicsSector, NaicsSubcategory, NaicsSubsector,
    NaicsSupplement, VariantMissing,
};
use strum::IntoEnumIterator;

/// Parent enum for types of NAICS codes.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
pub enum Naics {
    #[from(NaicsSector)]
    Sector(NaicsSector),
    #[from(NaicsSubsector)]
    Subsector(NaicsSubsector),
    #[from(NaicsCategory)]
    Category(NaicsCategory),
    #[from(NaicsSubcategory)]
    Subcategory(NaicsSubcategory),
    #[from(NaicsIndustry)]
    Industry(NaicsIndustry),
    #[from(NaicsSupplement)]
    Supplement(NaicsSupplement),
    #[from(NaicsInputOutput)]
    InputOutput(NaicsInputOutput),
}

impl Naics {
    pub fn from_code(key: &str) -> Option<Self> {
        // check that the key is a number
        let code = match key.parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                // Check for input output row/column codes, which can include letters
                if let Some(naics) = NaicsInputOutput::from_code(key) {
                    return Some(naics.into());
                } else {
                    // No codes match, return empty-handed
                    return None;
                }
            }
        };

        // check if key is supplement type
        if let Some(supplement) = NaicsSupplement::from_code(key) {
            return Some(Self::from(supplement));
        }

        // match key to naics code by length
        match code {
            11..100 => NaicsSector::from_code(key).map(|naics| naics.into()),
            111..1000 => NaicsSubsector::from_code(key).map(|naics| naics.into()),
            1111..10_000 => NaicsCategory::from_code(key).map(|naics| naics.into()),
            11111..100_000 => NaicsSubcategory::from_code(key).map(|naics| naics.into()),
            111110..1_000_000 => NaicsIndustry::from_code(key).map(|naics| naics.into()),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Sector(naics) => naics.description(),
            Self::Subsector(naics) => naics.description(),
            Self::Category(naics) => naics.description(),
            Self::Subcategory(naics) => naics.description(),
            Self::Industry(naics) => naics.description(),
            Self::Supplement(naics) => naics.description(),
            Self::InputOutput(naics) => naics.description(),
        }
    }

    pub fn code(&self) -> String {
        let code = match self {
            Self::Sector(naics) => naics.code(),
            Self::Subsector(naics) => naics.code(),
            Self::Category(naics) => naics.code(),
            Self::Subcategory(naics) => naics.code(),
            Self::Industry(naics) => naics.code(),
            Self::Supplement(naics) => naics.code(),
            Self::InputOutput(naics) => return naics.code().to_owned(),
        };
        code.to_string()
    }

    pub fn variants() -> std::collections::BTreeSet<Self> {
        let mut variants = std::collections::BTreeSet::new();
        NaicsSector::iter()
            .map(|v| variants.insert(Naics::from(v)))
            .for_each(drop);
        NaicsSubsector::iter()
            .map(|v| variants.insert(Naics::from(v)))
            .for_each(drop);
        NaicsCategory::iter()
            .map(|v| variants.insert(Naics::from(v)))
            .for_each(drop);
        NaicsSubcategory::iter()
            .map(|v| variants.insert(Naics::from(v)))
            .for_each(drop);
        NaicsIndustry::iter()
            .map(|v| variants.insert(Naics::from(v)))
            .for_each(drop);
        NaicsSupplement::iter()
            .map(|v| variants.insert(Naics::from(v)))
            .for_each(drop);
        NaicsInputOutput::iter()
            .map(|v| variants.insert(Naics::from(v)))
            .for_each(drop);
        variants
    }

    /// Converts the given NAICS to its `NaicsSector` value.
    pub fn sector(&self) -> Option<NaicsSector> {
        // Sector is the shortest code, so all codes include a sector.
        let code = self.code().to_string();
        let key = code.chars().take(2).collect::<String>();
        NaicsSector::from_code(&key)
    }

    /// Converts a NAICS value to its `NaicsSubsector` value. Returns None if no subsector is
    /// associated with the code, or if the type is the Sector variant, which does not contain
    /// subsector codes.
    pub fn subsector(&self) -> Option<NaicsSubsector> {
        match self {
            Self::Sector(_) => None,
            Self::Subsector(ss) => Some(ss.to_owned()),
            _ => {
                let code = self.code().to_string();
                let key = code.chars().take(3).collect::<String>();
                NaicsSubsector::from_code(&key)
            }
        }
    }

    /// Converts a NAICS value to its `NaicsCategory` value. Returns None if no category is
    /// associated with the code, or if the type is the Sector or Subsector variants, which do not contain
    /// category codes.
    pub fn category(&self) -> Option<NaicsCategory> {
        match self {
            Self::Sector(_) => None,
            Self::Subsector(_) => None,
            Self::Category(c) => Some(c.to_owned()),
            _ => {
                let code = self.code().to_string();
                let key = code.chars().take(4).collect::<String>();
                NaicsCategory::from_code(&key)
            }
        }
    }

    /// Converts a NAICS value to its `NaicsSubcategory` value. Returns None if no subcategory is
    /// associated with the code, or if the type is the Sector, Subsector or Category variants, which do not contain
    /// subcategory codes.
    pub fn subcategory(&self) -> Option<NaicsSubcategory> {
        match self {
            Self::Sector(_) => None,
            Self::Subsector(_) => None,
            Self::Category(_) => None,
            Self::Subcategory(sc) => Some(sc.to_owned()),
            _ => {
                let code = self.code().to_string();
                let key = code.chars().take(5).collect::<String>();
                NaicsSubcategory::from_code(&key)
            }
        }
    }

    /// Converts a NAICS value to its `NaicsIndustry` value. Returns None if no indsutry is
    /// associated with the code, or if the type is the Sector, Subsector, Category or Subcategory variants, which do not contain
    /// industry codes.
    pub fn industry(&self) -> Option<NaicsIndustry> {
        match self {
            Self::Sector(_) => None,
            Self::Subsector(_) => None,
            Self::Category(_) => None,
            Self::Subcategory(_) => None,
            Self::Industry(i) => Some(i.to_owned()),
            _ => {
                let code = self.code().to_string();
                let key = code.chars().take(6).collect::<String>();
                NaicsIndustry::from_code(&key)
            }
        }
    }
}

impl std::str::FromStr for Naics {
    type Err = VariantMissing;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(naics) = NaicsSector::from_str(s) {
            Ok(Self::from(naics))
        } else if let Ok(naics) = NaicsSubsector::from_str(s) {
            Ok(Self::from(naics))
        } else if let Ok(naics) = NaicsCategory::from_str(s) {
            Ok(Self::from(naics))
        } else if let Ok(naics) = NaicsSubcategory::from_str(s) {
            Ok(Self::from(naics))
        } else if let Ok(naics) = NaicsIndustry::from_str(s) {
            Ok(Self::from(naics))
        } else if let Ok(naics) = NaicsSupplement::from_str(s) {
            Ok(Self::from(naics))
        } else if let Ok(naics) = NaicsInputOutput::from_str(s) {
            Ok(Self::from(naics))
        } else {
            let error = VariantMissing::new(
                "Naics".to_string(),
                s.to_owned(),
                line!(),
                file!().to_owned(),
            );
            Err(error)
        }
    }
}
