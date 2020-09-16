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
    ExternalSteps {
        path: String,
    },
    
    #[serde(rename_all = "camelCase")]
    SetState {
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
        block_hashes: Vec<BytesValue>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_block_info: Option<BlockInfo>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        current_block_info: Option<BlockInfo>,
    },

    #[serde(rename_all = "camelCase")]
    ScCall {
        tx_id: BytesValue,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<BytesValue>,

        tx: TxCall,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpect>,
    },

    #[serde(rename_all = "camelCase")]
    ScDeploy {
        tx_id: BytesValue,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<BytesValue>,

        tx: TxDeploy,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpect>,
    },

    #[serde(rename_all = "camelCase")]
    Transfer {
        tx_id: BytesValue,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<BytesValue>,

        tx: TxTransfer,
    },

    #[serde(rename_all = "camelCase")]
    ValidatorReward {
        tx_id: BytesValue,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<BytesValue>,

        tx: TxValidatorReward,
    },

    CheckState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        accounts: CheckAccounts,
    },

    DumpState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<BytesValue>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAddress {
    pub creator_address: BytesValue,
    pub creator_nonce: U64Value,
    pub new_address: BytesValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<U64Value>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_nonce: Option<U64Value>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_round: Option<U64Value>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_epoch: Option<U64Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCall {
    pub from: BytesValue,
    pub to: BytesValue,
    pub value: BigUintValue,
    pub function: String,

    #[serde(default)]
    pub arguments: Vec<BytesValue>,

    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxDeploy {
    pub from: BytesValue,
    pub value: BigUintValue,

    pub contract_code: BytesValue,

    #[serde(default)]
    pub arguments: Vec<BytesValue>,

    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxTransfer {
    pub from: BytesValue,
    pub to: BytesValue,
    pub value: BigUintValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxValidatorReward {
    pub to: BytesValue,
    pub value: BigUintValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxExpect {

    #[serde(default)]
    pub out: Vec<BytesValue>,

    pub status: U64Value,

    pub logs: CheckLogs,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<BytesValue>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<CheckU64Value>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<CheckU64Value>,

}


