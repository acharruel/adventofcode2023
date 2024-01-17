use anyhow::Result;
use num::abs;

use crate::lines_from_file;

fn line_is_empty(line: &str) -> bool {
    line.chars().all(|c| c == '.')
}

fn col_is_empty(input: &Vec<String>, col: i32) -> bool {
    input
        .iter()
        .all(|line| line.chars().nth(col as usize).unwrap() == '.')
}

fn process_distance(
    (x1, y1): (i32, i32),
    (x2, y2): (i32, i32),
    empty_lines: &[i32],
    empty_cols: &[i32],
    incr: i64,
) -> Result<i64> {
    let mut dist = y2 as i64 - y1 as i64 + abs(x2 as i64 - x1 as i64);

    for i in y1..y2 {
        if empty_lines.contains(&i) {
            dist += incr;
        }
    }

    for i in x1..x2 {
        if empty_cols.contains(&i) {
            dist += incr;
        }
    }

    for i in x2..x1 {
        if empty_cols.contains(&i) {
            dist += incr;
        }
    }

    Ok(dist)
}

fn process(input: Vec<String>, incr: i64) -> Result<i64> {
    let galaxies: Vec<(i32, i32)> = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let empty_lines: Vec<i32> = input
        .iter()
        .enumerate()
        .filter(|(_, line)| line_is_empty(line))
        .map(|(y, _)| y as i32)
        .collect();

    let empty_cols: Vec<i32> = (0..input[0].len() as i32)
        .filter(|col| col_is_empty(&input, *col))
        .collect();

    let mut res = 0;
    for i in 0..galaxies.len() {
        let (x1, y1) = galaxies[i];
        for (x2, y2) in galaxies.iter().skip(i + 1) {
            res += process_distance((x1, y1), (*x2, *y2), &empty_lines, &empty_cols, incr)?;
        }
    }

    Ok(res)
}

pub fn run() {
    let res = process(lines_from_file("./input/day11.txt"), 1);
    println!(" * {:?}", res.unwrap());
    let res = process(lines_from_file("./input/day11.txt"), 999999);
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day11::process;

    #[test]
    fn test1() {
        let input = vec![
            "...#......".to_string(),
            ".......#..".into(),
            "#.........".into(),
            "..........".into(),
            "......#...".into(),
            ".#........".into(),
            ".........#".into(),
            "..........".into(),
            ".......#..".into(),
            "#...#.....".into(),
        ];
        assert!(process(input, 1).unwrap() == 374);
    }

    #[test]
    fn test2() {
        let input = vec![
            "...#......".to_string(),
            ".......#..".into(),
            "#.........".into(),
            "..........".into(),
            "......#...".into(),
            ".#........".into(),
            ".........#".into(),
            "..........".into(),
            ".......#..".into(),
            "#...#.....".into(),
        ];
        assert!(process(input, 99).unwrap() == 8410);
    }
}
