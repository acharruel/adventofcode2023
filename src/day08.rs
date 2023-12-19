use num::integer::lcm;
use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::lines_from_file;

fn process(input: Vec<String>) -> Result<i32> {
    let commands = input[0].chars().collect::<Vec<char>>();
    let map = input[2..]
        .iter()
        .map(|x| {
            let mut parts = x.split(" = ");
            let key = parts.next().unwrap();
            let value = parts
                .next()
                .unwrap()
                .trim_start_matches("(")
                .trim_end_matches(")")
                .split(", ")
                .collect::<Vec<&str>>();
            (key, value)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut index = 0;
    let mut key = "AAA";
    loop {
        if key == "ZZZ" {
            break;
        }

        key = match commands[index % commands.len()] {
            'L' => map[key][0],
            'R' => map[key][1],
            _ => bail!("Unknown command"),
        };

        index += 1;
    }

    Ok(index as i32)
}

fn process2(input: Vec<String>) -> Result<i64> {
    let commands = input[0].chars().collect::<Vec<char>>();
    let map = input[2..]
        .iter()
        .map(|x| {
            let mut parts = x.split(" = ");
            let key = parts.next().unwrap();
            let value = parts
                .next()
                .unwrap()
                .trim_start_matches("(")
                .trim_end_matches(")")
                .split(", ")
                .collect::<Vec<&str>>();
            (key, value)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let keys = map
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| *x)
        .collect::<Vec<&str>>();

    let mut res: Option<i64> = None;

    for mut key in keys {
        let mut index = 0;
        loop {
            if key.ends_with("Z") {
                break;
            }

            key = match commands[index % commands.len()] {
                'L' => map[key][0],
                'R' => map[key][1],
                _ => bail!("Unknown command"),
            };

            index += 1;
        }

        match res {
            None => res = Some(index as i64),
            Some(x) => res = Some(lcm(x, index as i64)),
        }
    }

    match res {
        None => bail!("No result"),
        Some(x) => Ok(x),
    }
}

pub fn run() {
    let res = process(lines_from_file("./input/day08.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process2(lines_from_file("./input/day08.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day08::process;
    use crate::day08::process2;

    #[test]
    fn test1() {
        let input = vec![
            "LLR".to_string(),
            "".into(),
            "AAA = (BBB, BBB)".into(),
            "BBB = (AAA, ZZZ)".into(),
            "ZZZ = (ZZZ, ZZZ)".into(),
        ];
        assert!(process(input).unwrap() == 6);
    }

    #[test]
    fn test2() {
        let input = vec![
            "LR".to_string(),
            "".into(),
            "11A = (11B, XXX)".into(),
            "11B = (XXX, 11Z)".into(),
            "11Z = (11B, XXX)".into(),
            "22A = (22B, XXX)".into(),
            "22B = (22C, 22C)".into(),
            "22C = (22Z, 22Z)".into(),
            "22Z = (22B, 22B)".into(),
            "XXX = (XXX, XXX)".into(),
        ];
        assert!(process2(input).unwrap() == 6);
    }
}
