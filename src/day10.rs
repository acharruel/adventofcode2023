use anyhow::{bail, Result};
use multimap::MultiMap;

use crate::lines_from_file;

fn build_graph(input: &Vec<String>, graph: &mut MultiMap<usize, usize>) -> Result<()> {
    let nline = input.len();
    let ncol = input[0].len();

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

    Ok(())
}

fn find_starting_point(input: &Vec<String>) -> Result<usize> {
    let ncol = input[0].len();
    let mut x = 0;
    let mut y = 0;
    for (i, line) in input.iter().enumerate() {
        if let Some(pos) = line.find('S') {
            x = pos;
            y = i;
        }
    }
    Ok(x + y * ncol)
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
    let mut graph: MultiMap<usize, usize> = MultiMap::new();

    build_graph(&input, &mut graph)?;

    let Ok(root) = find_starting_point(&input) else {
        bail!("No starting point found");
    };

    Ok(bfs(&graph, root).len() as i32 / 2)
}

fn ray_casting(input: &Vec<String>, x: usize, y: usize, visited: &Vec<usize>) -> Result<i32> {
    let ncol = input[0].len();
    let mut count = 0;
    if x == 0 || x == ncol - 1 {
        return Ok(0);
    }
    if visited.contains(&(x + y * ncol)) {
        return Ok(0);
    }
    for i in 0..x {
        if !visited.contains(&(i + y * ncol)) {
            continue;
        }
        if input[y].chars().nth(i).unwrap() == '|' {
            count += 1;
        }
        if input[y].chars().nth(i).unwrap() == '7' {
            count += 1;
        }
        if input[y].chars().nth(i).unwrap() == 'F' {
            count += 1;
        }
    }
    Ok(count)
}

fn process2(input: Vec<String>) -> Result<i32> {
    let nline = input.len();
    let ncol = input[0].len();
    let mut graph: MultiMap<usize, usize> = MultiMap::new();
    let mut sum = 0;

    build_graph(&input, &mut graph)?;
    let Ok(root) = find_starting_point(&input) else {
        bail!("No starting point found");
    };
    let visited = bfs(&graph, root);

    for x in 0..ncol {
        for y in 0..nline {
            if visited.contains(&(x + y * ncol)) {
                continue;
            }
            if ray_casting(&input, x, y, &visited)? % 2 == 1 {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

pub fn run() {
    let res = process(lines_from_file("./input/day10.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process2(lines_from_file("./input/day10.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day10::process;
    use crate::day10::process2;

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

    #[test]
    fn test3() {
        let input = vec![
            "...........".to_string(),
            ".S-------7.".into(),
            ".|F-----7|.".into(),
            ".||.....||.".into(),
            ".||.....||.".into(),
            ".|L-7.F-J|.".into(),
            ".|..|.|..|.".into(),
            ".L--J.L--J.".into(),
            "...........".into(),
        ];
        assert!(process2(input).unwrap() == 4);
    }

    #[test]
    fn test4() {
        let input = vec![
            ".F----7F7F7F7F-7....".to_string(),
            ".|F--7||||||||FJ....".into(),
            ".||.FJ||||||||L7....".into(),
            "FJL7L7LJLJ||LJ.L-7..".into(),
            "L--J.L7...LJS7F-7L7.".into(),
            "....F-J..F7FJ|L7L7L7".into(),
            "....L7.F7||L7|.L7L7|".into(),
            ".....|FJLJ|FJ|F7|.LJ".into(),
            "....FJL-7.||.||||...".into(),
            "....L---J.LJ.LJLJ...".into(),
        ];
        assert!(process2(input).unwrap() == 8);
    }

    #[test]
    fn test5() {
        let input = vec![
            "FF7FSF7F7F7F7F7F---7".to_string(),
            "L|LJ||||||||||||F--J".into(),
            "FL-7LJLJ||||||LJL-77".into(),
            "F--JF--7||LJLJ7F7FJ-".into(),
            "L---JF-JLJ.||-FJLJJ7".into(),
            "|F|F-JF---7F7-L7L|7|".into(),
            "|FFJF7L7F-JF7|JL---7".into(),
            "7-L-JL7||F7|L7F-7F7|".into(),
            "L.L7LFJ|||||FJL7||LJ".into(),
            "L7JLJL-JLJLJL--JLJ.L".into(),
        ];
        assert!(process2(input).unwrap() == 10);
    }
}
