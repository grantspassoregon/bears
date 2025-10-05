use crate::{BeaErr, KeyMissing, NotArray, NotObject, ParameterName, map_to_string};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_getters::Getters,
)]
pub struct Note {
    reference: Option<String>,
    text: String,
}

impl Note {
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let key = ParameterName::NoteRef.to_string();
        let reference = map_to_string(&key, m).ok();
        tracing::trace!("reference is {reference:?}.");
        let key = ParameterName::NoteText.to_string();
        let text = map_to_string(&key, m)?;
        tracing::trace!("text is {text}.");
        Ok(Self { reference, text })
    }
}

impl TryFrom<serde_json::Value> for Note {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading Note.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
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
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_more::From,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[from(Vec<Note>)]
pub struct Notes(Vec<Note>);

impl Notes {
    #[tracing::instrument(skip_all)]
    pub fn set(&self) -> std::collections::BTreeSet<&Note> {
        self.iter().collect()
    }

    #[tracing::instrument(skip_all)]
    pub fn try_from_results(result: &serde_json::Value) -> Result<&serde_json::Value, BeaErr> {
        tracing::trace!("Reading notes from results.");
        let key = ParameterName::Notes.to_string();
        match result {
            serde_json::Value::Object(m) => {
                if let Some(data) = m.get(&key) {
                    Ok(data)
                } else {
                    tracing::trace!("{key} missing.");
                    let error = KeyMissing::new(key, line!(), file!().to_string());
                    Err(error.into())
                }
            }
            serde_json::Value::Array(v) => {
                tracing::trace!("Array detected: {} records.", v.len());
                if let Some(item) = v.first() {
                    match item {
                        serde_json::Value::Object(m) => {
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
                    let error = KeyMissing::new(
                        "empty array for {key}".to_owned(),
                        line!(),
                        file!().to_owned(),
                    );
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
}

impl TryFrom<&serde_json::Value> for Notes {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading Notes");
        match Notes::try_from_results(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = Note::read_json(m)?;
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
            _ => {
                let error = NotArray::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}
