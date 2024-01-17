use anyhow::Result;

use crate::lines_from_file;

fn is_vec_all_zeros(vec: &[i32]) -> bool {
    vec.iter().all(|&x| x == 0)
}

fn process_line(line: String, next_values: &mut Vec<i32>) -> Result<i32> {
    let mut vec = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    while !is_vec_all_zeros(&vec) {
        next_values.push(vec[vec.len() - 1]);
        vec = vec
            .iter()
            .zip(vec.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<i32>>();
    }

    Ok(next_values.iter().fold(0, |acc, x| acc + *x))
}

fn process(input: Vec<String>) -> Result<i32> {
    let mut res = 0;
    let mut next_values: Vec<i32> = vec![];
    for line in input {
        next_values.clear();
        res += process_line(line, &mut next_values).unwrap();
    }
    Ok(res)
}

fn process_line2(line: String, first_values: &mut Vec<i32>) -> Result<i32> {
    let mut vec = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    while !is_vec_all_zeros(&vec) {
        first_values.push(vec[0]);
        vec = vec
            .iter()
            .zip(vec.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<i32>>();
    }

    let mut res = 0;
    for x in first_values.iter().rev() {
        res = x - res;
    }

    Ok(res)
}

fn process2(input: Vec<String>) -> Result<i32> {
    let mut res = 0;
    let mut first_values: Vec<i32> = vec![];
    for line in input {
        first_values.clear();
        res += process_line2(line, &mut first_values).unwrap();
    }
    Ok(res)
}

pub fn run() {
    let res = process(lines_from_file("./input/day09.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process2(lines_from_file("./input/day09.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day09::process;
    use crate::day09::process2;

    #[test]
    fn test1() {
        let input = vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".into(),
            "10 13 16 21 30 45".into(),
        ];
        assert!(process(input).unwrap() == 114);
    }

    #[test]
    fn test2() {
        let input = vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".into(),
            "10 13 16 21 30 45".into(),
        ];
        assert!(process2(input).unwrap() == 2);
    }
}
