use std::path::PathBuf;
use noorm::prelude::*;

#[test]
fn test_parser_builder() {
    let config = ParserConfig::new();
    assert_eq!(config, ParserConfig{
        sql_dialect: Dialect::Generic,
        migrations: PathBuf::new(),
        queries: PathBuf::new(),
    });
    let parser = Parser::new().set_config(config.clone());
    assert_eq!(&config, parser.get_config())
}
