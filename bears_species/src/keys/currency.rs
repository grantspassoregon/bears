use crate::Measure;

/// Scale factor associated with a currency value.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::FromStr,
    derive_more::Display,
)]
pub enum Scale {
    /// Units in millions, multiplying base values by a factor of six.
    Million,
    /// Base value units, equivalent to multiplying by a factor of one.
    #[default]
    Unit,
}

impl Scale {
    /// Unit multiplier for data values.
    #[tracing::instrument(skip_all)]
    pub fn factor(&self) -> i64 {
        match self {
            Self::Million => 6,
            Self::Unit => 1,
        }
    }

    /// Attempts to create an instance of Self from an i64.
    #[tracing::instrument]
    pub fn from_key(key: i64) -> Result<Self, BadScale> {
        let scale = match key {
            6 => Self::Million,
            0 => Self::Unit,
            _ => return Err(BadScale::new(key, line!(), file!().to_owned())),
        };
        Ok(scale)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("Bad Scale value: {value} at line {line} in {file}")]
pub struct BadScale {
    value: i64,
    line: u32,
    file: String,
}

impl std::error::Error for BadScale {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Variants contain different data value types, with methods to retrieve values by type.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
pub enum ValueKind {
    /// Decimal or floating point values.
    #[from(f64)]
    Decimal(f64),
    /// Integer values.
    #[from(i64)]
    Unit(i64),
}

impl ValueKind {
    /// Getter for floating point values stored in Self.
    #[tracing::instrument(skip_all)]
    pub fn decimal(&self) -> Option<f64> {
        match self {
            Self::Decimal(value) => Some(*value),
            _ => None,
        }
    }

    /// Getter for unit values stored in Self.
    #[tracing::instrument(skip_all)]
    pub fn unit(&self) -> Option<i64> {
        match self {
            Self::Unit(value) => Some(*value),
            _ => None,
        }
    }
}

impl std::fmt::Display for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(value) => write!(f, "{value}"),
            Self::Decimal(value) => write!(f, "{value}"),
        }
    }
}

/// The `Currency` type represents BEA values by embedding the raw data value with contextual
/// metadata related to unit multipliers and currency standard.
/// Prevents misinterpretation of raw data values and facilitates conversion between value types.
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[display("{value}")]
pub struct Currency {
    /// A typed wrapper around the raw data value.
    #[deref]
    #[deref_mut]
    value: ValueKind,
    /// The scale factor or unit multiplier associated with the data value.
    scale: Scale,
    /// The currency standard associated with the data value.
    unit: Measure,
}

impl Default for Currency {
    fn default() -> Self {
        let value = ValueKind::from(0);
        let scale = Scale::default();
        let unit = Measure::default();
        Self { value, scale, unit }
    }
}

impl From<(i64, Scale, Measure)> for Currency {
    fn from(value: (i64, Scale, Measure)) -> Self {
        let unit = value.2;
        let scale = value.1;
        let value = ValueKind::from(value.0);
        Self { unit, scale, value }
    }
}

impl From<(f64, Scale, Measure)> for Currency {
    fn from(value: (f64, Scale, Measure)) -> Self {
        let unit = value.2;
        let scale = value.1;
        let value = ValueKind::from(value.0);
        Self { unit, scale, value }
    }
}
