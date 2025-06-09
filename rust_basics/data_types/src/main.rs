use std::env;
mod compound;
mod scalar;
use clap::{Parser, Subcommand};

// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    print: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Write {
        /// Optionally set a title for what you are going to write about
        #[clap(short, long)]
        title: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.print.as_str() {
        "int" => scalar::int(),
        "float" => scalar::float(),
        "sum" => scalar::sum(),
        "difference" => scalar::difference(),
        "multiply" => scalar::multiply(),
        "divide" => scalar::divide(),
        "bool" => scalar::bool(),
        "char" => scalar::char(),
        _ => println!("command was not found"),
    }
}
