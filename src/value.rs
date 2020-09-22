use super::value_interpreter::*;
use super::value_raw::*;

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, context: &InterpreterContext) -> Self;
}

pub fn interpret_subtree(vst: &ValueSubTree, context: &InterpreterContext) -> Vec<u8> {
    match vst {
        ValueSubTree::Str(s) => interpret_string(s, context),
        ValueSubTree::List(l) => {
            let mut concat = Vec::<u8>::new();
            for item in l.iter() {
                concat.extend_from_slice(interpret_subtree(item, context).as_slice());
            }
            concat
        },
        ValueSubTree::Map(m) => {
            let mut concat = Vec::<u8>::new();
            for (_, value) in m.iter() {
                concat.extend_from_slice(interpret_subtree(value, context).as_slice());
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


impl InterpretableFrom<ValueSubTree> for BytesValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        BytesValue {
            value: interpret_subtree(&from, context),
            original: from,
        }
    }
}

// TODO: these will not remain aliases
pub type BigUintValue = BytesValue;
pub type U64Value = BytesValue;


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

impl InterpretableFrom<ValueSubTree> for CheckBytesValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        match &from {
            ValueSubTree::Str(s) => {
                if s == "" {
                    CheckBytesValue::DefaultStar
                } else if s == "*" {
                    CheckBytesValue::Star
                } else {
                    CheckBytesValue::Equal(BytesValue::interpret_from(from, context))
                }
            },
            ValueSubTree::List(_) => CheckBytesValue::Equal(BytesValue::interpret_from(from, context)),
            ValueSubTree::Map(_) => CheckBytesValue::Equal(BytesValue::interpret_from(from, context)),
        }
    }
}


pub type CheckBigUintValue = CheckBytesValue;
pub type CheckU64Value = CheckBytesValue;

