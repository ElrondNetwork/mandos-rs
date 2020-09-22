
extern crate mandos_rs;
use mandos_rs::*;

const EMPTY: Vec<u8> = Vec::<u8>::new();

#[test]
fn test_bool() {
    let context = &InterpreterContext::default();
    assert_eq!(vec![1], interpret_string("true", context));
    assert_eq!(EMPTY, interpret_string("false", context));
}

#[test]
fn test_string() {
    let context = &InterpreterContext::default();

    assert_eq!(b"abcdefg".to_vec(), interpret_string("``abcdefg", context));
    assert_eq!(EMPTY, interpret_string("``", context));
    assert_eq!(b"`".to_vec(), interpret_string("```", context));
    assert_eq!(b" ".to_vec(), interpret_string("`` ", context));

    assert_eq!(b"abcdefg".to_vec(), interpret_string("''abcdefg", context));
    assert_eq!(EMPTY, interpret_string("''", context));
    assert_eq!(b"'".to_vec(), interpret_string("'''", context));
    assert_eq!(b"``".to_vec(), interpret_string("''``", context));
    
    assert_eq!(b"abcdefg".to_vec(), interpret_string("str:abcdefg", context));
    assert_eq!(EMPTY, interpret_string("str:", context));
}

#[test]
fn test_address() {
    let context = &InterpreterContext::default();

    assert_eq!(b"________________________________".to_vec(), interpret_string("address:", context));
    assert_eq!(b"a_______________________________".to_vec(), interpret_string("address:a", context));
    assert_eq!(b"an_address______________________".to_vec(), interpret_string("address:an_address", context));
    assert_eq!(b"12345678901234567890123456789012".to_vec(), interpret_string("address:12345678901234567890123456789012", context));
    assert_eq!(b"12345678901234567890123456789012".to_vec(), interpret_string("address:123456789012345678901234567890123", context));
}

#[test]
fn test_unsigned_number() {
    let context = &InterpreterContext::default();

    assert_eq!(vec![0x12, 0x34], interpret_string("0x1234", context));
    assert_eq!(EMPTY, interpret_string("0x", context));
    assert_eq!(EMPTY, interpret_string("0", context));
    assert_eq!(vec![12], interpret_string("12", context));
    assert_eq!(vec![0x01, 0x00], interpret_string("256", context));
    assert_eq!(vec![0x01], interpret_string("0b1", context));
    assert_eq!(vec![0x05], interpret_string("0b101", context));
}

#[test]
fn test_signed_number() {
    let context = &InterpreterContext::default();

    assert_eq!(vec![0xff], interpret_string("-1", context));
    assert_eq!(vec![0xff], interpret_string("255", context));
    assert_eq!(vec![0xff], interpret_string("0xff", context));
    assert_eq!(vec![0x00, 0xff], interpret_string("+255", context));
    assert_eq!(vec![0x00, 0xff], interpret_string("+0xff", context));

    assert_eq!(vec![0xff, 0x00], interpret_string("-256", context));
    assert_eq!(vec![0xfb], interpret_string("-0b101", context));
}
