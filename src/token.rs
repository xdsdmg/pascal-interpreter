use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Token {
    r#type: String,
    value: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{type: \"{}\", value: \"{}\"}}",
            &self.r#type, &self.value
        )
    }
}

impl Token {
    pub fn new(r#type: &str, value: &str) -> Token {
        Token {
            r#type: r#type.to_string(),
            value: value.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn clone(&self) -> Token {
        Token::new(&self.r#type, &self.value)
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
