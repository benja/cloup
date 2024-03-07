// This is a rudimentary lexer for the TOML format.
// It's not comprehensive and doesn't handle all potential scenarios.
// It's specifically designed to only parse key-value pairs and tables.

use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Key,
    Value,
    Equals,
    Newline,
    LeftBracket,
    RightBracket,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

/// Tokenizer for the TOML format
pub struct Lexer<'a> {
    /// Source string to tokenize
    input: &'a str,

    /// Current position in the input string (used while tokenizing)
    position: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// Create a lexer for the given input string
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            position: input.chars().peekable(),
        }
    }

    /// Consume the next character in the input string
    fn consume_char(&mut self) -> Option<char> {
        self.position.next()
    }

    /// Peek at the next character in the input string without consuming it.
    /// The position is incremented only when `read_char` is called, not here.
    /// To consume the character, invoke `read_char` which will increment the position.
    fn peek_char(&mut self) -> Option<char> {
        self.position.peek().cloned()
    }

    /// Check if character represents a newline
    fn is_newline(&self, c: char) -> bool {
        c == '\n' || c == '\r'
    }

    /// Check if character is a valid key character
    fn is_key_char(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    /// Check if character is a valid value character
    fn is_value_char(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '"' || c == '_' || c == '\'' || c == '/' || c == '.'
    }

    /// Tokenize the input and return the next token
    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.consume_char() {
            if self.is_newline(c) {
                return Some(Token {
                    token_type: TokenType::Newline,
                    value: c.to_string(),
                });
            } else if c == '#' {
                // Skip comments until the end of the line
                while let Some(next_char) = self.consume_char() {
                    if self.is_newline(next_char) {
                        break;
                    }
                }

                // Return a newline token after skipping the comment
                return Some(Token {
                    token_type: TokenType::Newline,
                    value: "\n".to_string(),
                });
            } else if let Some(token_type) = match c {
                '=' => Some(TokenType::Equals),
                '[' => Some(TokenType::LeftBracket),
                ']' => Some(TokenType::RightBracket),
                _ => None, // Can be extended
            } {
                // Return a token for recognized characters
                return Some(Token {
                    token_type,
                    value: c.to_string(),
                });
            } else if self.is_key_char(c) {
                let mut value = c.to_string();

                // Keep reading characters until a non-key character is encountered
                // We peek at the next character to avoid consuming it unless it is a key character
                while let Some(next_char) = self.peek_char() {
                    if self.is_key_char(next_char) {
                        value.push(self.consume_char().unwrap());
                    } else {
                        break;
                    }
                }

                // Return a Value token if the first character is a number
                if value.chars().nth(0).unwrap().is_numeric() {
                    return Some(Token {
                        token_type: TokenType::Value,
                        value,
                    });
                }

                return Some(Token {
                    token_type: TokenType::Key,
                    value,
                });
            } else if self.is_value_char(c) {
                let mut value = c.to_string();

                // Keep reading characters until a non-value character is encountered
                // We peek at the next character to avoid consuming it unless it is a value character
                while let Some(next_char) = self.peek_char() {
                    if self.is_value_char(next_char) {
                        value.push(self.consume_char().unwrap());
                    } else {
                        break;
                    }
                }

                // Remove quotes from the value string
                if value.starts_with('"') && value.ends_with('"')
                    || value.starts_with('\'') && value.ends_with('\'')
                {
                    value = value[1..value.len() - 1].to_string();
                }

                // Return a value token for the value characters
                return Some(Token {
                    token_type: TokenType::Value,
                    value,
                });
            }
        }

        None // Return None when there are no more characters in the input
    }

    /// Tokenize the input and return all tokens
    pub fn collect(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn toml_lexer() {
        let toml_string = fs::read_to_string("src/toml/test.toml").unwrap();

        let mut lexer = Lexer::new(&toml_string);

        while let Some(token) = lexer.next_token() {
            println!("{:?}", token);
        }
    }
}
