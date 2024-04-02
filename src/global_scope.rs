use crate::ast::procedure::Procedure;
use crate::error::Error;
use crate::lexer::lexeme::keyword::Keyword;
use crate::lexer::lexeme::number::NumberType;
use crate::lexer::lexeme::{Type, Value};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    rc::Rc,
};

/* KeywordSymbol */
pub struct KeywordSymbol {
    r#type: String,
    value: String,
}

impl KeywordSymbol {
    pub fn new(r#type: &str, value: &str) -> KeywordSymbol {
        KeywordSymbol {
            r#type: r#type.to_string(),
            value: value.to_string(),
        }
    }
}

impl Display for KeywordSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{type: {}, value: {}}}", self.r#type, self.value)
    }
}

impl Clone for KeywordSymbol {
    fn clone(&self) -> Self {
        Self {
            r#type: self.r#type.clone(),
            value: self.value.clone(),
        }
    }
}

/* VariableSymbol */
pub struct VariableSymbol {
    r#type: NumberType,
    value: Option<String>,
}

impl VariableSymbol {
    pub fn new(r#type: NumberType, value: Option<String>) -> VariableSymbol {
        VariableSymbol {
            r#type,
            value: value.clone(),
        }
    }

    pub fn r#type(&self) -> NumberType {
        self.r#type
    }

    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }
}

impl Display for VariableSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.value.clone().unwrap_or(String::from("")); // TODO: need op
        write!(f, "{{type: {}, value: {}}}", self.r#type.r#type(), &value)
    }
}

impl Clone for VariableSymbol {
    fn clone(&self) -> Self {
        Self {
            r#type: self.r#type,
            value: self.value.clone(),
        }
    }
}

/* ProcedureSymbol */
pub struct ProcedureSymbol {
    name: String,
    procedure: Rc<Procedure>,
}

impl ProcedureSymbol {
    pub fn new(name: &str, procedure: Rc<Procedure>) -> ProcedureSymbol {
        ProcedureSymbol {
            name: name.to_string(),
            procedure,
        }
    }
}

impl Display for ProcedureSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{name: {}}}", self.name,)
    }
}

impl Clone for ProcedureSymbol {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            procedure: self.procedure.clone(),
        }
    }
}

pub enum Identifier {
    Keyword(KeywordSymbol),
    Variable(VariableSymbol),
    Procedure(ProcedureSymbol),
}

impl Identifier {
    pub fn r#type(&self) -> &'static str {
        match self {
            Identifier::Keyword(_) => "Keyword Symbol",
            Identifier::Variable(_) => "Variable Symbol",
            Identifier::Procedure(_) => "Procedure Symbol",
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Keyword(ks) => {
                write!(f, "type: {}, value: {}", self.r#type(), ks)
            }
            Identifier::Variable(vs) => {
                write!(f, "type: {}, value: {}", self.r#type(), vs)
            }
            Identifier::Procedure(ps) => {
                write!(f, "type: {}, value: {}", self.r#type(), ps)
            }
        }
    }
}

impl Clone for Identifier {
    fn clone(&self) -> Self {
        match self {
            Identifier::Keyword(ks) => Identifier::Keyword(ks.clone()),
            Identifier::Variable(vs) => Identifier::Variable(vs.clone()),
            Identifier::Procedure(ps) => Identifier::Procedure(ps.clone()),
        }
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
            symbol_table.insert(
                kw.value().to_string(),
                Identifier::Keyword(KeywordSymbol::new(kw.r#type(), kw.value())),
            );
        }

        Scope {
            name: name.to_string(),
            symbol_table,
            parent,
            level,
        }
    }

    pub fn set(&mut self, key: &str, id: Identifier) -> Result<(), Error> {
        if let Some(_) = self.symbol_table.get(key) {
            return Err(Error::VarRedeclared);
        }

        self.symbol_table.insert(key.to_string(), id);

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
            return Some(id);
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
