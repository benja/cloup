use super::{lexer::Lexer, parser::Parser};

#[derive(Debug)]
pub enum TomlValueKind {
    String(String),
    Integer(i64),
    Boolean(bool),
    Table(Vec<TomlValue>),
}

#[derive(Debug)]
pub struct TomlValue {
    pub key: String,
    pub kind: TomlValueKind,
}

#[derive(Debug)]
pub struct Toml {
    pub data: Vec<TomlValue>,
}

impl From<String> for Toml {
    fn from(value: String) -> Self {
        let mut lexer = Lexer::new(&value);
        Parser::new(lexer.collect()).parse()
    }
}

impl From<Option<Vec<TomlValue>>> for Toml {
    fn from(value: Option<Vec<TomlValue>>) -> Self {
        if let Some(data) = value {
            return Toml { data };
        }

        Toml { data: vec![] }
    }
}

impl Toml {
    /// Create a new Toml structure
    pub fn new() -> Self {
        Toml { data: vec![] }
    }

    /// Get a value from the TOML file
    pub fn get(&self, key: &str) -> Option<&TomlValueKind> {
        self.data.iter().find(|v| v.key == key).map(|v| &v.kind)
    }

    /// Get a mutable reference to a value from the TOML file
    pub fn get_mut(&mut self, key: &str) -> Option<&mut TomlValueKind> {
        self.data
            .iter_mut()
            .find(|v| v.key == key)
            .map(|v| &mut v.kind)
    }

    /// Set a value in the TOML file
    pub fn set(&mut self, key: String, kind: TomlValueKind) {
        // Check if key is alphanumeric
        if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            panic!("Key must be alphanumeric: {}", key);
        }

        let data = TomlValue { key, kind };

        // If key exists, overwrite
        if let Some(index) = self.data.iter().position(|v| v.key == data.key) {
            self.data[index] = data;
        } else {
            self.data.push(data)
        }
    }

    /// Delete a value from the TOML file
    pub fn delete(&mut self, key: &str) {
        self.data.retain(|v| v.key != key);
    }

    /// Convert the TOML structure to a TOML string
    pub fn to_toml(&self) -> String {
        let mut toml = String::new();

        for value in &self.data {
            match &value.kind {
                TomlValueKind::String(val) => {
                    toml.push_str(&format!("{} = \"{}\"\n", value.key, val))
                }
                TomlValueKind::Integer(val) => toml.push_str(&format!("{} = {}\n", value.key, val)),
                TomlValueKind::Boolean(val) => toml.push_str(&format!("{} = {}\n", value.key, val)),
                TomlValueKind::Table(val) => {
                    if self.data.first().unwrap().key == value.key {
                        toml.push_str(&format!("[{}]\n", value.key));
                    } else {
                        toml.push_str(&format!("\n[{}]\n", value.key));
                    }

                    let key_values = val.iter();

                    for kv in key_values {
                        match &kv.kind {
                            TomlValueKind::String(val) => {
                                toml.push_str(&format!("{} = \"{}\"\n", kv.key, val))
                            }
                            TomlValueKind::Integer(val) => {
                                toml.push_str(&format!("{} = {}\n", kv.key, val))
                            }
                            TomlValueKind::Boolean(val) => {
                                toml.push_str(&format!("{} = {}\n", kv.key, val))
                            }
                            _ => (),
                        }
                    }

                    toml.push('\n')
                }
            }
        }

        format!("{}\n", toml.trim_end())
    }
}
