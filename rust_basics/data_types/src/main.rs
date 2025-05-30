use std::env;
mod scalar;
mod tuple;
use clap::Parser;

// Scalar
//
// fn main() {
//     // list of all arguments passed to the binary
//     let args: Vec<String> = env::args().collect();
//     // first argument
//     // TODO allow for handling of arguments
//     let query = args.get(1).expect("Expected first argument").to_lowercase();
//     if query == "scalar" {
//         println!("It worked");
//     } else if query == "vector" {
//         println!("vector");
//     } else if query == "string" {
//         println!("string");
//     } else if query == "tuple" {
//         println!("tuple");
//     } else {
//         panic!("Expected first argument of either scalar, vector, string, or tuple");
//     }
// }

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
