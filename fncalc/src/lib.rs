mod branch;
mod expression;
mod function;
mod instruction;
mod parser;
mod session;
mod token;
mod while_loop;

#[cfg(test)]
mod tests;

use ::rust_decimal::prelude::*;

type Value = Decimal;
const ONE: Value = Decimal::ONE;
const NEGATIVE_ONE: Value = Decimal::NEGATIVE_ONE;
const ZERO: Value = Decimal::ZERO;
const PI: Value = Decimal::PI;

const LOOP_LIMIT: usize = 1_000_000;
const FUNCTION_CALL_LIMIT: usize = 500;

pub fn reset_session() {
    session::clear();
}

pub fn process(input: &str) -> String {
    let mut output_stream = String::new();
    let parse_result = parser::parse(input);
    match parse_result {
        Ok(instructions) => {
            let mut result: Option<Value> = None;
            for item in instructions {
                match item.exec(&mut output_stream) {
                    Ok(output) => match output {
                        instruction::ReturnValue::Value(value) => result = Some(value),
                        instruction::ReturnValue::Return(value) => {
                            return format_value(value);
                        }
                        instruction::ReturnValue::None => (),
                        instruction::ReturnValue::Break => {
                            return match result {
                                Some(value) => {
                                    output_stream.push_str(&format_value(value));
                                    if output_stream.ends_with('\n') {
                                        output_stream.pop();
                                    }
                                    output_stream
                                }
                                None => {
                                    if output_stream.ends_with('\n') {
                                        output_stream.pop();
                                    }
                                    output_stream
                                }
                            };
                        }
                    },
                    Err(e) => return e.to_string(),
                }
            }
            match result {
                Some(value) => {
                    if output_stream.is_empty() {
                        output_stream.push_str(&format_value(value));
                    }
                    if output_stream.ends_with('\n') {
                        output_stream.pop();
                    }
                    output_stream
                }
                None => {
                    if output_stream.ends_with('\n') {
                        output_stream.pop();
                    }
                    output_stream
                }
            }
        }
        Err(e) => e.to_string(),
    }
}

fn format_value(number: Decimal) -> String {
    number.round_dp(6).normalize().to_string()
}

#[allow(dead_code)]
fn format_float(number: f64) -> String {
    let mut string = format!("{:.6}", number);
    while string.ends_with('0') {
        string.pop();
    }
    if string.ends_with('.') {
        string.pop();
    }
    string
}

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn fncalc_process(input: *const c_char) -> *mut c_char {
    let cstr = unsafe { CStr::from_ptr(input) };
    CString::new(process(cstr.to_str().unwrap()))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn fncalc_reset() {
    session::clear();
}

#[no_mangle]
pub extern "C" fn fncalc_free(input: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(input);
    }
}
