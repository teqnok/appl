mod main2;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    // Operation (Add, Remove, Update)
    #[arg(short, long)]
    operation: String
}

fn main() {
    let args = Args::parse();

    println!("{} package {}...", args.operation, args.name)
}