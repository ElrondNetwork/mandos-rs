use num_bigint::BigUint;
use std::str::FromStr;

pub fn interpret_string(s: &String) -> Vec<u8> {
    if s.is_empty() {
        return Vec::new();
    }

    if let Ok(big_uint) = BigUint::from_str(s) {
        return big_uint.to_bytes_be()
    }
    
    // TEMP
    s.as_bytes().to_vec()
}

