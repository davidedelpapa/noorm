use std::path::PathBuf;
use noorm::{parser::{DummyAst, Language}, prelude::*};

#[test]
fn test_parser_builder() {
    let migrations = PathBuf::new();
    let queries = PathBuf::new();
    let d_ast = DummyAst::default();

    let config = ParserConfig::new();
    assert_eq!(config, ParserConfig{
        sql_dialect: Dialect::Generic,
        language: Language::Dummy(d_ast),
        migrations,
        queries,
    });
    let parser = Parser::new().set_config(config.clone());
    assert_eq!(&config, parser.get_config())
}
