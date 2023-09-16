#![allow(dead_code)]

use crate::function::{Function, FunctionCall};
use crate::instruction;
use crate::Value;
use crate::FUNCTION_CALL_LIMIT;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

static GLOBAL_NAMESPACE: Lazy<Mutex<HashMap<String, Value>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static LOCAL_NAMESPACES: Lazy<Mutex<Vec<HashMap<String, Value>>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

static FUNCTION_STORE: Lazy<RwLock<HashMap<String, Function>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

static CALL_COUNT: Lazy<Mutex<usize>> =
    Lazy::new(||Mutex::new(0));

pub fn get_variable(var_name: &str) -> Option<Value> {
    let local_namespaces = LOCAL_NAMESPACES.lock().unwrap();
    let len = local_namespaces.len();
    if len > 0 {
        local_namespaces[len - 1].get(var_name).copied()
    } else {
        GLOBAL_NAMESPACE.lock().unwrap().get(var_name).copied()
    }
}

pub fn set_variable(var_name: &str, val: Value) {
    let mut local_namespaces = LOCAL_NAMESPACES.lock().unwrap();
    let len = local_namespaces.len();
    if len > 0 {
        local_namespaces[len - 1].insert(var_name.to_string(), val);
    } else {
        GLOBAL_NAMESPACE
            .lock()
            .unwrap()
            .insert(var_name.to_string(), val);
    }
}

pub fn clear() {
    GLOBAL_NAMESPACE.lock().unwrap().clear();
    LOCAL_NAMESPACES.lock().unwrap().clear();
    FUNCTION_STORE.write().unwrap().clear();
}

fn add_namespace(namespace: HashMap<String, Value>) {
    LOCAL_NAMESPACES.lock().unwrap().push(namespace);
}

fn pop_namespace() {
    LOCAL_NAMESPACES.lock().unwrap().pop();
}

pub fn add_function(name: String, value: Function) {
    FUNCTION_STORE.write().unwrap().insert(name, value);
}

fn increment_call_count() -> Result<(), instruction::Error> {
    let mut call_counter = CALL_COUNT.lock().unwrap();
    if *call_counter >= FUNCTION_CALL_LIMIT {
        Err(instruction::Error::new(String::new(), 0, instruction::ErrorKind::IterationLimitReached))
    } else {
        *call_counter += 1;
        Ok(())
    }
}

fn decrement_call_count() {
    let mut call_counter = CALL_COUNT.lock().unwrap();
    *call_counter -= 1;
}

pub fn call_function(
    fncall: &FunctionCall,
    pos: usize,
    output_stream: &mut String,
) -> instruction::Result {
    let fnstore = FUNCTION_STORE.read().unwrap();
    let function = match fnstore.get(fncall.name()) {
        Some(f) => f,
        None => {
            return Err(instruction::Error::new(
                fncall.context().to_string(),
                pos,
                instruction::ErrorKind::IdentifierNotFound,
            ));
        }
    };

    if function.argument_names().len() != fncall.arguments().len() {
        return Err(instruction::Error::new(
            fncall.context().to_string(),
            pos,
            instruction::ErrorKind::InvalidNumberOfArgument,
        ));
    }

    let mut fn_namespace: HashMap<String, Value> = HashMap::new();
    for i in 0..function.argument_names().len() {
        let expr = &fncall.arguments()[i];
        let result = expr
            .exec(output_stream)?
            .expect("Expressions should always return a value on success");

        fn_namespace.insert(function.argument_names()[i].clone(), result);
    }

    increment_call_count()?;
    add_namespace(fn_namespace);
    let result = function.exec(output_stream);
    pop_namespace();
    decrement_call_count();
    return result;
}
