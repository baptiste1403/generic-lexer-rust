use std::fmt::{self, Debug};

#[derive(Clone)]
pub struct Token {
    token_type: String,
    value: String,
}

impl Token {
    pub fn new(token_type: & str, value: & str) -> Self {
        Self {
            token_type : token_type.to_string(),
            value : value.to_string(),
        }
    }

    pub fn get_token_type(&self) -> &str {
        return &self.token_type;
    }

    pub fn get_value(&self) -> &str {
        return &self.value;
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Token")
            .field("token_type", &self.token_type)
            .field("value", &self.value)
            .finish()
    }
}