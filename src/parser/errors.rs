use thiserror::Error;

/// ParserConfig Errors.
#[derive(Error, Debug)]
pub enum ParserConfigError {
    /// ParserConfig TOML import error
    #[error("ParserConfig TOML import error")]
    TomlImport(#[from] toml::de::Error),
    /// Unknown ParserConfig error
    #[error("Unknown ParserConfig error")]
    Unknown,
}

/// Parser.parse() Errors.
#[derive(Error, Debug)]
pub enum ParserError {
    /// Parser statement error
    #[error("Parser statement error")]
    Statement,
    /// Unknown Parser error
    #[error("Unknown Parser error")]
    Unknown,
}