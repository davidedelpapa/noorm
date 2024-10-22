use std::path::PathBuf;
use noorm::{parser::Language, prelude::*};

#[test]
fn test_parser_builder() {
    let migrations = PathBuf::new();
    let queries = PathBuf::new();

    let config = ParserConfig::new();
    assert_eq!(config, ParserConfig{
        sql_dialect: Dialect::Generic,
        language: Language::default(),
        migrations,
        queries,
    });
    let parser = Parser::new().set_config(config.clone());
    assert_eq!(&config, parser.get_config())
}
