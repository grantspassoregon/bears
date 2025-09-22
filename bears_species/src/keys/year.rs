use crate::{
    BeaErr, Frequency, JsonParseError, JsonParseErrorKind, MneDoi, NipaYear, ParameterFields,
    ParameterValueTable, ParameterValueTableVariant, ParseInt,
};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}", self.date.year())]
pub struct Year {
    date: jiff::civil::Date,
    description: String,
}

impl Year {
    pub fn key(&self) -> String {
        self.date.year().to_string()
    }
}

impl TryFrom<&ParameterFields> for Year {
    type Error = ParseInt;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let date = parse_year(value.key())?;
        Ok(Self::new(date, value.desc().into()))
    }
}

impl TryFrom<&MneDoi> for Year {
    type Error = ParseInt;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let date = parse_year(value.key())?;
        Ok(Self::new(date, value.desc().into()))
    }
}

pub fn parse_year(input: &str) -> Result<jiff::civil::Date, ParseInt> {
    match input.parse::<i16>() {
        Ok(num) => {
            let date = jiff::civil::date(num, 1, 1);
            Ok(date)
        }
        Err(source) => {
            let error = ParseInt::new(input.into(), source, line!(), file!().into());
            Err(error)
        }
    }
}

/// Converts Roman numeral notation to a [`Date`](jiff::civil::Date) from the `jiff` crate.
///
/// In the GDPbyIndustry tables, the BEA reports the relevant quarter in Roman numerals (I-IV).
/// This function will return `None` if the `quarter` parameter does not contain either "I", "II",
/// "III" or "IV".  
///
/// Requires the `year` argument to derive the year componenet of the `Date` type.
/// Since the BEA return data includes a `Year` field, we first convert that `String` to a `jiff`
/// `Data` type, then derive the value of `Quarter` using `Year` as the context.
pub fn roman_numeral_quarter(quarter: &str, year: jiff::civil::Date) -> Option<jiff::civil::Date> {
    let duration = match quarter {
        "I" => jiff::Span::new(),
        "II" => jiff::Span::new().months(3),
        "III" => jiff::Span::new().months(6),
        "IV" => jiff::Span::new().months(9),
        _ => return None,
    };
    Some(year + duration)
}

pub fn date_by_quarter(input: &str) -> Option<jiff::civil::Date> {
    if let Some((q, _)) = input.match_indices("Q").next() {
        let (year, quarter) = input.split_at(q);
        match year.parse::<i16>() {
            Ok(num) => match quarter {
                "Q1" => Some(jiff::civil::date(num, 1, 1)),
                "Q2" => Some(jiff::civil::date(num, 4, 1)),
                "Q3" => Some(jiff::civil::date(num, 7, 1)),
                "Q4" => Some(jiff::civil::date(num, 10, 1)),
                other => {
                    let error = NotQuarter::new(
                        format!("Q* pattern expected, found {other}"),
                        line!(),
                        file!().to_string(),
                    );
                    let error = JsonParseErrorKind::from(error);
                    let error = JsonParseError::from(error);
                    tracing::error!("{error}");
                    None
                }
            },
            Err(source) => {
                let error = ParseInt::new(input.into(), source, line!(), file!().into());
                tracing::error!("{error}");
                None
            }
        }
    } else {
        None
    }
}

pub fn date_by_month(input: &str) -> Option<jiff::civil::Date> {
    if let Some((m, _)) = input.match_indices("M").next() {
        let (year, quarter) = input.split_at(m);
        match year.parse::<i16>() {
            Ok(num) => match quarter {
                "M01" => Some(jiff::civil::date(num, 1, 1)),
                "M02" => Some(jiff::civil::date(num, 2, 1)),
                "M03" => Some(jiff::civil::date(num, 3, 1)),
                "M04" => Some(jiff::civil::date(num, 4, 1)),
                "M05" => Some(jiff::civil::date(num, 5, 1)),
                "M06" => Some(jiff::civil::date(num, 6, 1)),
                "M07" => Some(jiff::civil::date(num, 7, 1)),
                "M08" => Some(jiff::civil::date(num, 8, 1)),
                "M09" => Some(jiff::civil::date(num, 9, 1)),
                "M10" => Some(jiff::civil::date(num, 10, 1)),
                "M11" => Some(jiff::civil::date(num, 11, 1)),
                "M12" => Some(jiff::civil::date(num, 12, 1)),
                other => {
                    tracing::error!("Unexpected month value: {other}");
                    None
                }
            },
            Err(source) => {
                let error = ParseInt::new(input.into(), source, line!(), file!().into());
                tracing::error!("{error}");
                None
            }
        }
    } else {
        None
    }
}

