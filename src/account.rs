use super::*;
use std::fmt;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeMap};
use serde::de::{self, Deserializer, Visitor, MapAccess};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<BytesValue>,

    pub nonce: U64Value,
    pub balance: BigUintValue,
    pub storage: BTreeMap<String, BytesValue>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<BytesValue>,
}

pub enum CheckStorage {
    Star,
    Equal(BTreeMap<String, CheckBytesValue>)
}

impl CheckStorage {
    pub fn is_star(&self) -> bool {
        if let CheckStorage::Star = self { true } else { false }
    }
}

impl Serialize for CheckStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckStorage::Star => serializer.serialize_str("*"),
            CheckStorage::Equal(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
        }
    }
}

struct CheckStorageVisitor;

impl<'de> Visitor<'de> for CheckStorageVisitor {
    type Value = CheckStorage;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckStorage::Star)    
        } else {
            Err(de::Error::custom("only '*' allowed as logs string value"))
        }
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = BTreeMap::<String, CheckBytesValue>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(CheckStorage::Equal(map))
    }
}


impl<'de> Deserialize<'de> for CheckStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckStorageVisitor)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAccount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<BytesValue>,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckU64Value::is_default_star")]
    pub nonce: CheckU64Value,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBigUintValue::is_default_star")]
    pub balance: CheckBigUintValue,

    pub storage: CheckStorage,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CheckBytesValue>,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValue::is_default_star")]
    pub async_call_data: CheckBytesValue,
}

pub enum CheckAccountOrNothing {
    Some(CheckAccount),
    Nothing
}

struct CheckAccountOrNothingVisitor;

impl<'de> Visitor<'de> for CheckAccountOrNothingVisitor {
    type Value = CheckAccountOrNothing;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("CheckAccount or nothing")
    }

    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CheckAccountOrNothing::Nothing)
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckAccountOrNothing::Some(Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?))
    }
}

impl<'de> Deserialize<'de> for CheckAccountOrNothing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckAccountOrNothingVisitor)
    }
}

pub struct CheckAccounts {
    pub other_accounts_allowed: bool,
    pub accounts: BTreeMap<String, CheckAccount>
}

impl Serialize for CheckAccounts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.accounts.len()))?;
        for (k, v) in self.accounts.iter() {
            map.serialize_entry(k, v)?;
        }
        if self.other_accounts_allowed {
            map.serialize_entry("+", "")?;
        }
        map.end()
    }
}

struct CheckAccountsVisitor;

impl<'de> Visitor<'de> for CheckAccountsVisitor {
    type Value = CheckAccounts;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized CheckAccounts")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut accounts = BTreeMap::<String, CheckAccount>::new();
        let mut other_accounts_allowed = false;

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            if key == "+" {
                other_accounts_allowed = true;
            } else {
                if let CheckAccountOrNothing::Some(check_account) = value {
                    accounts.insert(key, check_account);
                } else {
                    return Err(de::Error::custom("invalid CheckAccount"))
                }
            }
        }

        Ok(CheckAccounts {
            accounts,
            other_accounts_allowed
        })
    }
}

impl<'de> Deserialize<'de> for CheckAccounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckAccountsVisitor)
    }
}
