use anyhow::{bail, Result};
use std::collections::HashMap;

use crate::lines_from_file;

fn check_for_digit(substr: String) -> Option<char> {
    let digits_map = HashMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for (k, v) in digits_map.iter() {
        if substr.starts_with(k) {
            return Some(*v);
        }
    }

    None
}

fn parse_line(str: String) -> Result<i32> {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for i in 0..str.len() {
        if first.is_none() {
            let c = str.chars().nth(i).unwrap();
            if c.is_digit(10) {
                first = Some(c);
            } else {
                first = check_for_digit(str[i..].to_string());
            }
        }

        if last.is_none() {
            let c = str.chars().nth(str.len() - i - 1).unwrap();
            if c.is_digit(10) {
                last = Some(c);
            } else {
                last = check_for_digit(str[(str.len() - i - 1)..].to_string());
            }
        }

        if first.is_some() && last.is_some() {
            break;
        }
    }

    let catstr = match (first, last) {
        (Some(f), Some(l)) => format!("{}{}", f, l),
        _ => String::from(""),
    };

    match catstr.parse::<i32>() {
        Ok(res) => Ok(res),
        Err(e) => bail!("Failed to parse string: {}", e),
    }
}

fn process(input: Vec<String>) -> Result<i32> {
    let mut add = 0;
    for str in input {
        match parse_line(str) {
            Ok(res) => add = add + res,
            Err(e) => bail!("Failed to parse line: {}", e),
        }
    }
    Ok(add)
}

pub fn run() {
    print!("Day 1: ");
    let res = process(lines_from_file("./input/day01.txt"));
    println!("{:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day01::process;

    #[test]
    fn test1() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".into(),
            "a1b2c3d4e5f".into(),
            "treb7uchet".into(),
        ];
        assert!(process(input).unwrap() == 142);
    }

    #[test]
    fn test2() {
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".into(),
            "abcone2threexyz".into(),
            "xtwone3four".into(),
            "4nineeightseven2".into(),
            "zoneight234".into(),
            "7pqrstsixteen".into(),
        ];
        assert!(process(input).unwrap() == 281);
    }
}
