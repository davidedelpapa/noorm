use std::path::PathBuf;
use serde::Deserialize;
use toml;

use super::ParserConfigError;

/// Dialects used to parse SQL.
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Dialect {
    /// A permissive, general purpose Dialect, which parses a wide variety of SQL statements, from many different dialects.
    Generic,
    /// MySQL dialect
    MySQL,   
}

/// Parser configuration
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ParserConfig {
    /// Dialect used for SQL parsing
    pub dialect: Dialect,
    /// Directory where the migrations are found
    pub migrations: PathBuf,
    /// Directory where the queries are found
    pub queries: PathBuf,
}
impl ParserConfig {
    /// Create a new ParserConfig.
    /// 
    /// # Examples
    ///
    /// ```
    /// use noorm::prelude::*;
    /// 
    /// let config = ParserConfig::new();
    /// ```
    pub fn new() ->Self {
        Self {
            dialect: Dialect::Generic,
            migrations: PathBuf::new(),
            queries: PathBuf::new(),
        }
    }

    /// Create a new ParserConfig from a toml string.
    /// 
    /// # Arguments
    ///
    /// * `conf_string` - A TOML configuration string.
    /// 
    /// # Errors
    /// 
    /// * Can return a `ParserConfigError` if the TOML parsing fails
    /// 
    /// # Examples
    ///
    /// ```
    /// use noorm::prelude::*;
    /// 
    /// let conf_string = r#"
    ///     
    ///     migrations = '.'
    ///     queries = '.'
    ///     dialect = {type = "Generic"}
    /// "#;
    /// 
    /// let config = ParserConfig::from_toml(&conf_string).unwrap();
    /// ```
    pub fn from_toml(conf_string: &str) -> Result<Self, ParserConfigError> {
        let config: ParserConfig = toml::from_str(conf_string)?;
        Ok(config)
    }
}

/// Generic Parser.
pub struct Parser {
    conf: ParserConfig
}
impl Parser {
    /// Create a new Parser.
    /// 
    /// # Examples
    ///
    /// ```
    /// use noorm::prelude::*;
    /// 
    /// let parser = Parser::new();
    /// ```
    pub fn new() -> Self {
        Self {
            conf: ParserConfig::new(), 
        }
    }

    /// Set a Parser's configuration.
    /// 
    /// # Arguments
    ///
    /// * `config` - A parserConfig.
    ///
    /// # Examples
    ///
    /// ```
    /// use noorm::prelude::*;
    /// 
    /// let config = ParserConfig::new();
    /// let parser = Parser::new().set_config(config);
    /// ```
    pub fn set_config (mut self, config: ParserConfig) -> Self {
        self.conf = config;
        self
    }

    /// Get a Parser's configuration.
    /// 
    /// # Returns
    ///
    /// * `ParserConfig` - A ParserConfig.
    ///
    /// # Examples
    ///
    /// ```
    /// use noorm::prelude::*;
    /// 
    /// let config = ParserConfig::new();
    /// let parser = Parser::new().set_config(config.clone());
    /// assert_eq!(&config, parser.get_config())
    /// ```
    pub fn get_config (&self,) -> &ParserConfig {
        &self.conf
    }

    /// Run a Parser.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use noorm::prelude::*;
    /// 
    /// let config = ParserConfig::new();
    /// let parser = Parser::new().set_config(config);
    /// parser.parse();
    /// ```
    pub fn parse (&self) {
        todo!();
    }
}
