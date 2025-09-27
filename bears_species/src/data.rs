use crate::{
    AnnotatedInteger, BeaErr, BeaResponse, DatasetMissing, FixedAssetData, GdpData, IipData,
    InputOutputData, IoError, ItaData, KeyMissing, NaicsItems, NipaData, NotArray, NotObject,
    RowCode, SerdeJson, VariantMissing, map_to_float, map_to_int, map_to_string, parse_year,
};

#[derive(
    Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, derive_more::From,
)]
#[from(Vec<NipaData>)]
pub enum Data {
    #[from(NipaData)]
    Nipa(NipaData),
    #[from(FixedAssetData)]
    FixedAssets(FixedAssetData),
    #[from(MneDiData)]
    MneDi(MneDiData),
    #[from(GdpData)]
    Gdp(GdpData),
    #[from(ItaData)]
    ItaData(ItaData),
    #[from(IipData)]
    Iip(IipData),
    #[from(InputOutputData)]
    InputOutput(InputOutputData),
}

pub fn result_to_data(result: &serde_json::Value) -> Result<&serde_json::Value, BeaErr> {
    tracing::trace!("Reading results to data.");
    match result {
        serde_json::Value::Object(m) => {
            let key = "Data".to_string();
            if let Some(data) = m.get(&key) {
                Ok(data)
            } else {
                tracing::trace!("Parameter Value Table missing.");
                let error = KeyMissing::new(key, line!(), file!().to_string());
                Err(error.into())
            }
        }
        serde_json::Value::Array(v) => {
            tracing::trace!("Array detected: {} records.", v.len());
            if let Some(item) = v.first() {
                match item {
                    serde_json::Value::Object(m) => {
                        let key = "Data".to_string();
                        tracing::trace!("Object detected in array.");
                        if let Some(data) = m.get(&key) {
                            Ok(data)
                        } else {
                            let error = KeyMissing::new(key, line!(), file!().to_owned());
                            Err(error.into())
                        }
                    }
                    _ => {
                        let error = NotObject::new(line!(), file!().to_string());
                        Err(error.into())
                    }
                }
            } else {
                tracing::error!("Array should not be empty.");
                let error = KeyMissing::new("empty array".to_owned(), line!(), file!().to_owned());
                Err(error.into())
            }
        }
        _ => {
            tracing::trace!("Wrong Value type: {result:#?}");
            let error = NotObject::new(line!(), file!().to_string());
            Err(error.into())
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct MneDiDatum {
    column: String,
    column_code: i64,
    column_g_parent: String,
    column_g_parent_code: i64,
    column_parent: String,
    column_parent_code: i64,
    data_value: String,
    data_value_unformatted: AnnotatedInteger,
    row: String,
    row_code: RowCode,
    series_id: i64,
    series_name: String,
    table_column_display_order: f64,
    table_row_display_order: f64,
    table_scale: String,
    year: jiff::civil::Date,
}

impl MneDiDatum {
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
        naics: &NaicsItems,
    ) -> Result<Self, BeaErr> {
        tracing::trace!("Reading MneDiDatum.");
        let column = map_to_string("Column", m)?;
        tracing::trace!("Column: {column}.");
        let column_code = map_to_int("ColumnCode", m)?;
        tracing::trace!("Column Code: {column_code}.");
        let column_g_parent = map_to_string("ColumnGParent", m)?;
        tracing::trace!("Column G Parent: {column_g_parent}.");
        let column_g_parent_code = map_to_int("ColumnGParentCode", m)?;
        tracing::trace!("Column G Parent Code: {column_g_parent_code}.");
        let column_parent = map_to_string("ColumnParent", m)?;
        tracing::trace!("Column Parent: {column_parent}.");
        let column_parent_code = map_to_int("ColumnParentCode", m)?;
        tracing::trace!("Column Parent Code: {column_parent_code}.");
        let data_value = map_to_string("DataValue", m)?;
        tracing::trace!("Data Value: {data_value}.");
        let data_value_unformatted = map_to_string("DataValueUnformatted", m)?;
        let data_value_unformatted = AnnotatedInteger::from_value(&data_value_unformatted)?;
        tracing::trace!(
            "Data Value Unformatted: {}.",
            data_value_unformatted.as_value()
        );
        let row = map_to_string("Row", m)?;
        tracing::debug!("Row: {row}.");
        let row_code = RowCode::from_value(m, &row, naics)?;
        let series_id = map_to_int("SeriesID", m)?;
        let series_name = map_to_string("SeriesName", m)?;
        let table_column_display_order = map_to_float("TableColumnDisplayOrder", m)?;
        let table_row_display_order = map_to_float("TableRowDisplayOrder", m)?;
        let table_scale = map_to_string("TableScale", m)?;
        let year = map_to_string("Year", m)?;
        let year = parse_year(&year)?;
        Ok(Self {
            column,
            column_code,
            column_g_parent,
            column_g_parent_code,
            column_parent,
            column_parent_code,
            data_value,
            data_value_unformatted,
            row,
            row_code,
            series_id,
            series_name,
            table_column_display_order,
            table_row_display_order,
            table_scale,
            year,
        })
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
)]
#[from(Vec<MneDiDatum>)]
pub struct MneDiData(Vec<MneDiDatum>);

impl TryFrom<&std::path::PathBuf> for MneDiData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::trace!("Response read.");
        tracing::trace!("Response: {data:#?}");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::MneDi(value) => {
                    tracing::trace!("{} MneDi records read.", value.len());
                    Ok(value)
                }
                _ => {
                    let error =
                        DatasetMissing::new("MneDi".to_string(), line!(), file!().to_string());
                    Err(error.into())
                }
            }
        } else {
            tracing::warn!("Data variant missing.");
            let error = VariantMissing::new(
                "Data variant missing".to_string(),
                "Results".to_string(),
                line!(),
                file!().to_string(),
            );
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for MneDiData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading MneDiData");
        // use naics code to determine missing row codes from the row title
        let naics = NaicsItems::from_csv("../data/naics_codes.csv")?;
        match result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = MneDiDatum::read_json(m, &naics)?;
                            data.push(datum);
                        }
                        _ => {
                            let error = NotObject::new(line!(), file!().to_string());
                            return Err(error.into());
                        }
                    }
                }
                tracing::trace!("Data found: {} records.", data.len());
                Ok(Self(data))
            }
            // If there is a only a single observation, this will serialize to an object and not an
            // array, so this match catches singles before falling through to an array error.
            serde_json::Value::Object(m) => {
                let mut data = Vec::new();
                let datum = MneDiDatum::read_json(m, &naics)?;
                data.push(datum);
                tracing::trace!("Data found: {} records.", data.len());
                Ok(Self(data))
            }
            _ => {
                let error = NotArray::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}
