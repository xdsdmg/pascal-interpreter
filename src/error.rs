use core::fmt;
use std::error;

// Ref: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html
#[derive(Debug)]
pub enum Error {
    InvalidSyntax,
    VarNotFound,
    FileNotFound,
    VarRedefined,
    ProcedureNotFound,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidSyntax => write!(f, "invalid syntax"),
            Error::VarNotFound => write!(f, "variable not found"),
            Error::FileNotFound => write!(f, "file not found"),
            Error::VarRedefined => write!(f, "variable was redefined"),
            Error::ProcedureNotFound => write!(f, "procedure not found"),
        }
    }
}

impl Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Error::InvalidSyntax => "invalid syntax",
            Error::VarNotFound => "variable not found",
            Error::FileNotFound => "file not found",
            Error::VarRedefined => "variable was redefined",
            Error::ProcedureNotFound => "procedure not found",
        }
    }
}
