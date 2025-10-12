use crate::{
    AnnotationMissing, BadMetric, BadScale, BoolInvalid, IntegerInvalid, Mismatch, Nom, NotFloat,
    NotInteger, NotParameterName, NotQuarter, OwnershipInvalid, ParseFloat, ParseInteger,
    RowCodeMissing, UrlParseError, YearInvalid,
};

#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct BeaErr {
    kind: Box<BeaErrorKind>,
}

impl std::fmt::Display for BeaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for BeaErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

impl From<BeaErrorKind> for BeaErr {
    fn from(value: BeaErrorKind) -> Self {
        let kind = Box::new(value);
        Self { kind }
    }
}

macro_rules! impl_bea_err {
    ( $( $name:ident),* ) => {
        $(
            impl From<$name> for BeaErr {
                fn from(value: $name) -> Self {
                    let kind = BeaErrorKind::from(value).into();
                    Self { kind }
                }
            }
        )*
    };
    ( $( $name:ident),+ ,) => {
       impl_bea_err![ $( $name ),* ];
    };
}

impl_bea_err!(
    AnnotationMissing,
    BadMetric,
    BadScale,
    BoolInvalid,
    BTreeKeyMissing,
    Check,
    Csv,
    DatasetMissing,
    DeriveFromStr,
    EnvError,
    IntegerInvalid,
    InvestmentInvalid,
    IoError,
    Jiff,
    JsonParseError,
    Mismatch,
    Nom,
    OwnershipInvalid,
    ParameterValueTableVariant,
    ParseInt,
    Progress,
    RateLimit,
    ReqwestError,
    RowCodeMissing,
    SerdeJson,
    Set,
    UrlParseError,
    VariantMissing,
    YearInvalid,
);

macro_rules! impl_json_to_bea_err {
    ( $( $name:ident),* ) => {
        $(
            impl From<$name> for BeaErr {
                fn from(value: $name) -> Self {
                    let kind = JsonParseError::from(value);
                    let kind = BeaErrorKind::from(kind).into();
                    Self { kind }
                }
            }
        )*
    };
    ( $( $name:ident),+ ,) => {
       impl_json_to_bea_err![ $( $name ),* ];
    };
}

impl_json_to_bea_err!(NotArray, NotObject, KeyMissing);

#[derive(Debug, derive_more::From)]
pub enum BeaErrorKind {
    #[from(AnnotationMissing)]
    AnnotationMissing(AnnotationMissing),
    #[from(BadMetric)]
    BadMetric(BadMetric),
    #[from(BadScale)]
    BadScale(BadScale),
    #[from(BoolInvalid)]
    BoolInvalid(BoolInvalid),
    #[from(BTreeKeyMissing)]
    BTreeKeyMissing(BTreeKeyMissing),
    #[from(Check)]
    Check(Check),
    #[from(Csv)]
    Csv(Csv),
    #[from(DatasetMissing)]
    DatasetMissing(DatasetMissing),
    #[from(DeriveFromStr)]
    DeriveFromStr(DeriveFromStr),
    #[from(EnvError)]
    Env(EnvError),
    #[from(IntegerInvalid)]
    IntegerInvalid(IntegerInvalid),
    #[from(InvestmentInvalid)]
    InvestmentInvalid(InvestmentInvalid),
    #[from(IoError)]
    Io(IoError),
    #[from(Jiff)]
    Jiff(Jiff),
    #[from(JsonParseError)]
    JsonParse(JsonParseError),
    #[from(Mismatch)]
    Mismatch(Mismatch),
    #[from(Nom)]
    Nom(Nom),
    #[from(OwnershipInvalid)]
    OwnershipInvalid(OwnershipInvalid),
    #[from(ParameterValueTableVariant)]
    ParameterValueTableVariant(ParameterValueTableVariant),
    #[from(ParseInt)]
    ParseInt(ParseInt),
    #[from(Progress)]
    Progress(Progress),
    #[from(RateLimit)]
    RateLimit(RateLimit),
    #[from(ReqwestError)]
    Reqwest(ReqwestError),
    #[from(RowCodeMissing)]
    RowCodeMissing(RowCodeMissing),
    #[from(Set)]
    Set(Set),
    #[from(SerdeJson)]
    SerdeJson(SerdeJson),
    #[from(UrlParseError)]
    UrlParse(UrlParseError),
    #[from(VariantMissing)]
    VariantMissing(VariantMissing),
    #[from(YearInvalid)]
    YearInvalid(YearInvalid),
}

