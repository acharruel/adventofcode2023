use std::{cmp::Ordering, collections::HashMap};

use anyhow::{bail, Result};

use crate::lines_from_file;

fn find_hand_type(hand: &String) -> Result<i32> {
    let mut cards: HashMap<char, i32> = HashMap::new();
    for c in hand.chars() {
        if cards.get(&c).is_some() {
            cards.insert(c, cards.get(&c).unwrap() + 1);
        } else {
            cards.insert(c, 1);
        }
    }

    match cards.len() {
        5 => Ok(0),
        4 => Ok(1),
        3 => {
            if cards.values().any(|&x| x == 3) {
                Ok(3)
            } else {
                Ok(2)
            }
        }
        2 => {
            if cards.values().any(|&x| x == 4) {
                Ok(5)
            } else {
                Ok(4)
            }
        }
        1 => Ok(6),
        _ => bail!("No matching hand found"),
    }
}

fn compare_hands(hand1: &String, hand2: &String) -> Result<Ordering> {
    let type1 = find_hand_type(hand1)?;
    let type2 = find_hand_type(hand2)?;
    if type1 != type2 {
        return Ok(type1.cmp(&type2));
    }

    let a: Vec<i32> = hand1
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as i32,
        })
        .collect();
    let b: Vec<i32> = hand2
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as i32,
        })
        .collect();
    for i in 0..a.len() {
        if a[i] > b[i] {
            return Ok(Ordering::Greater);
        } else if a[i] < b[i] {
            return Ok(Ordering::Less);
        }
    }
    Ok(Ordering::Equal)
}

fn process(input: Vec<String>) -> Result<i32> {
    let mut hands: Vec<(String, i32)> = Vec::new();
    for line in input {
        let cards = line.split_whitespace().collect::<Vec<&str>>()[0].to_string();
        let bet = line.split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        hands.push((cards, bet));
    }

    hands.sort_by(|a, b| compare_hands(&a.0, &b.0).unwrap());

    let mut total = 0;
    let mut index = 1;

    hands.iter().for_each(|(_, x)| {
        total += x * index;
        index += 1;
    });

    Ok(total)
}

fn improve_hand(hand: &String) -> Result<String> {
    // special case when hand is full of jokers
    if hand == "JJJJJ" {
        return Ok("AAAAA".to_string());
    }

    let mut map: HashMap<char, i32> = HashMap::new();
    for c in hand.chars() {
        if map.get(&c).is_some() {
            map.insert(c, map.get(&c).unwrap() + 1);
        } else {
            map.insert(c, 1);
        }
    }

    if !map.contains_key(&'J') {
        return Ok(hand.to_string());
    }

    // If we have a joker and two pairs, we don't know which pair to improve
    // This not handled here, but we are lucky!
    map.remove(&'J');
    let c = map.iter().max_by_key(|(_, &v)| v).unwrap().0;
    Ok(hand.replace("J", &c.to_string()))
}

fn compare_hands_with_joker(hand1: &String, hand2: &String) -> Result<Ordering> {
    // check improvement using joker
    let improved_hand1 = improve_hand(hand1)?;
    let improved_hand2 = improve_hand(hand2)?;

    let type1 = find_hand_type(&improved_hand1)?;
    let type2 = find_hand_type(&improved_hand2)?;
    if type1 != type2 {
        return Ok(type1.cmp(&type2));
    }

    let a: Vec<i32> = hand1
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as i32,
        })
        .collect();
    let b: Vec<i32> = hand2
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as i32,
        })
        .collect();
    for i in 0..a.len() {
        if a[i] > b[i] {
            return Ok(Ordering::Greater);
        } else if a[i] < b[i] {
            return Ok(Ordering::Less);
        }
    }
    Ok(Ordering::Equal)
}

fn process2(input: Vec<String>) -> Result<i32> {
    let mut hands: Vec<(String, i32)> = Vec::new();
    for line in input {
        let cards = line.split_whitespace().collect::<Vec<&str>>()[0].to_string();
        let bet = line.split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        hands.push((cards, bet));
    }

    hands.sort_by(|a, b| compare_hands_with_joker(&a.0, &b.0).unwrap());

    let mut total = 0;
    let mut index = 1;

    hands.iter().for_each(|(_, x)| {
        total += x * index;
        index += 1;
    });

    Ok(total)
}

pub fn run() {
    let _input = vec![
        "32T3K 765".to_string(),
        "T55J5 684".into(),
        "KK677 28".into(),
        "KTJJT 220".into(),
        "QQQJA 483".into(),
    ];
    let res = process(lines_from_file("./input/day07.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process2(lines_from_file("./input/day07.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day07::process;
    use crate::day07::process2;

    #[test]
    fn test1() {
        let input = vec![
            "32T3K 765".to_string(),
            "T55J5 684".into(),
            "KK677 28".into(),
            "KTJJT 220".into(),
            "QQQJA 483".into(),
        ];
        assert!(process(input).unwrap() == 6440);
    }

    #[test]
    fn test2() {
        let input = vec![
            "32T3K 765".to_string(),
            "T55J5 684".into(),
            "KK677 28".into(),
            "KTJJT 220".into(),
            "QQQJA 483".into(),
        ];
        assert!(process2(input).unwrap() == 5905);
    }
}
