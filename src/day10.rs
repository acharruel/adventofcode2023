use std::task::Wake;

use anyhow::{bail, Result};
use multimap::MultiMap;

use crate::lines_from_file;

fn find_starting_point(input: &Vec<String>) -> Result<(usize, usize)> {
    let mut x = 0;
    let mut y = 0;
    for (i, line) in input.iter().enumerate() {
        if let Some(pos) = line.find('S') {
            x = pos;
            y = i;
        }
    }
    Ok((x, y))
}

fn bfs(graph: &MultiMap<usize, usize>, root: usize) -> Vec<usize> {
    let mut visited: Vec<usize> = Vec::new();
    let mut queue: Vec<usize> = Vec::new();

    queue.push(root);
    while !queue.is_empty() {
        let node = queue.remove(0);
        if !visited.contains(&node) {
            visited.push(node);
            if let Some(neighbors) = graph.get_vec(&node) {
                for neighbor in neighbors {
                    queue.push(*neighbor);
                }
            }
        }
    }
    visited
}

fn process(input: Vec<String>) -> Result<i32> {
    let nline = input.len();
    let ncol = input[0].len();
    let mut graph: MultiMap<usize, usize> = MultiMap::new();

    let Ok((x, y)) = find_starting_point(&input) else {
        bail!("No starting point found");
    };
    let root = x + y * ncol;

    for x in 0..ncol {
        for y in 0..nline {
            match input[y].chars().nth(x).unwrap() {
                '.' => {}
                'S' => {
                    if x < ncol - 1 && input[y].chars().nth(x + 1).unwrap() != '.' {
                        graph.insert(x + y * ncol, x + 1 + y * ncol);
                    }
                    if x > 0 && input[y].chars().nth(x - 1).unwrap() != '.' {
                        graph.insert(x + y * ncol, x - 1 + y * ncol);
                    }
                    if y > 0 && input[y - 1].chars().nth(x).unwrap() != '.' {
                        graph.insert(x + y * ncol, x + (y - 1) * ncol);
                    }
                    if y < nline - 1 && input[y + 1].chars().nth(x).unwrap() != '.' {
                        graph.insert(x + y * ncol, x + (y + 1) * ncol);
                    }
                }
                'J' => {
                    if x > 0 {
                        graph.insert(x + y * ncol, x - 1 + y * ncol);
                    }
                    if y > 0 {
                        graph.insert(x + y * ncol, x + (y - 1) * ncol);
                    }
                }
                'F' => {
                    if x < ncol - 1 {
                        graph.insert(x + y * ncol, x + 1 + y * ncol);
                    }
                    if y < nline - 1 {
                        graph.insert(x + y * ncol, x + (y + 1) * ncol);
                    }
                }
                '7' => {
                    if x > 0 {
                        graph.insert(x + y * ncol, x - 1 + y * ncol);
                    }
                    if y < nline - 1 {
                        graph.insert(x + y * ncol, x + (y + 1) * ncol);
                    }
                }
                'L' => {
                    if x < ncol - 1 {
                        graph.insert(x + y * ncol, x + 1 + y * ncol);
                    }
                    if y > 0 {
                        graph.insert(x + y * ncol, x + (y - 1) * ncol);
                    }
                }
                '|' => {
                    if y > 0 {
                        graph.insert(x + y * ncol, x + (y - 1) * ncol);
                    }
                    if y < nline - 1 {
                        graph.insert(x + y * ncol, x + (y + 1) * ncol);
                    }
                }
                '-' => {
                    if x > 0 {
                        graph.insert(x + y * ncol, x - 1 + y * ncol);
                    }
                    if x < ncol - 1 {
                        graph.insert(x + y * ncol, x + 1 + y * ncol);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(bfs(&graph, root).len() as i32 / 2)
}

pub fn run() {
    let input = vec![
        "7-F7-".to_string(),
        ".FJ|7".into(),
        "SJLL7".into(),
        "|F--J".into(),
        "LJ.LJ".into(),
    ];
    // let res = process(input);
    let res = process(lines_from_file("./input/day10.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day10::process;

    #[test]
    fn test1() {
        let input = vec![
            "..F7.".to_string(),
            ".FJ|.".into(),
            "SJ.L7".into(),
            "|F--J".into(),
            "LJ...".into(),
        ];
        assert!(process(input).unwrap() == 8);
    }

    #[test]
    fn test2() {
        let input = vec![
            "7-F7-".to_string(),
            ".FJ|7".into(),
            "SJLL7".into(),
            "|F--J".into(),
            "LJ.LJ".into(),
        ];
        assert!(process(input).unwrap() == 8);
    }
}
