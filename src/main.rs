use clap::Parser;
use std::time::{Instant, Duration};

mod args;
mod y2020;
mod y2021;
mod y2022;
mod y2023;

use crate::args::Args;

fn main() {
    let args = Args::parse();
    println!("Solving AoC {}/{:02}/{:?}...", args.year, args.day, args.part);

    let input_str = get_input(args.year, args.day);
    let input = input_str.lines().collect::<Vec<_>>();
    println!("Input: {} lines @ {:.1} kB", input.len(), input_str.len() / 1024);
    
    let total: Duration = (0..args.iterations).map(|i| {
        let start = Instant::now();
        let solution = match args.year {
            2020 => y2020::solve(args.day, args.part, &input),
            2021 => y2021::solve(args.day, args.part, &input),
            2022 => y2022::solve(args.day, args.part, &input),
            2023 => y2023::solve(args.day, args.part, &input),
            _ => { println!("Unsupported year: {}", args.year); String::new() },
        };
        let duration = start.elapsed();
        if i == 0 {
            if solution.is_empty() {
                println!("No solution output.");
            } else {
                println!("Solution: {solution} (in {duration:?})");
            }
        }
        duration
    }).sum();

    if args.iterations > 1 {
        let mean = total / args.iterations;
        println!("Mean: {mean:?} ({} runs)", args.iterations);
    }
}

fn get_input(year: u16, day: u8) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session = std::env::var("AOC_SESSION").expect("Could not fetch env var AOC_SESSION");
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
