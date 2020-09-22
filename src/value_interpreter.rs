use num_bigint::{BigInt, BigUint, Sign};
use num_traits::identities::Zero;

pub struct InterpreterContext{}

impl Default for InterpreterContext {
    fn default() -> Self {
        InterpreterContext{}
    }
}

const STR_PREFIXES: [&'static str; 3] = ["str:", "``", "''"];

const ADDR_PREFIX: &str = "address:";
// const filePrefix = "file:"
// const keccak256Prefix = "keccak256:"

// const u64Prefix = "u64:"
// const u32Prefix = "u32:"
// const u16Prefix = "u16:"
// const u8Prefix = "u8:"
// const i64Prefix = "i64:"
// const i32Prefix = "i32:"
// const i16Prefix = "i16:"
// const i8Prefix = "i8:"

pub fn interpret_string(s: &str, context: &InterpreterContext) -> Vec<u8> {
    if s.is_empty() {
        return Vec::new();
    }

    // concatenate values of different formats
    let split_parts: Vec<_> = s.split('|').collect();
    if split_parts.len() > 1 {
        let mut result = Vec::<u8>::new();
        for part in split_parts.iter() {
            result.extend_from_slice(interpret_string(part, context).as_slice());
        }
        return result;
    }

    if s == "true" {
        return [1u8].to_vec();
    }

    if s == "false" {
        return Vec::new()
    }

    for str_prefix in STR_PREFIXES.iter() {
        if s.starts_with(str_prefix) {
            return s[str_prefix.len() .. ].as_bytes().to_vec()
        }
    }
    
    if s.starts_with(ADDR_PREFIX) {
        return address(&s[ADDR_PREFIX.len() .. ]);
    }

    if s.starts_with("+") {
        let bi = BigInt::from_biguint(Sign::Plus, parse_unsigned(&s[1..]));
        return big_int_to_bytes_be(&bi);
    }

    if s.starts_with("-") {
        let bi = BigInt::from_biguint(Sign::Minus, parse_unsigned(&s[1..]));
        return big_int_to_bytes_be(&bi);
    }

    big_uint_to_bytes_be(&parse_unsigned(s))
}

fn parse_unsigned(s: &str) -> BigUint {
    let clean = s.replace(&['_', ','][..], "");
    if clean.starts_with("0x") || clean.starts_with("0X") {
        let clean = &clean[2..];
        if clean.is_empty() {
            return BigUint::zero();
        }
        return BigUint::parse_bytes(clean.as_bytes(), 16).unwrap();
    }

    if clean.starts_with("0b") || clean.starts_with("0B") {
        let clean = &clean[2..];
        if clean.is_empty() {
            return BigUint::zero();
        }
        return BigUint::parse_bytes(clean.as_bytes(), 2).unwrap();
    }

    BigUint::parse_bytes(clean.as_bytes(), 10).unwrap()
}

fn big_uint_to_bytes_be(bu: &BigUint) -> Vec<u8> {
    if bu.is_zero() {
        Vec::new()
    } else {
        bu.to_bytes_be()
    }
}

fn big_int_to_bytes_be(bi: &BigInt) -> Vec<u8> {
    if bi.is_zero() {
        Vec::new()
    } else {
        bi.to_signed_bytes_be()
    }
}

fn address(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    if bytes.len() > 32 {
        return bytes[.. 32].to_vec();
    }
    let mut result = vec![b'_'; 32];
    result[.. bytes.len()].copy_from_slice(bytes);
    result
}
