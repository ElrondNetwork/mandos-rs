use super::value_interpreter::*;
use super::value_raw::*;
use super::context::*;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, context: &InterpreterContext) -> Self;
}

#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub struct BigUintValue {
    pub value: BigUint,
    pub original: ValueSubTree,
}

impl InterpretableFrom<ValueSubTree> for BigUintValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        let bytes = interpret_subtree(&from, context);
        BigUintValue {
            value: BigUint::from_bytes_be(&bytes),
            original: from,
        }
    }
}

#[derive(Debug)]
pub struct U64Value {
    pub value: u64,
    pub original: ValueSubTree,
}

impl InterpretableFrom<ValueSubTree> for U64Value {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        let bytes = interpret_subtree(&from, context);
        let bu = BigUint::from_bytes_be(&bytes);
        U64Value {
            value: bu.to_u64().unwrap(),
            original: from,
        }
    }
}
