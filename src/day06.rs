use anyhow::Result;

fn beat_record(time: i64, distance: i64) -> Result<i64> {
    Ok((1..time)
        .map(|x| x * (time - x))
        .map(|x| (x > distance) as i64)
        .sum::<i64>())
}

fn process(input: &Vec<String>) -> Result<i64> {
    let mut times = Vec::new();
    let mut distances = Vec::new();

    for lines in input {
        if lines.contains("Time:") {
            times = lines.split(':').collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        }

        if lines.contains("Distance:") {
            distances = lines.split(':').collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        }
    }

    let mut mul = 1;
    for (i, time) in times.iter().enumerate() {
        mul *= beat_record(*time, distances[i])?;
    }

    Ok(mul)
}

fn process2(input: &Vec<String>) -> Result<i64> {
    let mut time = 0;
    let mut distance = 0;
    for lines in input {
        if lines.contains("Time:") {
            time = lines.split(':').collect::<Vec<&str>>()[1]
                .replace(' ', "")
                .parse::<i64>()
                .unwrap();
        }
        if lines.contains("Distance:") {
            distance = lines.split(':').collect::<Vec<&str>>()[1]
                .replace(' ', "")
                .parse::<i64>()
                .unwrap();
        }
    }
    beat_record(time, distance)
}

pub fn run() {
    let input = vec![
        "Time:        54     94     65     92".to_string(),
        "Distance:   302   1476   1029   1404".into(),
    ];
    let res = process(&input);
    println!(" * {:?}", res.unwrap());
    let res = process2(&input);
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day06::process;
    use crate::day06::process2;

    #[test]
    fn test1() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".into(),
        ];
        assert!(process(&input).unwrap() == 288);
    }

    #[test]
    fn test2() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".into(),
        ];
        assert!(process2(&input).unwrap() == 71503);
    }
}