impl std::fmt::Display for BeaErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnnotationMissing(e) => {
                write!(f, "{e}")
            }
            Self::BadMetric(e) => {
                write!(f, "{e}")
            }
            Self::BadScale(e) => {
                write!(f, "{e}")
            }
            Self::BoolInvalid(e) => {
                write!(f, "{e}")
            }
            Self::BTreeKeyMissing(e) => {
                write!(f, "{e}")
            }
            Self::Check(e) => {
                write!(f, "{e}")
            }
            Self::Csv(e) => {
                write!(f, "{e}")
            }
            Self::DatasetMissing(e) => {
                write!(f, "{e}")
            }
            Self::DeriveFromStr(e) => {
                write!(f, "{e}")
            }
            Self::Env(e) => {
                write!(f, "{e}")
            }
            Self::IntegerInvalid(e) => {
                write!(f, "{e}")
            }
            Self::InvestmentInvalid(e) => {
                write!(f, "{e}")
            }
            Self::Io(e) => {
                write!(f, "{e}")
            }
            Self::Jiff(e) => {
                write!(f, "{e}")
            }
            Self::JsonParse(e) => {
                write!(f, "{e}")
            }
            Self::Mismatch(e) => {
                write!(f, "{e}")
            }
            Self::Nom(e) => {
                write!(f, "{e}")
            }
            Self::OwnershipInvalid(e) => {
                write!(f, "{e}")
            }
            Self::ParameterValueTableVariant(e) => {
                write!(f, "{e}")
            }
            Self::ParseInt(e) => {
                write!(f, "{e}")
            }
            Self::Progress(e) => {
                write!(f, "{e}")
            }
            Self::RateLimit(e) => {
                write!(f, "{e}")
            }
            Self::Reqwest(e) => {
                write!(f, "{e}")
            }
            Self::RowCodeMissing(e) => {
                write!(f, "{e}")
            }
            Self::Set(e) => {
                write!(f, "{e}")
            }
            Self::SerdeJson(e) => {
                write!(f, "{e}")
            }
            Self::UrlParse(e) => {
                write!(f, "{e}")
            }
            Self::VariantMissing(e) => {
                write!(f, "{e}")
            }
            Self::YearInvalid(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for BeaErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AnnotationMissing(e) => e.source(),
            Self::BadMetric(e) => e.source(),
            Self::BadScale(e) => e.source(),
            Self::BoolInvalid(e) => e.source(),
            Self::BTreeKeyMissing(e) => e.source(),
            Self::Check(e) => e.source(),
            Self::Csv(e) => e.source(),
            Self::DatasetMissing(e) => e.source(),
            Self::DeriveFromStr(e) => e.source(),
            Self::Env(e) => Some(e.source()),
            Self::IntegerInvalid(e) => e.source(),
            Self::InvestmentInvalid(e) => e.source(),
            Self::Io(e) => Some(e.source()),
            Self::Jiff(e) => e.source(),
            Self::JsonParse(e) => e.source(),
            Self::Mismatch(e) => e.source(),
            Self::Nom(e) => e.source(),
            Self::OwnershipInvalid(e) => e.source(),
            Self::ParameterValueTableVariant(e) => e.source(),
            Self::ParseInt(e) => Some(e.source()),
            Self::Progress(e) => e.source(),
            Self::RateLimit(e) => e.source(),
            Self::Reqwest(e) => Some(e.source()),
            Self::RowCodeMissing(e) => e.source(),
            Self::Set(e) => e.source(),
            Self::SerdeJson(e) => Some(e.source()),
            Self::UrlParse(e) => Some(e.source()),
            Self::VariantMissing(e) => e.source(),
            Self::YearInvalid(e) => e.source(),
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum JsonParseErrorKind {
    #[from(FromStrError)]
    FromStr(FromStrError),
    #[from(KeyMissing)]
    KeyMissing(KeyMissing),
    #[from(NotArray)]
    NotArray(NotArray),
    NotBool,
    #[from(NotFloat)]
    NotFloat(NotFloat),
    #[from(NotInteger)]
    NotInteger(NotInteger),
    #[from(NotObject)]
    NotObject(NotObject),
    #[from(NotParameterName)]
    NotParameterName(NotParameterName),
    #[from(NotQuarter)]
    NotQuarter(NotQuarter),
    NotString,
    #[from(ParseFloat)]
    ParseFloat(ParseFloat),
    #[from(ParseInteger)]
    ParseInteger(ParseInteger),
}

impl std::fmt::Display for JsonParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromStr(e) => write!(f, "problem converting str to json: {e}"),
            Self::KeyMissing(e) => write!(f, "{e}"),
            Self::NotArray(e) => write!(f, "{e}"),
            Self::NotBool => write!(f, "serde_json::Value is not a Number or String variant"),
            Self::NotFloat(e) => write!(f, "{e}"),
            Self::NotInteger(e) => write!(f, "{e}"),
            Self::NotObject(e) => write!(f, "{e}"),
            Self::NotParameterName(e) => write!(f, "{e}"),
            Self::NotQuarter(e) => write!(f, "{e}"),
            Self::NotString => write!(f, "serde_json::Value is not a String variant"),
            Self::ParseFloat(e) => write!(f, "{e}"),
            Self::ParseInteger(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for JsonParseErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<JsonParseErrorKind> for JsonParseError {
    fn from(kind: JsonParseErrorKind) -> Self {
        Self { kind }
    }
}

macro_rules! impl_json_parse_error {
    ( $( $name:ident),* ) => {
        $(
            impl From<$name> for JsonParseError {
                fn from(value: $name) -> Self {
                    let error = JsonParseErrorKind::from(value);
                    Self::from(error)
                }
            }
        )*
    };
    ( $( $name:ident),+ ,) => {
       impl_json_parse_error![ $( $name ),* ];
    };
}

impl_json_parse_error!(
    FromStrError,
    KeyMissing,
    NotArray,
    NotFloat,
    NotInteger,
    NotObject,
    NotParameterName,
    NotQuarter,
    ParseFloat,
    ParseInteger,
);

#[derive(Debug, derive_new::new, derive_more::Deref, derive_more::DerefMut)]
pub struct JsonParseError {
    kind: JsonParseErrorKind,
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for JsonParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug, derive_new::new)]
#[non_exhaustive]
pub struct FromStrError {
    value: String,
    source: serde_json::Error,
}

impl std::fmt::Display for FromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.value)
    }
}

