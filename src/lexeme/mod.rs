/// lexeme mod implements the lexeme used by lexer.
use std::fmt::Display;
pub mod char;
pub mod id;
pub mod keyword;
pub mod number;
pub mod op;

/// The lexeme used to generate a token needs to implements this trait.
pub trait Type {
    fn r#type(&self) -> &str {
        ""
    }

    fn equal_type<T: Display>(&self, e: T) -> bool {
        self.r#type() == e.to_string()
    }
}

/// All lexeme need to implements this trait.
pub trait Value {
    fn value(&self) -> &str {
        ""
    }

    fn equal_value<T: Display>(&self, e: T) -> bool {
        self.value() == e.to_string()
    }
}
