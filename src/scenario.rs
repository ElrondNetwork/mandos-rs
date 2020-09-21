use super::*;
use std::collections::BTreeMap;

pub struct Scenario {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub check_gas: Option<bool>,
    pub steps: Vec<Step>,
}

pub enum Step {
    ExternalSteps {
        path: String,
    },
    SetState {
        comment: Option<String>,
        accounts: BTreeMap<String, Account>,
        new_addresses: Vec<NewAddress>,
        block_hashes: Vec<BytesValue>,
        previous_block_info: Option<BlockInfo>,
        current_block_info: Option<BlockInfo>,
    },
    ScCall {
        tx_id: BytesValue,
        comment: Option<BytesValue>,
        tx: TxCall,
        expect: Option<TxExpect>,
    },
    ScDeploy {
        tx_id: BytesValue,
        comment: Option<BytesValue>,
        tx: TxDeploy,
        expect: Option<TxExpect>,
    },
    Transfer {
        tx_id: BytesValue,
        comment: Option<BytesValue>,
        tx: TxTransfer,
    },
    ValidatorReward {
        tx_id: BytesValue,
        comment: Option<BytesValue>,
        tx: TxValidatorReward,
    },
    CheckState {
        comment: Option<BytesValue>,
        accounts: CheckAccounts,
    },
    DumpState {
        comment: Option<BytesValue>,
    },
}

impl InterpretableFrom<StepRaw> for Step {
    fn interpret_from(from: StepRaw, context: &InterpreterContext) -> Self {
        match from {
            StepRaw::ExternalSteps {
                path,
            } => Step::ExternalSteps {
                path
            },
            StepRaw::SetState {
                comment,
                accounts,
                new_addresses,
                block_hashes,
                previous_block_info,
                current_block_info,
            } => Step::SetState {
                comment,
                accounts: accounts.into_iter().map(|(k, v)| (k.clone(), Account::interpret_from(v, context))).collect(),
                new_addresses: new_addresses.into_iter().map(|t| NewAddress::interpret_from(t, context)).collect(),
                block_hashes: block_hashes.into_iter().map(|t| BytesValue::interpret_from(t, context)).collect(),
                previous_block_info: previous_block_info.map(|v| BlockInfo::interpret_from(v, context)),
                current_block_info: current_block_info.map(|v| BlockInfo::interpret_from(v, context)),
            },
            StepRaw::ScCall {
                tx_id,
                comment,
                tx,
                expect,
            } => Step::ScCall {
                tx_id: BytesValue::interpret_from(tx_id, context),
                comment: comment.map(|v| BytesValue::interpret_from(v, context)),
                tx: TxCall::interpret_from(tx, context),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            },
            StepRaw::ScDeploy {
                tx_id,
                comment,
                tx,
                expect,
            } => Step::ScDeploy {
                tx_id: BytesValue::interpret_from(tx_id, context),
                comment: comment.map(|v| BytesValue::interpret_from(v, context)),
                tx: TxDeploy::interpret_from(tx, context),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            },
            StepRaw::Transfer {
                tx_id,
                comment,
                tx,
            } => Step::Transfer {
                tx_id: BytesValue::interpret_from(tx_id, context),
                comment: comment.map(|v| BytesValue::interpret_from(v, context)),
                tx: TxTransfer::interpret_from(tx, context),
            },
            StepRaw::ValidatorReward {
                tx_id,
                comment,
                tx,
            } => Step::ValidatorReward {
                tx_id: BytesValue::interpret_from(tx_id, context),
                comment: comment.map(|v| BytesValue::interpret_from(v, context)),
                tx: TxValidatorReward::interpret_from(tx, context),
            },
            StepRaw::CheckState {
                comment,
                accounts,
            } => Step::CheckState {
                comment: comment.map(|v| BytesValue::interpret_from(v, context)),
                accounts: CheckAccounts::interpret_from(accounts, context),
            },
            StepRaw::DumpState {
                comment,
            } => Step::DumpState {
                comment: comment.map(|v| BytesValue::interpret_from(v, context)),
            },
        }
    }
}

pub struct NewAddress {
    pub creator_address: BytesValue,
    pub creator_nonce: U64Value,
    pub new_address: BytesValue,
}

