use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod day01;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("Avent of code 2023");
    day01::run();
}
