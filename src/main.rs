use clap::Parser;

/// CLI Interface for Pokemon Data
#[derive(Parser)]
#[clap(about, version)]
struct Cli {
    /// type of information to search [pokemon, move, info]
    #[clap(short,long,default_value="pokemon")]
    field: String,
    /// What to search for
    query: String,
}

fn main() {
    let args = Cli::parse();
    println!("Searching for the {}: `{}`", args.field, args.query);
}
