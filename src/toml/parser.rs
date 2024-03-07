// This is a rudimentary parser for the TOML format.
// It's not comprehensive and doesn't handle all potential scenarios.
// It's specifically designed to only parse key-value pairs and tables.

use super::{
    data::{Toml, TomlValue, TomlValueKind},
    lexer::{Token, TokenType},
};

pub struct Parser {
    /// Tokens returned from the lexer
    tokens: Vec<Token>,

    /// Current index in the Vec of tokens
    position: usize,
}

impl Parser {
    /// Create a new parser for the given tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /// Consume the next token in the input string
    fn consume_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    /// Peek at the next token in the input string without consuming it
    /// The position is incremented only when `consume_token` is called, not here.
    /// To consume the token, invoke `consume_token` which will increment the position.
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Parse a key-value pair (per the TokenType::Key)
    fn parse_key(&mut self) -> Option<TomlValue> {
        let key = self.consume_token()?.value.clone();
        self.consume_token()?; // consume the equals sign
        let value = self.consume_token()?.value.clone();

        let kind = match value.parse::<i64>() {
            Ok(int) => TomlValueKind::Integer(int),
            Err(_) => match value.as_str() {
                "true" => TomlValueKind::Boolean(true),
                "false" => TomlValueKind::Boolean(false),
                _ => TomlValueKind::String(value),
            },
        };

        Some(TomlValue { key, kind })
    }

    /// Parse a table (per the TokenType::LeftBracket)
    fn parse_table(&mut self) -> Option<TomlValue> {
        self.consume_token()?; // consume the left bracket
        let name = self.consume_token()?.value.clone();
        self.consume_token()?; // consume the right bracket

        let mut key_values: Vec<TomlValue> = vec![];

        // parse key-value pairs (and add them to key_values) until we hit two newlines in a row
        while let Some(token) = self.peek_token() {
            if token.token_type == TokenType::Newline {
                self.consume_token(); // consume the newline
                if let Some(token) = self.peek_token() {
                    // if the next token is also a newline, we're done parsing the table
                    if token.token_type == TokenType::Newline {
                        break;
                    }
                }
            }

            if let Some(kv) = self.parse_key() {
                // Push only if key isn't duplicate
                if !key_values.iter().any(|e| e.key == kv.key) {
                    key_values.push(kv);
                } else {
                    println!("Duplicate key for table key-value pair found: '{}'", kv.key);
                }
            }
        }

        Some(TomlValue {
            key: name,
            kind: TomlValueKind::Table(key_values),
        })
    }

    /// Parse the input tokens and return a vector of TomlElements
    pub fn parse(&mut self) -> Toml {
        let mut elements: Vec<TomlValue> = vec![];

        while let Some(token) = self.peek_token() {
            match token.token_type {
                TokenType::Key => {
                    if let Some(kv) = self.parse_key() {
                        // If key isn't duplicate
                        if !elements.iter().any(|e| e.key == kv.key) {
                            elements.push(kv);
                        } else {
                            println!("Duplicate key for key-value pair found: {}", kv.key)
                        }
                    }
                }
                TokenType::LeftBracket => {
                    if let Some(kv) = self.parse_table() {
                        // If key isn't duplicate
                        if !elements.iter().any(|e| e.key == kv.key) {
                            elements.push(kv);
                        } else {
                            println!("Duplicate key for table name found: {}", kv.key)
                        }
                    }
                }
                _ => {
                    self.consume_token();
                }
            }
        }

        Toml { data: elements }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toml::lexer::Lexer;
    use std::fs;

    #[test]
    fn toml_parser() {
        let toml_string = fs::read_to_string("src/toml/test.toml").unwrap();
        let mut lexer = Lexer::new(&toml_string);
        let mut parser = Parser::new(lexer.collect());

        let toml = parser.parse();

        println!("{:#?}", toml.data);
    }
}
