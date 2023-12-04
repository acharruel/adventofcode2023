use anyhow::Result;

use crate::lines_from_file;

fn update_points(points: i32) -> i32 {
    if points == 0 {
        1
    } else {
        points * 2
    }
}

fn process(input: Vec<String>) -> Result<i32> {
    let mut total = 0;
    for lines in input {
        let mut points = 0;

        let substr = &lines[lines.find(':').unwrap() + 2..];
        let parts = substr.split('|').collect::<Vec<&str>>();

        let winnings = parts[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let owns = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for n in owns {
            if winnings.contains(&n) {
                points = update_points(points);
            }
        }

        total += points;
    }

    Ok(total)
}

fn process2(input: Vec<String>) -> Result<i32> {
    let mut map: Vec<i32> = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        map.push(1);
    }

    for lines in input {
        let card_index_str = lines.split_whitespace().nth(1).unwrap();
        let card_index = card_index_str[..card_index_str.len() - 1]
            .parse::<i32>()
            .unwrap()
            - 1;

        let substr = &lines[lines.find(':').unwrap() + 2..];
        let parts = substr.split('|').collect::<Vec<&str>>();

        let winnings = parts[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let owns = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut i = 0;
        for n in owns {
            if winnings.contains(&n) {
                map[(card_index + i + 1) as usize] += map[card_index as usize];
                i += 1;
            }
        }
    }

    Ok(map.iter().sum())
}

pub fn run() {
    let res = process(lines_from_file("./input/day04.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process2(lines_from_file("./input/day04.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day04::process;
    use crate::day04::process2;

    #[test]
    fn test1() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".into(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".into(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".into(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into(),
        ];
        assert!(process(input).unwrap() == 13);
    }

    #[test]
    fn test2() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".into(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".into(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".into(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into(),
        ];
        assert!(process2(input).unwrap() == 30);
    }
}
