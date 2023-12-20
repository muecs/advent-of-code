use clap::Parser;
use std::{time::{Instant, Duration}, path::Path};

mod args;
mod utils;
mod y2020;
mod y2021;
mod y2022;
mod y2023;

use crate::args::Args;

type SolveFunc = fn(&Vec<&str>) -> String;

fn main() {
    let args = Args::parse();
    println!("Solving AoC {}/{:02}/{:?}...", args.year, args.day, args.part);

    let solve: SolveFunc = match args.year {
        2020 => y2020::solver(args.day, args.part),
        2021 => y2021::solver(args.day, args.part),
        2022 => y2022::solver(args.day, args.part),
        2023 => y2023::solver(args.day, args.part),
        _ => unimplemented!("Unsupported year: {}", args.year),
    };

    let input_str = get_input(args.year, args.day);
    let input = input_str.lines().collect::<Vec<_>>();
    println!("Input: {} lines @ {:.1} kB", input.len(), input_str.len() / 1024);
    
    let total: Duration = (0..args.iterations).map(|i| {
        let start = Instant::now();
        let solution = solve(&input);
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
    let cache_path = format!("cache/y{year}/d{day}.txt");
    let cache_path = Path::new(&cache_path);
    if let Ok(input) = std::fs::read_to_string(&cache_path) {
        return input;
    }

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session = std::env::var("AOC_SESSION").expect("Could not fetch env var AOC_SESSION");
    let input = reqwest::blocking::Client::default()
        .get(url)
        .header("cookie", format!("session={session}"))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap();

    if let Ok(_) = std::fs::create_dir_all(&cache_path.parent().unwrap()) {
        let _ = std::fs::write(&cache_path, &input);
    }

    input
}
