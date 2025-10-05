use crate::{Mode, ResultStatus};
use bears_species::{BeaErr, DeriveFromStr, Jiff, KeyMissing, NotObject, ParseInt, map_to_string};
use jiff::ToSpan;
use std::str::FromStr;

// Cannot exceed 30 errors per minute.
// Theory: calls may get ahead of tracker
// 14 ahead of 14 = 28
// hitting API rate limit at 14, try 10
// probably hitting the data upload limit of 100MB per minute
pub const ERROR_CAP: usize = 7;
// Cannot exceed 100 calls per minute.
// 14 ahead of 85 = 99
pub const CALL_CAP: usize = 30;

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
)]
// Tighten up the api for tracker so everything goes through methods, and no raw field access need
// occur, such as tracker.calls.push(event).
pub struct Tracker {
    pub calls: Vec<Event>,
    pub errors: Vec<Event>,
    pub cache: Vec<Event>,
    // Total size field to track cumulative dowload size over the last minute.
    pub size: Vec<SizeEvent>,
}

impl Tracker {
    #[tracing::instrument(skip_all)]
    pub fn update_status(&mut self, status: ResultStatus, mode: Mode) {
        match status {
            ResultStatus::Success(id, length) => {
                if let Some(event) = self.calls.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    event.length = Some(length);
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                } else if let Some(event) = self.cache.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    event.length = Some(length);
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                }
            }
            ResultStatus::Error(id) => {
                if let Some(event) = self.calls.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                } else if let Some(event) = self.cache.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                }
                if let Some(event) = self.calls.iter().find(|event| event.id == id) {
                    self.errors.push(event.clone());
                }
            }
            ResultStatus::Pending | ResultStatus::Abort | ResultStatus::Pass(_) => {}
        }
    }

    /// Remaining download capacity within the 100MB per minute rate limit set by the BEA server.
    #[tracing::instrument(skip_all)]
    pub fn size_available(&self) -> u64 {
        // Download rate limit of 100MB.
        100_000_000_u64.saturating_sub(self.total_size())
    }

    #[tracing::instrument(skip_all)]
    pub fn check_slack(&mut self) -> usize {
        tracing::trace!("Checking slack.");
        self.update_count();
        let pending = self
            .calls
            .iter()
            .filter(|c| c.status == ResultStatus::Pending)
            .collect::<Vec<&Event>>()
            .len();
        let pending_slack = ERROR_CAP.saturating_sub(pending);
        let error_slack = ERROR_CAP.saturating_sub(self.errors.len());
        let call_slack = CALL_CAP.saturating_sub(self.calls.len());
        let slack = error_slack.min(call_slack);
        let slack = slack.min(pending_slack);
        tracing::trace!("Pending slack {pending_slack}.");
        tracing::trace!("Error slack {error_slack}.");
        tracing::trace!("Call slack {call_slack}.");
        tracing::trace!("Slack is {slack}.");
        slack
    }

    /// Returns the cumulative size of events in the `size` field.  Used to determine if the next
    /// call will exceed the data download rate limit for the BEA server.
    #[tracing::instrument(skip_all)]
    pub fn total_size(&self) -> u64 {
        self.size.iter().map(|value| value.size).sum::<u64>()
    }

    #[tracing::instrument(skip_all)]
    pub fn update_count(&mut self) {
        tracing::trace!("Updating count.");
        let now = jiff::Timestamp::now();
        // Calls made over the last minute.
        if !self.calls.is_empty() {
            let mut old = self.calls.clone();
            old.retain(|call| {
                (now - call.time).total(jiff::Unit::Minute).unwrap()
                    >= 1.minutes().total(jiff::Unit::Minute).unwrap()
            });
            // Move older calls to the `cache` field so they can still receive status updates from
            // the listener.
            self.cache.extend(old);
            self.calls.retain(|call| {
                (now - call.time).total(jiff::Unit::Minute).unwrap()
                    < 1.minutes().total(jiff::Unit::Minute).unwrap()
            });
        }
        // Errors received over the last minute.
        if !self.errors.is_empty() {
            self.errors.retain(|error| {
                (now - error.time).total(jiff::Unit::Minute).unwrap()
                    < 1.minutes().total(jiff::Unit::Minute).unwrap()
            });
        }
        // Cumulative size requested over the last minute
        if !self.size.is_empty() {
            self.size.retain(|size| {
                (now - size.time).total(jiff::Unit::Minute).unwrap()
                    < 1.minutes().total(jiff::Unit::Minute).unwrap()
            });
        }
        tracing::trace!(
            "Calls: {}, Errors: {}, Sizes: {}.",
            self.calls.len(),
            self.errors.len(),
            self.size.len()
        );
    }

    #[tracing::instrument(skip_all)]
    pub async fn wait(&self) {
        tracing::trace!("Calling wait.");
        let minute = 1.minute().total(jiff::Unit::Millisecond).unwrap();
        let mut pause = minute.ceil() as u64;
        tracing::trace!("Maximum pause is {pause} millis.");
        let now = jiff::Timestamp::now();
        if !self.calls.is_empty() {
            let oldest = self
                .calls
                .iter()
                .map(|c| {
                    let cap = now - 1.minute();
                    (c.time - cap).total(jiff::Unit::Millisecond).unwrap()
                })
                .fold(minute, f64::min);
            let oldest = oldest.ceil() as u64;
            tracing::trace!("Oldest call is {oldest} millis away from expiring.");
            pause = pause.min(oldest);
        }
        if !self.errors.is_empty() {
            let oldest = self
                .errors
                .iter()
                .map(|c| {
                    let cap = now - 1.minute();
                    (c.time - cap).total(jiff::Unit::Millisecond).unwrap()
                })
                .fold(minute, f64::min);
            let oldest = oldest.ceil() as u64;
            tracing::trace!("Oldest call is {oldest} millis away from expiring.");
            pause = pause.min(oldest);
        }
        if pause > 5000 {
            tracing::trace!("Pausing for 5000 millis.");
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        } else {
            tracing::trace!("Pausing for {pause} millis.");
            tokio::time::sleep(tokio::time::Duration::from_millis(pause)).await;
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
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
)]
pub struct Event {
    id: uuid::Uuid,
    length: Option<u64>,
    mode: Mode,
    path: std::path::PathBuf,
    status: ResultStatus,
    time: jiff::Timestamp,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, length: {}, mode: {}, path: {:?}, status: {}, time: {}",
            self.id,
            self.len_as_str(),
            self.mode,
            self.path,
            self.status,
            self.time
        )
    }
}

