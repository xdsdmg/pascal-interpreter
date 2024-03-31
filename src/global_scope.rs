use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    rc::Rc,
    sync::{Mutex, OnceLock},
};
use crate::error::Error;
use crate::lexer::lexeme::keyword::Keyword;
use crate::lexer::lexeme::{Type, Value};

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

pub struct Scope {
    // TODO: how about setting this to &str?
    name: String,
    symbol_table: HashMap<String, Identifier>,
    parent: Option<Rc<RefCell<Scope>>>,
    level: u32,
}

impl Scope {
    pub fn new(name: &str, parent: Option<Rc<RefCell<Scope>>>, level: u32) -> Scope {
        /* Initialize symbol table */
        let mut symbol_table = HashMap::new();
        for kw in [
            Keyword::Begin,
            Keyword::End,
            Keyword::Var,
            Keyword::Program,
            Keyword::Procedure,
        ] {
            symbol_table.insert(kw.value().to_string(), Identifier::new(kw.r#type(), None));
        }

        Scope {
            name: name.to_string(),
            symbol_table,
            parent,
            level,
        }
    }

    // TODO: op '&'
    pub fn set(&mut self, key: &str, id: &Identifier) -> Result<(), Error> {
        if let Some(_) = self.symbol_table.get(key) {
            return Err(Error::VarRedeclared);
        }

        self.symbol_table.insert(key.to_string(), id.clone());

        Ok(())
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Scope>>> {
        self.parent.clone()
    }

    pub fn find_in_cur_scope(&self, key: &str) -> Option<Identifier> {
        match self.symbol_table.get(key) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<Identifier> {
        if let Some(id) = self.find_in_cur_scope(key) {
            return Some(id.clone());
        }
        match self.parent() {
            Some(s) => s.borrow().get(key),
            None => None,
        }
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn print(&self) {
        println!("scope: {}, level: {}\nsymbol table:", self.name, self.level);
        for (k, v) in self.symbol_table.iter() {
            println!("key: {}, value: {}", k, v);
        }
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
