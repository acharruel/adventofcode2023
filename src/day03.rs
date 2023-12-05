use std::cmp;

use atoi::FromRadix10;

use anyhow::Result;
use multimap::MultiMap;

use crate::lines_from_file;

fn is_adjacent_to_symbol(
    prev: Option<&String>,
    cur: Option<&String>,
    next: Option<&String>,
    pos: i32,
    len: i32,
) -> Option<(char, i32, i32)> {
    match prev {
        Some(line) => {
            let min = cmp::max(pos - 1, 0) as usize;
            let max = cmp::min(pos + len + 1, line.len() as i32) as usize;
            for (i, c) in line[min..max].chars().enumerate() {
                if !c.is_digit(10) && c != '.' {
                    return Some((c, (i + min) as i32, -1));
                }
            }
        }
        None => (),
    }

    match next {
        Some(line) => {
            let min = cmp::max(pos - 1, 0) as usize;
            let max = cmp::min(pos + len + 1, line.len() as i32) as usize;
            for (i, c) in line[min..max].chars().enumerate() {
                if !c.is_digit(10) && c != '.' {
                    return Some((c, (i + min) as i32, 1));
                }
            }
        }
        None => (),
    }

    match cur {
        Some(line) => {
            if pos > 0 {
                let c = line.chars().nth(pos as usize - 1).unwrap();
                if !c.is_digit(10) && c != '.' {
                    return Some((c, pos - 1, 0));
                }
            }
            if pos + len < line.len() as i32 {
                let c = line.chars().nth((pos + len) as usize).unwrap();
                if !c.is_digit(10) && c != '.' {
                    return Some((c, pos + len, 0));
                }
            }
        }
        None => (),
    }

    return None;
}

fn find_engine_part(
    engine_map: &mut MultiMap<String, i32>,
    prev_line: Option<&String>,
    cur_line: Option<&String>,
    next_line: Option<&String>,
    nline: i32,
) -> Result<()> {
    let mut i = 0;
    loop {
        let slice = &cur_line.unwrap()[i..];
        let (num, len) = i32::from_radix_10(slice.as_bytes());
        if len > 0 {
            match is_adjacent_to_symbol(prev_line, cur_line, next_line, i as i32, len as i32) {
                Some((c, p, m)) => engine_map.insert(format!("{} {} {}", c, nline + m, p), num),
                None => (),
            };
            i += len;
        } else {
            i += 1;
        }

        if i >= cur_line.unwrap().len() {
            break;
        }
    }
    Ok(())
}

fn parse_engine(input: Vec<String>) -> Result<MultiMap<String, i32>> {
    let mut engine_map: MultiMap<String, i32> = MultiMap::new();

    let mut line_idx = 0;

    // first line
    find_engine_part(&mut engine_map, None, Some(&input[0]), Some(&input[1]), 0)?;

    // main loop
    for lines in input[..].windows(3) {
        find_engine_part(
            &mut engine_map,
            Some(&lines[0]),
            Some(&lines[1]),
            Some(&lines[2]),
            line_idx + 1,
        )?;
        line_idx += 1;
    }

    // last line
    find_engine_part(
        &mut engine_map,
        Some(&input[line_idx as usize]),
        Some(&input[line_idx as usize + 1]),
        None,
        line_idx + 1,
    )?;

    Ok(engine_map)
}

fn process1(input: Vec<String>) -> Result<i32> {
    let engine_map = parse_engine(input)?;
    let mut add = 0;
    for (_, v) in engine_map.iter_all() {
        add += v.iter().sum::<i32>();
    }
    Ok(add)
}

fn process2(input: Vec<String>) -> Result<i32> {
    let engine_map = parse_engine(input)?;
    let mut gear = 0;
    for (k, v) in engine_map.iter_all() {
        if k.contains('*') && v.len() == 2 {
            gear += v.iter().product::<i32>();
        }
    }
    Ok(gear)
}

pub fn run() {
    let res = process1(lines_from_file("./input/day03.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process2(lines_from_file("./input/day03.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day03::process1;
    use crate::day03::process2;

    #[test]
    fn test1() {
        let input = vec![
            "467..114..".to_string(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];
        assert!(process1(input).unwrap() == 4361);
    }

    #[test]
    fn test2() {
        let input = vec![
            "467..114..".to_string(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];
        assert!(process2(input).unwrap() == 467835);
    }
}
