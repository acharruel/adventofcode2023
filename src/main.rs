use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

/// Advent of code 2023
#[derive(Debug, Parser)]
struct Arguments {
    #[clap(short, long)]
    /// Index of the day
    day: i32,
}

fn main() {
    let args = Arguments::parse();

    println!("Advent of code 2023");
    println!("Day {}:", &args.day);

    match args.day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        _ => println!("Day {} not covered yet...", args.day),
    }
}
