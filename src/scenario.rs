use super::*;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_gas: Option<bool>,
    pub steps: Vec<Step>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "step")]
pub enum Step {
    ExternalSteps{
        path: String,
    },
    
    #[serde(rename_all = "camelCase")]
    SetState{
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        #[serde(default)]
        #[serde(skip_serializing_if = "BTreeMap::is_empty")]
        accounts: BTreeMap<String, Account>,
        
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        new_addresses: Vec<NewAddress>,
        
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        block_hashes: Vec<Value>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_block_info: Option<BlockInfo>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        current_block_info: Option<BlockInfo>,
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    pub nonce: String,
    pub balance: String,
    pub storage: BTreeMap<String, Value>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAddress {
    pub creator_address: String,
    pub creator_nonce: String,
    pub new_address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<String>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_nonce: Option<String>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_round: Option<String>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_epoch: Option<String>,
}

