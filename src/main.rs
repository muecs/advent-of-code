use clap::Parser;
use std::time::SystemTime;

mod args;
mod y2021;
// mod y2022;

use crate::args::Args;

fn main() {
    let args = Args::parse();
    println!("Solving AoC {}/{:02}/{:?}...", args.year, args.day, args.part);

    let input_str = get_input(args.year, args.day);
    let input = input_str.lines().collect::<Vec<_>>();
    println!("Input: {} lines @ {:.1} kB", input.len(), input_str.len() / 1024);
    
    let start = SystemTime::now();
    let solution = match args.year {
        2021 => y2021::solve(args.day, args.part, &input),
        _ => { println!("Unsupported year: {}", args.year); String::new() },
    };
    if solution.is_empty() {
        println!("No solution output.");
    } else {
        let elapsed = start.elapsed().unwrap().as_millis();
        println!("Solution: {solution} (in {elapsed} ms)");
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
