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

pub fn run() {
    let input = vec![
        "LLR".to_string(),
        "".into(),
        "AAA = (BBB, BBB)".into(),
        "BBB = (AAA, ZZZ)".into(),
        "ZZZ = (ZZZ, ZZZ)".into(),
    ];
    let res = process(lines_from_file("./input/day08.txt"));
    // let res = process(input);
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day08::process;

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
}
