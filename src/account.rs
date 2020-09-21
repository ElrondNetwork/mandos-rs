use super::*;
use std::collections::BTreeMap;

pub struct Account {
    pub comment: Option<BytesValue>,
    pub nonce: U64Value,
    pub balance: BigUintValue,
    pub storage: BTreeMap<String, BytesValue>,
    pub code: Option<BytesValue>,
}

impl InterpretableFrom<AccountRaw> for Account {
    fn interpret_from(from: AccountRaw, context: &InterpreterContext) -> Self {
        Account {
            comment: from.comment.map(|c| BytesValue::interpret_from(c, context)),
            nonce: U64Value::interpret_from(from.nonce, context),
            balance: BigUintValue::interpret_from(from.balance, context),
            storage: from.storage.into_iter().map(|(k, v)| (k.clone(), BytesValue::interpret_from(v, context))).collect(),
            code: from.code.map(|c| BytesValue::interpret_from(c, context)),
        }
    }
}

pub enum CheckStorage {
    Star,
    Equal(BTreeMap<String, CheckBytesValue>)
}

impl InterpretableFrom<CheckStorageRaw> for CheckStorage {
    fn interpret_from(from: CheckStorageRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckStorageRaw::Star => CheckStorage::Star,
            CheckStorageRaw::Equal(m) => CheckStorage::Equal(
                m.into_iter().map(|(k, v)| (k.clone(), CheckBytesValue::interpret_from(v, context))).collect(),
            )
        }
    }
}

impl CheckStorage {
    pub fn is_star(&self) -> bool {
        if let CheckStorage::Star = self { true } else { false }
    }
}
pub struct CheckAccount {
    pub comment: Option<BytesValue>,
    pub nonce: CheckU64Value,
    pub balance: CheckBigUintValue,
    pub storage: CheckStorage,
    pub code: Option<CheckBytesValue>,
    pub async_call_data: CheckBytesValue,
}

impl InterpretableFrom<CheckAccountRaw> for CheckAccount {
    fn interpret_from(from: CheckAccountRaw, context: &InterpreterContext) -> Self {
        CheckAccount {
            comment: from.comment.map(|c| BytesValue::interpret_from(c, context)),
            nonce: CheckU64Value::interpret_from(from.nonce, context),
            balance: CheckBigUintValue::interpret_from(from.balance, context),
            storage: CheckStorage::interpret_from(from.storage, context),
            code: from.code.map(|c| CheckBytesValue::interpret_from(c, context)),
            async_call_data: CheckBytesValue::interpret_from(from.async_call_data, context),
        }
    }
}

pub struct CheckAccounts {
    pub other_accounts_allowed: bool,
    pub accounts: BTreeMap<String, CheckAccount>
}

impl InterpretableFrom<CheckAccountsRaw> for CheckAccounts {
    fn interpret_from(from: CheckAccountsRaw, context: &InterpreterContext) -> Self {
        CheckAccounts {
            other_accounts_allowed: from.other_accounts_allowed,
            accounts: from.accounts.into_iter().map(|(k, v)| (k.clone(), CheckAccount::interpret_from(v, context))).collect(),
        }
    }
}