impl Event {
    pub fn new<P: AsRef<std::path::Path>>(path: P, mode: Mode) -> Self {
        let id = uuid::Uuid::new_v4();
        let path = std::path::PathBuf::from(path.as_ref());
        let status = ResultStatus::Pending;
        let time = jiff::Timestamp::now();
        Self {
            id,
            length: None,
            mode,
            path,
            status,
            time,
        }
    }

    pub fn len_as_str(&self) -> String {
        match self.length {
            Some(num) => num.to_string(),
            None => "None".to_string(),
        }
    }

    pub fn len_from_str(len: &str) -> Result<Option<u64>, BeaErr> {
        match len {
            "None" => Ok(None),
            number => match number.parse::<u64>() {
                Ok(num) => Ok(Some(num)),
                Err(source) => {
                    let error =
                        ParseInt::new(number.to_string(), source, line!(), file!().to_string());
                    Err(error.into())
                }
            },
        }
    }

    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let msg = map_to_string("message", m)?;
        // skip the key in key: value pair sequence
        // skip the first field "id"
        let result = msg
            .split(",")
            .flat_map(|pair| pair.split(":").skip(1))
            .skip(1)
            .collect::<Vec<&str>>();
        tracing::trace!("Raw Event: {result:#?}");
        let id = uuid::Uuid::new_v4();
        if result.len() < 7 {
            let error = KeyMissing::new("invalid Event".to_string(), line!(), file!().to_string());
            Err(error.into())
        } else {
            let length = Self::len_from_str(result[0].trim())?;
            let mode = result[1].trim().to_string();
            let mode = Mode::from_str(&mode)
                .map_err(|e| DeriveFromStr::new(mode, e, line!(), file!().to_string()))?;
            let path = result[2].trim().trim_matches('"');
            tracing::trace!("Path is {path}");
            let path = std::path::PathBuf::from(path);
            let status = result[3].trim().to_string();
            let status = ResultStatus::from_str(&status)?;
            let time = result[4].trim().to_string();
            let time = format!("{time}:{}:{}", result[5], result[6]);
            let time = match jiff::Timestamp::from_str(&time) {
                Ok(stamp) => stamp,
                Err(source) => {
                    let error = Jiff::new(time, source);
                    return Err(error.into());
                }
            };
            Ok(Self {
                id,
                length,
                mode,
                path,
                status,
                time,
            })
        }
    }
}

impl TryFrom<&serde_json::Value> for Event {
    type Error = BeaErr;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading Event.");
        match value {
            serde_json::Value::Object(m) => {
                let key = "fields".to_string();
                if let Some(fields) = m.get(&key) {
                    tracing::trace!("Fields present: {fields:#?}");
                    match fields {
                        serde_json::Value::Object(m) => Self::read_json(m),
                        _ => {
                            tracing::trace!("Invalid Value: {value:#?}");
                            let error = NotObject::new(line!(), file!().to_string());
                            Err(error.into())
                        }
                    }
                } else {
                    let error = KeyMissing::new(key, line!(), file!().to_string());
                    Err(error.into())
                }
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
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_")]
pub struct SizeEvent {
    size: u64,
    time: jiff::Timestamp,
}

impl SizeEvent {
    pub fn new(size: u64) -> Self {
        Self {
            size,
            time: jiff::Timestamp::now(),
        }
    }
}

impl From<u64> for SizeEvent {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
