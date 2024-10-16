use std::path::PathBuf;

/// Dialects used to parse SQL.
#[derive(Debug, PartialEq, Clone)]
pub enum Dialect {
    /// A permissive, general purpose Dialect, which parses a wide variety of SQL statements, from many different dialects.
    Generic,
    /// MySQL dialect
    MySQL,   
}

/// Parser configuration
#[derive(Debug, PartialEq, Clone)]
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
