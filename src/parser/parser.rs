use std::{fmt::Display, path::PathBuf};
use serde::Deserialize;
use toml;

use crate::parser::Language;

use super::{errors::ParserError, ParserConfigError};

/// Dialects used to parse SQL.
#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Dialect {
    /// A permissive, general purpose Dialect, which parses a wide variety of SQL statements, from many different dialects.
    #[default]
    Generic,
    /// MySQL dialect
    MySQL,   
}

/// Parser configuration
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ParserConfig {
    /// Dialect used for SQL parsing
    #[serde(default)]
    pub sql_dialect: Dialect,
    /// Target laguage for code generation
    #[serde(default)]
    pub language: Language,
    /// Directory where the migrations are found
    #[serde(default = "get_default_path")]
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
            sql_dialect: Dialect::Generic,
            language: Language::default(),
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
    ///     migrations = "migrations/"
    ///     queries = "queries/"
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

fn get_default_path() -> PathBuf {
    PathBuf::from(".").to_owned()
}

/// Generic Parser.
pub struct Parser {
    conf: ParserConfig,
    statement: Option<String>,
    /// Output of the parser exection
    pub output: Option<String>,
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
            statement: None,
            output: None, 
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

    /// Set a Parser's statement.
    /// 
    /// # Arguments
    ///
    /// * `str` - A statement to parse.
    ///
    /// # Examples
    ///
    /// ```
    /// use noorm::prelude::*;
    /// 
    /// let mut parser = Parser::new().statement("CREATE TABLE person ( Id INTEGER NOT NULL, name VARCHAR(255) )");
    /// ```
    pub fn statement (mut self, sql: impl Display) -> Self {
        let s = sql.to_string();
        self.statement = if s.is_empty() {
            None
        } else {
            Some(s)
        };
        self
    }

    /// Run a Parser.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use noorm::prelude::*;
    /// 
    /// let config = ParserConfig::new();
    /// let mut parser = Parser::new().set_config(config);
    /// parser.parse().expect("Parser error");
    /// ```
    pub fn parse (&mut self) -> Result<(), ParserError> {
        let lang = &mut self.conf.language;
 
        match &self.statement {
            Some(sql) => {
                lang.parse_create_table(sql.as_str());
                self.output = Some(lang.to_string());
                Ok(())
            },
            None => Err(ParserError::Statement),
        }
    }
}
