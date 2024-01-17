use anyhow::Result;

use crate::lines_from_file;

#[derive(Debug)]
struct MyMap {
    base: i64,
    range: i64,
    dest: i64,
}

#[derive(Clone, Debug)]
struct Segment {
    base: i64,
    range: i64,
}

fn build_map(input: &Vec<String>) -> Result<Vec<MyMap>> {
    let mut maps = Vec::new();
    for lines in input {
        if lines.eq("") {
            break;
        }
        let v = lines
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        maps.push(MyMap {
            base: v[1],
            range: v[2],
            dest: v[0],
        });
    }

    Ok(maps)
}

fn process_seeds(src: Vec<i64>, maps: &Vec<MyMap>) -> Result<Vec<i64>> {
    let mut new_seeds = Vec::new();

    for s in src {
        let mut found = false;
        for map in maps {
            if s >= map.base && s <= map.base + map.range {
                new_seeds.push(map.dest + s - map.base);
                found = true;
            }
        }
        if !found {
            new_seeds.push(s);
        }
    }
    Ok(new_seeds)
}

fn process(input: &Vec<String>) -> Result<i64> {
    let mut seeds = Vec::new();
    for (line_index, lines) in input.iter().enumerate() {
        if lines.contains("seeds:") {
            seeds = lines.split(':').collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        }

        if lines.contains("map:") {
            let maps = build_map(&input[line_index + 1..].to_vec())?;
            seeds = process_seeds(seeds, &maps)?;
        }
    }

    Ok(*seeds.iter().min().unwrap())
}

fn process_map(segments: Vec<Segment>, maps_list: &[Vec<MyMap>]) -> Result<i64> {
    let mut new_segments: Vec<Segment> = Vec::new();

    // walk through all maps to update list of segments
    if maps_list.is_empty() {
        let min = segments.iter().min_by_key(|x| x.base).unwrap();
        return Ok(min.base);
    }

    for s in segments {
        let mut found = false;
        for m in &maps_list[0] {
            if found {
                break;
            }

            if s.base >= m.base && s.base + s.range <= m.base + m.range {
                // segment entirely included in map
                new_segments.push(Segment {
                    base: m.dest + s.base - m.base,
                    range: s.range,
                });
                found = true;
                continue;
            }

            if s.base < m.base && s.base + s.range > m.base + m.range {
                // segment entirely overlapping map
                new_segments.push(Segment {
                    base: s.base,
                    range: m.base - s.base,
                });
                new_segments.push(Segment {
                    base: m.dest,
                    range: m.range,
                });
                new_segments.push(Segment {
                    base: m.base + m.range,
                    range: s.range - m.base + s.base - m.range,
                });
                found = true;
                continue;
            }

            if s.base < m.base && s.base + s.range > m.base && s.base + s.range < m.base + m.range {
                // segment partially overlapping map
                new_segments.push(Segment {
                    base: s.base,
                    range: m.base - s.base,
                });
                new_segments.push(Segment {
                    base: m.dest,
                    range: s.base + s.range - m.base,
                });
                found = true;
                continue;
            }

            if s.base > m.base && s.base < m.base + m.range && s.base + s.range > m.base + m.range {
                // segment partially overlapping map
                new_segments.push(Segment {
                    base: m.dest + s.base - m.base,
                    range: m.base + m.range - s.base,
                });
                new_segments.push(Segment {
                    base: m.base + m.range,
                    range: s.base + s.range - m.base - m.range,
                });
                found = true;
                continue;
            }
        }
        if !found {
            new_segments.push(s);
        }
    }

    process_map(new_segments, &maps_list[1..])
}

fn process2(input: &Vec<String>) -> Result<i64> {
    let mut seeds = Vec::new();
    let mut all_maps = Vec::new();
    for (line_index, lines) in input.iter().enumerate() {
        if lines.contains("seeds:") {
            seeds = lines.split(':').collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        }

        if lines.contains("map:") {
            all_maps.push(build_map(&input[line_index + 1..].to_vec())?);
        }
    }

    let segments = seeds
        .chunks(2)
        .map(|x| Segment {
            base: x[0],
            range: x[1],
        })
        .collect::<Vec<Segment>>();

    process_map(segments, &all_maps)
}

pub fn run() {
    let res = process(&lines_from_file("./input/day05.txt"));
    println!(" * {:?}", res.unwrap());
    let res = process2(&lines_from_file("./input/day05.txt"));
    println!(" * {:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day05::process;
    use crate::day05::process2;

    #[test]
    fn test1() {
        let input = vec![
            "seeds: 79 14 55 13".to_string(),
            "".into(),
            "seed-to-soil map:".into(),
            "50 98 2".into(),
            "52 50 48".into(),
            "".into(),
            "soil-to-fertilizer map:".into(),
            "0 15 37".into(),
            "37 52 2".into(),
            "39 0 15".into(),
            "".into(),
            "fertilizer-to-water map:".into(),
            "49 53 8".into(),
            "0 11 42".into(),
            "42 0 7".into(),
            "57 7 4".into(),
            "".into(),
            "water-to-light map:".into(),
            "88 18 7".into(),
            "18 25 70".into(),
            "".into(),
            "light-to-temperature map:".into(),
            "45 77 23".into(),
            "81 45 19".into(),
            "68 64 13".into(),
            "".into(),
            "temperature-to-humidity map:".into(),
            "0 69 1".into(),
            "1 0 69".into(),
            "".into(),
            "humidity-to-location map:".into(),
            "60 56 37".into(),
            "56 93 4".into(),
        ];
        assert!(process(&input).unwrap() == 35);
    }

    #[test]
    fn test2() {
        let input = vec![
            "seeds: 79 14 55 13".to_string(),
            "".into(),
            "seed-to-soil map:".into(),
            "50 98 2".into(),
            "52 50 48".into(),
            "".into(),
            "soil-to-fertilizer map:".into(),
            "0 15 37".into(),
            "37 52 2".into(),
            "39 0 15".into(),
            "".into(),
            "fertilizer-to-water map:".into(),
            "49 53 8".into(),
            "0 11 42".into(),
            "42 0 7".into(),
            "57 7 4".into(),
            "".into(),
            "water-to-light map:".into(),
            "88 18 7".into(),
            "18 25 70".into(),
            "".into(),
            "light-to-temperature map:".into(),
            "45 77 23".into(),
            "81 45 19".into(),
            "68 64 13".into(),
            "".into(),
            "temperature-to-humidity map:".into(),
            "0 69 1".into(),
            "1 0 69".into(),
            "".into(),
            "humidity-to-location map:".into(),
            "60 56 37".into(),
            "56 93 4".into(),
        ];
        assert!(process2(&input).unwrap() == 46);
    }
}
