use clap::Parser;

mod args;

use crate::args::Args;

fn main() {
    let args = Args::parse();

    // TODO: load input file
    
    let puzzle_id = format!("{}-{:02}-{:?}", args.year, args.day, args.part);
    let solution = match puzzle_id {
        // "2021-01-A" => 
        _ => format!("Unknown puzzle: {puzzle_id}"),
    };
    println!("{solution}");
}
