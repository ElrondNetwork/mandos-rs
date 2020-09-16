
use super::interpret_string;
use std::collections::BTreeMap;
use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

pub fn interpret_subtree(vst: &ValueSubTree) -> Vec<u8> {
    match vst {
        ValueSubTree::Str(s) => interpret_string(s),
        ValueSubTree::List(l) => {
            let mut concat = Vec::<u8>::new();
            for item in l.iter() {
                concat.extend_from_slice(interpret_subtree(item).as_slice());
            }
            concat
        },
        ValueSubTree::Map(m) => {
            let mut concat = Vec::<u8>::new();
            for (_, value) in m.iter() {
                concat.extend_from_slice(interpret_subtree(value).as_slice());
            }
            concat
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BytesValue {
    pub value: Vec<u8>,
    pub original: ValueSubTree,
}

// TODO: these will not remain aliases
pub type BigUintValue = BytesValue;
pub type U64Value = BytesValue;

#[derive(PartialEq, Clone, Debug)]
pub enum ValueSubTree {
    Str(String),
    List(Vec<ValueSubTree>),
    Map(BTreeMap<String, ValueSubTree>),
}

impl Serialize for ValueSubTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValueSubTree::Str(s) => serializer.serialize_str(s.as_str()),
            ValueSubTree::List(l) => {
                let mut seq = serializer.serialize_seq(Some(l.len()))?;
                for item in l {
                    seq.serialize_element(item)?;
                }
                seq.end()
            },
            ValueSubTree::Map(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

impl Serialize for BytesValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.original.serialize(serializer)
    }
}

struct ValueSubTreeVisitor;

impl<'de> Visitor<'de> for ValueSubTreeVisitor {
    type Value = ValueSubTree;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ValueSubTree::Str(String::from(value)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::<ValueSubTree>::new();

        while let Some(item) = seq.next_element()? {
            list.push(item);
        }

        Ok(ValueSubTree::List(list)) 
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = BTreeMap::<String, ValueSubTree>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(ValueSubTree::Map(map))
    }
}

impl<'de> Deserialize<'de> for ValueSubTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueSubTreeVisitor)
    }
}

impl<'de> Deserialize<'de> for BytesValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let original = ValueSubTree::deserialize(deserializer)?;
        Ok(BytesValue{
            value: interpret_subtree(&original),
            original
        })
    }
}

pub enum CheckBytesValue {
    DefaultStar,
    Star,
    Equal(BytesValue),
}

impl CheckBytesValue {
    pub fn is_star(&self) -> bool {
        if let CheckBytesValue::Star | CheckBytesValue::DefaultStar = self { true } else { false }
    }

    pub fn is_default_star(&self) -> bool {
        if let CheckBytesValue::DefaultStar = self { true } else { false }
    }
}

impl Default for CheckBytesValue {
    fn default() -> Self {
        CheckBytesValue::DefaultStar
    }
}

impl Serialize for CheckBytesValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckBytesValue::Star | CheckBytesValue::DefaultStar => serializer.serialize_str("*"),
            CheckBytesValue::Equal(bytes_value) => bytes_value.serialize(serializer),
        }
    }
}

struct CheckBytesValueVisitor;

impl<'de> Visitor<'de> for CheckBytesValueVisitor {
    type Value = CheckBytesValue;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized CheckBytesValue")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckBytesValue::Star)
        } else {
            let original = ValueSubTreeVisitor.visit_str(value)?;
            Ok(CheckBytesValue::Equal(BytesValue{
                value: interpret_subtree(&original),
                original
            }))
        }
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let original = ValueSubTreeVisitor.visit_seq(seq)?;
        Ok(CheckBytesValue::Equal(BytesValue{
            value: interpret_subtree(&original),
            original
        }))
    }

    fn visit_map<M>(self, access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let original = ValueSubTreeVisitor.visit_map(access)?;
        Ok(CheckBytesValue::Equal(BytesValue{
            value: interpret_subtree(&original),
            original
        }))
    }
}

impl<'de> Deserialize<'de> for CheckBytesValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckBytesValueVisitor)
    }
}

pub type CheckBigUintValue = CheckBytesValue;
pub type CheckU64Value = CheckBytesValue;

