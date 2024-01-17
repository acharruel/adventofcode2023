use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Result};

use crate::lines_from_file;

fn parse_game(line: &str) -> Result<i32> {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut possible = true;
    let tokens: Vec<&str> = line.split(' ').collect();
    let mut chunks = tokens.chunks(2);

    // check first chunk to get game index
    let id = chunks.next().unwrap()[1].replace(':', "");

    // iterate next chunks and check whether game is possible
    for elem in chunks {
        let n: i32 = FromStr::from_str(elem[0])?;
        let color = elem[1].replace([',', ';'], "");

        if color.eq("blue") && n > MAX_BLUE {
            possible = false;
            break;
        }

        if color.eq("green") && n > MAX_GREEN {
            possible = false;
            break;
        }

        if color.eq("red") && n > MAX_RED {
            possible = false;
            break;
        }
    }

    match possible {
        true => Ok(id.parse()?),
        false => Ok(0),
    }
}

fn parse_game_power(line: &str) -> Result<i32> {
    let tokens: Vec<&str> = line.split(' ').collect();
    let mut chunks = tokens.chunks(2);
    let mut color_map: HashMap<&str, i32> = [("blue", 0), ("green", 0), ("red", 0)].into();

    // skip first chunk containing game id
    chunks.next();

    // iterate next chunks
    for elem in chunks {
        let n: i32 = FromStr::from_str(elem[0])?;
        let color = elem[1].replace([',',';'], "").to_string();

        // replace value in color_map if greater than existing
        if let Some(value) = color_map.get_mut(color.as_str()) {
            if n > *value {
                *value = n;
            }
        }
    }

    let mut power = 1;
    for (_, value) in color_map {
        power *= value;
    }

    Ok(power)
}

fn process(input: Vec<String>) -> Result<i32> {
    let mut add = 0;
    for line in input {
        match parse_game(line.as_str()) {
            Ok(res) => add += res,
            Err(_) => bail!("Failed to parse line"),
        }
    }
    Ok(add)
}

fn process_power(input: Vec<String>) -> Result<i32> {
    let mut add = 0;
    for line in input {
        match parse_game_power(line.as_str()) {
            Ok(res) => add += res,
            Err(_) => bail!("Failed to parse line"),
        }
    }
    Ok(add)
}

pub fn run() {
    let res = process(lines_from_file("./input/day02.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process_power(lines_from_file("./input/day02.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day02::{process, process_power};

    #[test]
    fn test1() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".into(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".into(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".into(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".into(),
        ];
        assert!(process(input).unwrap() == 8);
    }

    #[test]
    fn test2() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".into(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".into(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".into(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".into(),
        ];
        assert!(process_power(input).unwrap() == 2286);
    }
}
