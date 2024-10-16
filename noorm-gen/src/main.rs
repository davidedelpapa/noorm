use clap::Parser;
use noorm::greet;

/// A simple CLI to greet people.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The name of the person to greet
    #[arg(short, long)]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    let greeting = greet(&cli.name);
    println!("{}", greeting);
}
