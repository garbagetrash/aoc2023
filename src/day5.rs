// Vector of Rows of (numbers, symbols)
type Input = (Vec<i64>, Vec<Vec<(i64, i64, i64)>>);

fn parse_map(input: &str) -> Vec<(i64, i64, i64)> {
    let mut output = vec![];
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let temp: Vec<_> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let dst = temp[0];
        let src = temp[1];
        let range = temp[2];
        output.push((dst, src, range));
    }
    output
}

#[aoc_generator(day5)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];

    let seeds_line = input.split('\n').next().unwrap();
    let seeds_line = seeds_line.split(':').nth(1).unwrap();
    let seeds: Vec<_> = seeds_line
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let sections = input.split(':');
    let asdf = sections.clone().nth(2).unwrap();
    let asdf = asdf.trim_start();
    let seed_to_soil: Vec<(i64, i64, i64)> = parse_map(asdf);
    output.push(seed_to_soil);

    let asdf = sections.clone().nth(3).unwrap();
    let asdf = asdf.trim_start();
    let soil_to_fert = parse_map(asdf);
    output.push(soil_to_fert);

    let asdf = sections.clone().nth(4).unwrap();
    let asdf = asdf.trim_start();
    let fert_to_water = parse_map(asdf);
    output.push(fert_to_water);

    let asdf = sections.clone().nth(5).unwrap();
    let asdf = asdf.trim_start();
    let water_to_light = parse_map(asdf);
    output.push(water_to_light);

    let asdf = sections.clone().nth(6).unwrap();
    let asdf = asdf.trim_start();
    let light_to_temp = parse_map(asdf);
    output.push(light_to_temp);

    let asdf = sections.clone().nth(7).unwrap();
    let asdf = asdf.trim_start();
    let temp_to_humid = parse_map(asdf);
    output.push(temp_to_humid);

    let asdf = sections.clone().nth(8).unwrap();
    let asdf = asdf.trim_start();
    let humid_to_loc = parse_map(asdf);
    output.push(humid_to_loc);

    (seeds, output)
}

/// Propagates states from direction of Seeds -> Locations.
fn forward(seed: i64, rules: &[Vec<(i64, i64, i64)>]) -> i64 {
    let mut nseed = seed;

    // Layers of conversions
    for _conv in rules {
        // For each tuple mapping in this layer
        for conv in _conv {
            if nseed >= conv.1 && nseed < conv.1 + conv.2 {
                // contains mapping
                let diff = nseed - conv.1;
                nseed = conv.0 + diff;
                break;
            }
        }
    }
    nseed
}

/*
 *
        let dst = temp[0];
        let src = temp[1];
        let range = temp[2];
*/
#[aoc(day5, part1)]
pub fn part1(input: &Input) -> i64 {
    let (seeds, rules) = input;
    seeds.iter().map(|&x| forward(x, rules)).min().unwrap()
}

/// Propagates states from direction of Locations -> Seeds.
fn backward(seed: i64, rev_rules: &[Vec<(i64, i64, i64)>]) -> i64 {
    let mut nseed = seed;

    // Layers of conversions
    for _conv in rev_rules {
        // For each tuple mapping in this layer
        for conv in _conv {
            if nseed >= conv.0 && nseed < conv.0 + conv.2 {
                // contains mapping
                let diff = nseed - conv.0;
                nseed = conv.1 + diff;
                break;
            }
        }
    }
    nseed
}

#[aoc(day5, part2)]
/// 46210551 is too low
pub fn part2(input: &Input) -> i64 {
    let (seeds, rules) = input;
    let mut seed_pairs: Vec<(i64, i64)> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        seed_pairs.push((seeds[i], seeds[i + 1]));
        i += 2;
    }

    let rev_rules: Vec<_> = rules.iter().cloned().rev().collect();

    // The min_seed is the first all-pass-through value which could work. We
    // presume the intervals cover this value, but its worth checking.
    let min_seed = seed_pairs.iter().map(|x| x.0).min().unwrap();
    println!("min_seed: {}", min_seed);

    let min_interval_value = rules.iter().map(|_x| {
        _x.iter().map(|x| {
            if x.0 < x.1 {
                x.0
            } else {
                x.1
            }
        }).min().unwrap()
    }).min().unwrap();
    println!("min_interval_value: {}", min_interval_value);

    let mut loc = 0;
    loop {
        let seed = backward(loc, &rev_rules);
        for pair in &seed_pairs {
            if seed >= pair.0 && seed < pair.0 + pair.1 {
                return loc;
            }
        }

        // This loc doesn't map back to a valid seed, try next
        loc += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/05a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/05a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 46);
    }
}
