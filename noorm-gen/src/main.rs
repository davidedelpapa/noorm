use clap::Parser as cParser;
use noorm::{parser::Language, prelude::*};
use std::path::PathBuf;

/// A simple CLI to generate NoORM code.
#[derive(cParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path of the migrations
    #[arg(short, long)]
    migrations: String,
    /// The path of the queries
    #[arg(short, long)]
    queries: String,
}

fn main() {
    let cli = Cli::parse();
    let migrations = PathBuf::from(cli.migrations);
    let queries = PathBuf::from(cli.queries);

    let config = ParserConfig {
        sql_dialect: Dialect::Generic,
        language: Language::new("dummy").expect("Unsupported language!"),
        migrations,
        queries,
    };
    let sql_statement = "
    CREATE TABLE person ( Id INTEGER NOT NULL, name VARCHAR(255), phone_number VARCHAR(255) );
    CREATE TABLE office ( Id INTEGER NOT NULL, name VARCHAR(255) );
    ";
    let mut parser = Parser::new()
        .set_config(config)
        .statement(sql_statement);
    if parser.parse().is_ok() {
        println!("{}", parser.output.unwrap())
    } else {
        println!("No bueno!");
    }
}