impl std::error::Error for FromStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug, derive_getters::Getters, derive_setters::Setters)]
#[setters(prefix = "with_", strip_option, borrow_self)]
pub struct ReqwestError {
    pub url: String,
    pub method: String,
    pub headers: Option<Vec<(String, String)>>,
    pub body: Option<Vec<(String, String)>>,
    pub form: Option<Vec<(String, String)>>,
    pub source: reqwest::Error,
    line: u32,
    file: String,
}

impl ReqwestError {
    #[tracing::instrument(skip_all)]
    pub fn new<S: std::fmt::Display>(
        url: S,
        method: S,
        source: reqwest::Error,
        line: u32,
        file: String,
    ) -> Self {
        let url = url.to_string();
        let method = method.to_string();
        Self {
            url,
            method,
            headers: None,
            body: None,
            form: None,
            source,
            line,
            file,
        }
    }
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = "error in ".to_string();
        msg.push_str(&format!("{} request to {}", self.method, self.url));
        if let Some(headers) = self.headers.clone() {
            msg.push_str(&format!(" including headers {headers:?}"));
        }
        if let Some(body) = self.body.clone() {
            msg.push_str(&format!(" with body {body:?}"));
        }
        if let Some(form) = self.form.clone() {
            msg.push_str(&format!(" with form body {form:?}"));
        }
        msg.push_str(&format!(" at line {} in file {}", self.line, self.file));
        write!(f, "{msg}")
    }
}

