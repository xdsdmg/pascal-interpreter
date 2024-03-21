/// global_scope.rs implements Global Scope.
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

pub fn global_scope() -> &'static Mutex<HashMap<String, String>> {
    static GLOBAL_SCOPE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    GLOBAL_SCOPE.get_or_init(|| Mutex::new(HashMap::<String, String>::new()))
}

pub fn global_scope_set(key: &str, value: &str) {
    global_scope()
        .lock()
        .unwrap()
        .insert(key.to_string(), value.to_string());
}

pub fn global_scope_get(key: &str) -> Option<String> {
    match global_scope().lock().unwrap().get(key) {
        Some(v) => Some(v.to_string()),
        None => None,
    }
}

pub fn global_scope_print() {
    println!("GLOBAL SCOPE:");
    for (k, v) in global_scope().lock().unwrap().clone().into_iter() {
        println!("id: {}, value: {}", k, v);
    }
}
