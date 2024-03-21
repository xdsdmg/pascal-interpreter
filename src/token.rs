/// token.rs implements the model of token used by interpreter.

#[derive(Debug)]
pub struct Token {
    r#type: String,
    value: String,
}

impl Token {
    pub fn new(r#type: &str, value: &str) -> Token {
        Token {
            r#type: r#type.to_string(),
            value: value.to_string(),
        }
    }

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
