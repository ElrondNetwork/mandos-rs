
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
pub struct Value {
    pub value: Vec<u8>,
    pub original: ValueSubTree,
}

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

impl Serialize for Value {
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

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let original = ValueSubTree::deserialize(deserializer)?;
        Ok(Value{
            value: interpret_subtree(&original),
            original
        })
    }
}