pub fn date_by_period(input: &str) -> Result<jiff::civil::Date, BeaErr> {
    if let Some(date) = date_by_quarter(input) {
        Ok(date)
    } else if let Some(date) = date_by_month(input) {
        Ok(date)
    } else {
        Ok(parse_year(input)?)
    }
}

impl TryFrom<&ParameterValueTable> for Year {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "ParameterFields needed".to_string(),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

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
    derive_more::From,
)]
pub enum YearKind {
    #[default]
    All,
    Year(Year),
    #[from(NipaRange)]
    Range(NipaRange),
    #[from(NipaRanges)]
    Ranges(NipaRanges),
}

impl YearKind {
    pub fn keys(&self) -> YearKindIterator<'_> {
        YearKindIterator::from(self)
    }
}

impl TryFrom<&ParameterFields> for YearKind {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        match Year::try_from(value) {
            Ok(year) => Ok(Self::Year(year)),
            Err(_) => match value.key().as_str() {
                "all" => Ok(Self::All),
                other => {
                    let error = YearInvalid::new(other.into(), line!(), file!().to_string());
                    Err(error.into())
                }
            },
        }
    }
}

impl TryFrom<&MneDoi> for YearKind {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match Year::try_from(value) {
            Ok(year) => Ok(Self::Year(year)),
            Err(_) => match value.key().as_str() {
                "all" => Ok(Self::All),
                other => {
                    let error = YearInvalid::new(other.into(), line!(), file!().to_string());
                    Err(error.into())
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct YearKindIterator<'a> {
    kind: &'a YearKind,
    range: Option<NipaRangeIterator<'a>>,
    ranges: Option<NipaRangesIterator<'a>>,
    finish: bool,
}

impl<'a> YearKindIterator<'a> {
    pub fn new(kind: &'a YearKind) -> Self {
        Self {
            kind,
            range: None,
            ranges: None,
            finish: false,
        }
    }
}

impl<'a> From<&'a YearKind> for YearKindIterator<'a> {
    fn from(value: &'a YearKind) -> Self {
        Self::new(value)
    }
}

impl Iterator for YearKindIterator<'_> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finish {
            return None;
        }
        match self.kind {
            YearKind::All => {
                self.finish = true;
                Some(vec!["ALL".to_string()])
            }
            YearKind::Year(year) => {
                self.finish = true;
                Some(vec![year.key()])
            }
            YearKind::Range(range) => {
                self.range = Some(range.iter());
                if let Some(rng) = &mut self.range {
                    match rng.next() {
                        Some(next) => Some(next),
                        None => {
                            self.finish = true;
                            None
                        }
                    }
                } else {
                    tracing::error!("Range failed to set.");
                    None
                }
            }
            YearKind::Ranges(ranges) => {
                self.ranges = Some(ranges.iter());
                if let Some(rng) = &mut self.ranges {
                    match rng.next() {
                        Some(next) => Some(next),
                        None => {
                            self.finish = true;
                            None
                        }
                    }
                } else {
                    tracing::error!("Ranges failed to set.");
                    None
                }
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct YearOptions {
    key: String,
    kind: YearKind,
}

impl From<YearKind> for YearOptions {
    fn from(kind: YearKind) -> Self {
        let key = kind
            .keys()
            .next()
            .expect("Each impl has at least one valid key.");
        // strings of keys have at least one value, so this won't panic.
        // intended use is to return ALL for the default value
        let key = key[0].clone();
        Self { key, kind }
    }
}

impl Default for YearOptions {
    fn default() -> Self {
        let kind = YearKind::default();
        Self::from(kind)
    }
}

impl TryFrom<&ParameterFields> for YearOptions {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = YearKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&MneDoi> for YearOptions {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = YearKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for YearOptions {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("ParameterFields or MneDoi needed, found {other:#?}"),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}-{}", self.first.year(), self.last.year())]
pub struct YearRange {
    first: jiff::civil::Date,
    last: jiff::civil::Date,
}

impl YearRange {
    pub fn keys(&self) -> Vec<String> {
        let first = self.first.year();
        let last = self.last.year();
        (first..=last).map(|year| year.to_string()).collect()
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
    serde::Deserialize,
    serde::Serialize,
)]
#[setters(prefix = "with_", strip_option)]
pub struct NipaRange {
    annual: Option<YearRange>,
    monthly: Option<YearRange>,
    quarterly: Option<YearRange>,
}

impl NipaRange {
    pub fn iter(&self) -> NipaRangeIterator<'_> {
        NipaRangeIterator::new(self)
    }
}

pub fn year_opt(input: &str) -> Result<Option<jiff::civil::Date>, ParseInt> {
    match input {
        "0" => Ok(None),
        num => Ok(Some(parse_year(num)?)),
    }
}

impl TryFrom<&NipaYear> for NipaRange {
    type Error = ParseInt;
    fn try_from(value: &NipaYear) -> Result<Self, Self::Error> {
        let from = year_opt(value.first_annual_year())?;
        let to = year_opt(value.last_annual_year())?;
        let annual = match (from, to) {
            (Some(first), Some(last)) => Some(YearRange::new(first, last)),
            _ => None,
        };
        let from = year_opt(value.first_monthly_year())?;
        let to = year_opt(value.last_monthly_year())?;
        let monthly = match (from, to) {
            (Some(first), Some(last)) => Some(YearRange::new(first, last)),
            _ => None,
        };
        let from = year_opt(value.first_quarterly_year())?;
        let to = year_opt(value.last_quarterly_year())?;
        let quarterly = match (from, to) {
            (Some(first), Some(last)) => Some(YearRange::new(first, last)),
            _ => None,
        };
        Ok(Self::new(annual, monthly, quarterly))
    }
}

impl TryFrom<&ParameterValueTable> for NipaRange {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::NipaYear(nipa_year) => Ok(Self::try_from(nipa_year)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "NipaFrequency needed".to_string(),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct NipaRangeIterator<'a> {
    range: &'a NipaRange,
    frequency: Frequency,
    finish: bool,
}

impl<'a> NipaRangeIterator<'a> {
    pub fn new(range: &'a NipaRange) -> Self {
        let frequency = Frequency::Annual;
        Self {
            range,
            frequency,
            finish: false,
        }
    }
}

impl Iterator for NipaRangeIterator<'_> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.frequency {
            Frequency::Annual => {
                self.frequency = Frequency::Monthly;
                if let Some(range) = self.range.annual() {
                    Some(range.keys())
                } else {
                    self.next()
                }
            }
            Frequency::Monthly => {
                self.frequency = Frequency::Quarterly;
                if let Some(range) = self.range.quarterly() {
                    Some(range.keys())
                } else {
                    self.next()
                }
            }
            Frequency::Quarterly => {
                if self.finish {
                    return None;
                }
                if let Some(range) = self.range.quarterly() {
                    self.finish = true;
                    Some(range.keys())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    serde::Deserialize,
    serde::Serialize,
)]
#[from(std::collections::BTreeMap<String, NipaRange>)]
pub struct NipaRanges(std::collections::BTreeMap<String, NipaRange>);

impl NipaRanges {
    pub fn iter(&self) -> NipaRangesIterator<'_> {
        NipaRangesIterator::new(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct NipaRangesIterator<'a> {
    ranges: &'a NipaRanges,
    keys: Vec<&'a String>,
    index: usize,
    range: Option<NipaRangeIterator<'a>>,
}

impl<'a> NipaRangesIterator<'a> {
    pub fn new(ranges: &'a NipaRanges) -> Self {
        let keys = ranges.keys().collect::<Vec<&String>>();
        let range = if let Some(k) = ranges.get(keys[0]) {
            Some(k.iter())
        } else {
            tracing::error!("Range iterator should be set.");
            None
        };
        Self {
            ranges,
            keys,
            index: 0,
            range,
        }
    }
}

impl Iterator for NipaRangesIterator<'_> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(range) = &mut self.range {
            match range.next() {
                Some(keys) => Some(keys),
                None => {
                    if self.index < self.keys.len() - 1 {
                        self.index += 1;
                    } else {
                        return None;
                    }
                    if let Some(rng) = self.ranges.get(self.keys[self.index]) {
                        self.range = Some(rng.iter());
                    } else {
                        tracing::error!("Range should have valid values.");
                        self.range = None;
                    }
                    if let Some(r) = &mut self.range {
                        r.next()
                    } else {
                        tracing::error!("Range should have valid values.");
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid Year at line {line} in file {file}")]
pub struct YearInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for YearInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("Not Quarter error: {clue} at line {line} in {file}")]
pub struct NotQuarter {
    clue: String,
    line: u32,
    file: String,
}

impl std::error::Error for NotQuarter {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
