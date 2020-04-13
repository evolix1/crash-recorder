use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HowItWasStopped {
    SelfCrashed,
    ManuallyKilled,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    #[serde(
        serialize_with="opt_dt_serde::serialize",
        deserialize_with="opt_dt_serde::deserialize")]
    pub frozen: Option<DateTime<Utc>>,
    #[serde(
        serialize_with="opt_dt_serde::serialize",
        deserialize_with="opt_dt_serde::deserialize")]
    pub busy: Option<DateTime<Utc>>,
    pub description: String,
    pub how: HowItWasStopped,
    #[serde(
        serialize_with="dt_serde::serialize",
        deserialize_with="dt_serde::deserialize")]
    pub when: DateTime<Utc>,
}


impl Default for Record {
    fn default() -> Self {
        Self {
            frozen: None,
            busy: None,
            description: String::new(),
            how: HowItWasStopped::SelfCrashed,
            when: Utc::now(),
        }
    }
}


// modified from [https://earvinkayonga.com/posts/deserialize-date-in-rust/]
pub mod dt_serde {
    use chrono::{DateTime, Utc};
    use serde::*;

    pub fn serialize<S: Serializer>(dt: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error> {
        dt.to_rfc3339()
            .serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
        let time: String = Deserialize::deserialize(d)?;
        DateTime::parse_from_rfc3339(&time)
            .map_err(serde::de::Error::custom)
            .map(|fixed_dt| fixed_dt.into())
    }
}

pub mod opt_dt_serde {
    use chrono::{DateTime, Utc};
    use serde::*;

    pub fn serialize<S: Serializer>(dt: &Option<DateTime<Utc>>, s: S) -> Result<S::Ok, S::Error> {
        dt.as_ref()
            .map(|dt| dt.to_rfc3339())
            .serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<DateTime<Utc>>, D::Error> {
        let time: String = match Deserialize::deserialize(d) {
            Ok(v) => v,
            // erase error from reading, bc it can happen with `null` value
            Err(_) => return Ok(None),
        };
        DateTime::parse_from_rfc3339(&time)
            .map_err(serde::de::Error::custom)
            .map(|fixed_dt| Some(fixed_dt.into()))
    }
}
