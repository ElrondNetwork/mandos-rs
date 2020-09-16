use super::*;
use std::fmt;

use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeSeq};
use serde::de::{self, Deserializer, Visitor, SeqAccess};

#[derive(Serialize, Deserialize)]
pub struct CheckLog {
    pub address: BytesValue,
    pub identifier: BytesValue,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<BytesValue>,

    pub data: BytesValue,
}

pub enum CheckLogs {
    Star,
    List(Vec<CheckLog>),
}

impl CheckLogs {
    pub fn is_star(&self) -> bool {
        if let CheckLogs::Star = self { true } else { false }
    }
}

impl Serialize for CheckLogs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckLogs::Star => serializer.serialize_str("*"),
            CheckLogs::List(l) => {
                let mut seq = serializer.serialize_seq(Some(l.len()))?;
                for item in l {
                    seq.serialize_element(item)?;
                }
                seq.end()
            },
        }
    }
}

struct CheckLogsVisitor;

impl<'de> Visitor<'de> for CheckLogsVisitor {
    type Value = CheckLogs;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckLogs::Star)    
        } else {
            Err(de::Error::custom("only '*' allowed as logs string value"))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::<CheckLog>::new();

        while let Some(item) = seq.next_element()? {
            list.push(item);
        }

        Ok(CheckLogs::List(list)) 
    }
}


impl<'de> Deserialize<'de> for CheckLogs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckLogsVisitor)
    }
}
