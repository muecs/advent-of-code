use clap::Parser;

mod args;
mod y2021;
// mod y2022;

use crate::args::Args;

fn main() {
    let args = Args::parse();

    let input_str = get_input(args.year, args.day);
    let input = input_str.lines().collect::<Vec<_>>();
    
    let solution = match args.year {
        2021 => y2021::solve(args.day, args.part, &input),
        _ => format!("Unsupported year: {}", args.year),
    };
    if solution.is_empty() {
        println!("No solution output.");
    } else {
        println!("{solution}");
    }
}

fn get_input(year: u16, day: u8) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session = std::env::var("AOC_SESSION").unwrap();
    reqwest::blocking::Client::default()
        .get(url)
        .header("cookie", format!("session={session}"))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap()
}
