use clap::Parser as cParser;
use std::path::PathBuf;
use noorm::prelude::*;

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
    
    let config = ParserConfig{
        dialect: Dialect::Generic,
        migrations,
        queries,
    };
    let _parser = Parser::new().set_config(config);

    println!("So far everything looks OK, boss!");
}