impl std::error::Error for ReqwestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source())
    }
}

#[derive(
    Debug, derive_getters::Getters, derive_setters::Setters, derive_more::Display, derive_new::new,
)]
#[display("io error at path {path:?} in line {line} of {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct IoError {
    pub path: std::path::PathBuf,
    pub source: std::io::Error,
    line: u32,
    file: String,
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("serde_json error: {source} at line {line} in {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct SerdeJson {
    source: serde_json::Error,
    line: u32,
    file: String,
}

impl std::error::Error for SerdeJson {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug, derive_getters::Getters, derive_setters::Setters, derive_more::Display, derive_new::new,
)]
#[display("variant missing for dataset {name} at {line} in {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct DatasetMissing {
    name: String,
    line: u32,
    file: String,
}

impl std::error::Error for DatasetMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("Check failed: {}", self.desc)]
#[setters(prefix = "with_", borrow_self)]
#[from(String, &str)]
pub struct Check {
    pub desc: String,
}

impl std::error::Error for Check {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("error parsing input {input} to integer at line {line} in {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct ParseInt {
    input: String,
    source: std::num::ParseIntError,
    line: u32,
    file: String,
}

impl std::error::Error for ParseInt {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("{} at line {} in {}", self.issue, self.line, self.file)]
#[setters(prefix = "with_", borrow_self)]
pub struct ParameterValueTableVariant {
    issue: String,
    line: u32,
    file: String,
}

impl std::error::Error for ParameterValueTableVariant {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, derive_new::new, derive_more::Display, derive_more::Error)]
#[display("could not parse from {} into target value on line {} in {}", self.value, self.line, self.file)]
pub struct DeriveFromStr {
    value: String,
    source: derive_more::FromStrError,
    line: u32,
    file: String,
}

#[derive(Debug, Clone, derive_new::new, derive_more::Display, derive_more::Error)]
#[display("could not parse from {} into target type", self.value)]
pub struct Jiff {
    value: String,
    source: jiff::Error,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, strum::EnumIter,
)]
pub enum Set {
    Empty,
    ParameterFieldsMissing,
    ParameterNameMissing(String),
    ParameterValuesMissing,
}

impl std::error::Error for Set {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{clue} with {input} at line {line} in file {file}")]
pub struct VariantMissing {
    clue: String,
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for VariantMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid Investment at line {line} in file {file}")]
pub struct InvestmentInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for InvestmentInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("serde::Value::Object variant expected at line {line} in file {file}")]
pub struct NotObject {
    line: u32,
    file: String,
}

impl std::error::Error for NotObject {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("serde::Value::Array variant expected at line {line} in file {file}")]
pub struct NotArray {
    line: u32,
    file: String,
}

impl std::error::Error for NotArray {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("Key Missing error: {clue} at line {line} in {file}")]
pub struct KeyMissing {
    clue: String,
    line: u32,
    file: String,
}

impl std::error::Error for KeyMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::Error,
)]
#[display(".env file missing {target} at line {line} in {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct EnvError {
    target: String,
    source: std::env::VarError,
    line: u32,
    file: String,
}

/// The `Csv` struct contains error information associated with the `csv` crate.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("csv error at path {path:?} in line {line} of {file}")]
pub struct Csv {
    path: std::path::PathBuf,
    source: csv::Error,
    line: u32,
    file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("BTree Key Missing: {key} at line {line} in {file}")]
pub struct BTreeKeyMissing {
    key: String,
    line: u32,
    file: String,
}

impl std::error::Error for BTreeKeyMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("indicatif style template error for {key} in line {line} of {file}")]
pub struct Progress {
    key: String,
    source: indicatif::style::TemplateError,
    line: u32,
    file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("rate limit hit: {clue} at line {line} in {file}")]
pub struct RateLimit {
    clue: String,
    line: u32,
    file: String,
}

impl std::error::Error for RateLimit {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
