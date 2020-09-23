use super::value_interpreter::*;
use super::value_raw::*;
use super::context::*;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

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

#[derive(Debug)]
pub enum CheckValue<T: InterpretableFrom<ValueSubTree>> {
    DefaultStar,
    Star,
    Equal(T),
}

impl<T: InterpretableFrom<ValueSubTree>> CheckValue<T> {
    pub fn is_star(&self) -> bool {
        if let CheckValue::Star | CheckValue::DefaultStar = self { true } else { false }
    }

    pub fn is_default_star(&self) -> bool {
        if let CheckValue::DefaultStar = self { true } else { false }
    }
}

impl<T: InterpretableFrom<ValueSubTree>> Default for CheckValue<T> {
    fn default() -> Self {
        CheckValue::DefaultStar
    }
}

impl<T: InterpretableFrom<ValueSubTree>> InterpretableFrom<ValueSubTree> for CheckValue<T> {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        if let ValueSubTree::Str(s) = &from {
            if s == "" {
                return CheckValue::DefaultStar;
            } else if s == "*" {
                return CheckValue::Star;
            }
        }

        CheckValue::Equal(T::interpret_from(from, context))
    }
}
