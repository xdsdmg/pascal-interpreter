use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::{Mutex, OnceLock},
};

pub struct Identifier {
    r#type: String,
    value: Option<String>,
}

impl Identifier {
    pub fn new(r#type: &str, value: Option<String>) -> Identifier {
        Identifier {
            r#type: r#type.to_string(),
            value,
        }
    }

    pub fn clone(&self) -> Identifier {
        Identifier {
            r#type: self.r#type.clone(),
            value: self.value.clone(),
        }
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type: {}, value: {}",
            self.r#type,
            self.value().unwrap_or(String::from(""))
        )
    }
}

pub fn global_scope() -> &'static Mutex<HashMap<String, Identifier>> {
    static GLOBAL_SCOPE: OnceLock<Mutex<HashMap<String, Identifier>>> = OnceLock::new();
    GLOBAL_SCOPE.get_or_init(|| Mutex::new(HashMap::<String, Identifier>::new()))
}

pub fn global_scope_set(key: &str, id: &Identifier) {
    global_scope()
        .lock()
        .unwrap()
        .insert(key.to_string(), id.clone());
}

pub fn global_scope_get(key: &str) -> Option<Identifier> {
    match global_scope().lock().unwrap().get(key) {
        Some(id) => Some(id.clone()),
        None => None,
    }
}

pub fn global_scope_print() {
    println!("GLOBAL SCOPE:");
    for (k, v) in global_scope().lock().unwrap().iter() {
        println!("id: {}, value: {}", k, v);
    }
}
