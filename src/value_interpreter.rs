use num_bigint::BigUint;
use std::str::FromStr;


const STR_PREFIXES: [&'static str; 3] = ["str:", "``", "''"];
// var strPrefixes = []string{"str:", "``", "''"}

// const ADDR_PREFIX: &str = "address";
// const addrPrefix = "address:"
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

pub fn interpret_string(s: &str) -> Vec<u8> {
    if s.is_empty() {
        return Vec::new();
    }

    if let Ok(big_uint) = BigUint::from_str(s) {
        return big_uint.to_bytes_be()
    }

    // concatenate values of different formats
    let split_parts: Vec<_> = s.split('|').collect();
    if split_parts.len() > 1 {
        let mut result = Vec::<u8>::new();
        for part in split_parts.iter() {
            result.extend_from_slice(interpret_string(part).as_slice());
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
            return s.as_bytes().to_vec()
        }
    }
    
    

    // TEMP
    s.as_bytes().to_vec()
}

