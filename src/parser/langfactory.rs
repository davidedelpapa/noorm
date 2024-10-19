use serde::Deserialize;
use sqlparser::ast::DataType;
use super::{langgen::LanguageGenerator, JavaScriptGenerator, PythonGenerator, RustGenerator};

/// Language selection enum
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Language {
    /// Rust language implementation
    Rust(RustGenerator),
    /// Python language implementation
    Python(PythonGenerator),
    /// JavaScript language implementation
    JavaScript(JavaScriptGenerator),
}
impl Default for Language{
    fn default() -> Self {
        Language::Rust(RustGenerator)
    }
}

/// Language selection implementations
impl Language {
    /// Create new Language
    pub fn new(language_name: &str) -> Option<Self> {
        match language_name.to_lowercase().as_str() {
            "rust" => Some(Language::Rust(RustGenerator)),
            "python" => Some(Language::Python(PythonGenerator)),
            "javascript" => Some(Language::JavaScript(JavaScriptGenerator)),
            _ => None, // Handle unsupported languages
        }
    }
    
    /// Parse a `CREATE TABLE` statement
    pub fn parse_create_table(&self, sql: &str) -> String {
        match self {
            Language::Rust(gen) => gen.parse_create_table(sql),
            Language::Python(gen) => gen.parse_create_table(sql),
            Language::JavaScript(gen) => gen.parse_create_table(sql),
        }
    }
    
    /// Convert `SQL DataType`s to the target languages' ones
    pub fn sql_to_type(&self, data_type: &DataType, is_nullable: bool) -> String {
        match self {
            Language::Rust(gen) => gen.sql_to_type(data_type, is_nullable),
            Language::Python(gen) => gen.sql_to_type(data_type, is_nullable),
            Language::JavaScript(gen) => gen.sql_to_type(data_type, is_nullable),
        }
    }
}