impl InterpretableFrom<NewAddressRaw> for NewAddress {
    fn interpret_from(from: NewAddressRaw, context: &InterpreterContext) -> Self {
        NewAddress {
            creator_address: BytesValue::interpret_from(from.creator_address, context),
            creator_nonce: U64Value::interpret_from(from.creator_nonce, context),
            new_address: BytesValue::interpret_from(from.new_address, context),
        }
    }
}

pub struct BlockInfo {
    pub block_timestamp: Option<U64Value>,
    pub block_nonce: Option<U64Value>,
    pub block_round: Option<U64Value>,
    pub block_epoch: Option<U64Value>,
}

impl InterpretableFrom<BlockInfoRaw> for BlockInfo {
    fn interpret_from(from: BlockInfoRaw, context: &InterpreterContext) -> Self {
        BlockInfo {
            block_timestamp: from.block_timestamp.map(|v| U64Value::interpret_from(v, context)),
            block_nonce: from.block_nonce.map(|v| U64Value::interpret_from(v, context)),
            block_round: from.block_round.map(|v| U64Value::interpret_from(v, context)),
            block_epoch: from.block_epoch.map(|v| U64Value::interpret_from(v, context)),
        }
    }
}

pub struct TxCall {
    pub from: BytesValue,
    pub to: BytesValue,
    pub value: BigUintValue,
    pub function: String,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxCallRaw> for TxCall {
    fn interpret_from(from: TxCallRaw, context: &InterpreterContext) -> Self {
        TxCall {
            from: BytesValue::interpret_from(from.from, context),
            to: BytesValue::interpret_from(from.to, context),
            value: BigUintValue::interpret_from(from.value, context),
            function: from.function,
            arguments: from.arguments.into_iter().map(|t| BytesValue::interpret_from(t, context)).collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price, context),
        }
    }
}

pub struct TxDeploy {
    pub from: BytesValue,
    pub value: BigUintValue,
    pub contract_code: BytesValue,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxDeployRaw> for TxDeploy {
    fn interpret_from(from: TxDeployRaw, context: &InterpreterContext) -> Self {
        TxDeploy {
            from: BytesValue::interpret_from(from.from, context),
            value: BigUintValue::interpret_from(from.value, context),
            contract_code: BytesValue::interpret_from(from.contract_code, context),
            arguments: from.arguments.into_iter().map(|t| BytesValue::interpret_from(t, context)).collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price, context),
        }
    }
}

pub struct TxTransfer {
    pub from: BytesValue,
    pub to: BytesValue,
    pub value: BigUintValue,
}

impl InterpretableFrom<TxTransferRaw> for TxTransfer {
    fn interpret_from(from: TxTransferRaw, context: &InterpreterContext) -> Self {
        TxTransfer {
            from: BytesValue::interpret_from(from.from, context),
            to: BytesValue::interpret_from(from.to, context),
            value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

pub struct TxValidatorReward {
    pub to: BytesValue,
    pub value: BigUintValue,
}

impl InterpretableFrom<TxValidatorRewardRaw> for TxValidatorReward {
    fn interpret_from(from: TxValidatorRewardRaw, context: &InterpreterContext) -> Self {
        TxValidatorReward {
            to: BytesValue::interpret_from(from.to, context),
            value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

pub struct TxExpect {
    pub out: Vec<BytesValue>,
    pub status: U64Value,
    pub logs: CheckLogs,
    pub message: Option<BytesValue>,
    pub gas: Option<CheckU64Value>,
    pub refund: Option<CheckU64Value>,

}

impl InterpretableFrom<TxExpectRaw> for TxExpect {
    fn interpret_from(from: TxExpectRaw, context: &InterpreterContext) -> Self {
        TxExpect {
            out: from.out.into_iter().map(|t| BytesValue::interpret_from(t, context)).collect(),
            status: U64Value::interpret_from(from.status, context),
            logs: CheckLogs::interpret_from(from.logs, context),
            message: from.message.map(|v| BytesValue::interpret_from(v, context)),
            gas: from.gas.map(|v| CheckU64Value::interpret_from(v, context)),
            refund: from.refund.map(|v| CheckU64Value::interpret_from(v, context)),
        }
    }
}
