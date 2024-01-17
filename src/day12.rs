use std::{collections::HashMap, time::Instant};

use anyhow::Result;

use crate::lines_from_file;

fn process_spring(
    spring: String,
    group: &Vec<usize>,
    cache: &mut HashMap<(String, Vec<usize>), i64>,
) -> Result<i64> {
    if spring.is_empty() {
        return match group.is_empty() {
            true => Ok(1),
            false => Ok(0),
        };
    }

    if group.is_empty() {
        return match spring.contains('#') {
            true => Ok(0),
            false => Ok(1),
        };
    }

    let mut res = 0;
    let key = (spring.clone(), group.clone());

    if cache.contains_key(&key) {
        return Ok(cache[&key]);
    }

    if ".?".contains(spring.chars().next().unwrap()) {
        res += process_spring(spring.chars().skip(1).collect::<String>(), group, cache)?;
    }

    if "#?".contains(spring.chars().next().unwrap())
        && group[0] <= spring.len()
        && !&spring[..group[0]].contains('.')
        && (group[0] == spring.len() || spring.chars().nth(group[0]).unwrap() != '#')
    {
        res += process_spring(
            spring.chars().skip(group[0] + 1).collect::<String>(),
            &group[1..].to_vec(),
            cache,
        )?;
    }

    cache.insert(key, res);
    Ok(res)
}

fn process(input: Vec<String>) -> Result<i64> {
    let mut cache: HashMap<(String, Vec<usize>), i64> = HashMap::new();

    let springs = input
        .iter()
        .map(|line| line.split(' ').collect::<Vec<&str>>()[0])
        .collect::<Vec<&str>>();
    let groups = input
        .iter()
        .map(|line| {
            line.split(' ').collect::<Vec<&str>>()[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut total = 0;
    for it in springs.iter().zip(groups.iter()) {
        let (spring, group) = it;
        total += process_spring(spring.to_string(), group, &mut cache)?;
    }

    Ok(total)
}

fn process2(input: Vec<String>) -> Result<i64> {
    let mut cache: HashMap<(String, Vec<usize>), i64> = HashMap::new();

    let springs = input
        .iter()
        .map(|line| line.split(' ').collect::<Vec<&str>>()[0])
        .collect::<Vec<&str>>();
    let mut groups = input
        .iter()
        .map(|line| {
            line.split(' ').collect::<Vec<&str>>()[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // unfold data
    let new_springs = springs
        .iter()
        .map(|s| s.to_string() + "?" + s + "?" + s + "?" + s + "?" + s)
        .collect::<Vec<String>>();
    let groups = groups
        .iter_mut()
        .map(|g| g.repeat(5))
        .collect::<Vec<Vec<usize>>>();

    let mut total = 0;
    for it in new_springs.iter().zip(groups.iter()) {
        let (spring, group) = it;
        total += process_spring(spring.to_string(), group, &mut cache)?;
    }

    Ok(total)
}

pub fn run() {
    let start = Instant::now();
    let res = process(lines_from_file("./input/day12.txt"));
    let duration = start.elapsed();
    println!(" * {:?} (computed in {:?})", res.unwrap(), duration);

    let start = Instant::now();
    let res = process2(lines_from_file("./input/day12.txt"));
    let duration = start.elapsed();
    println!(" * {:?} (computed in {:?})", res.unwrap(), duration);
}

#[cfg(test)]
mod tests {
    use crate::day12::process;
    use crate::day12::process2;

    #[test]
    fn test1() {
        let input = vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".into(),
            "?#?#?#?#?#?#?#? 1,3,1,6".into(),
            "????.#...#... 4,1,1".into(),
            "????.######..#####. 1,6,5".into(),
            "?###???????? 3,2,1".into(),
        ];
        assert!(process(input).unwrap() == 21);
    }

    #[test]
    fn test2() {
        let input = vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".into(),
            "?#?#?#?#?#?#?#? 1,3,1,6".into(),
            "????.#...#... 4,1,1".into(),
            "????.######..#####. 1,6,5".into(),
            "?###???????? 3,2,1".into(),
        ];
        assert!(process2(input).unwrap() == 525152);
    }
}
