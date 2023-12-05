use std::collections::{HashSet, HashMap};
use scan_fmt::scan_fmt;
// Vector of Rows of (numbers, symbols)
type Input = (Vec<i64>, Vec<Vec<(i64, i64, i64)>>);


fn parse_map(input: &str) -> Vec<(i64, i64, i64)> {
    let mut output = vec![];
    for line in input.lines() {
        if line == "" {
            break;
        }
        let mut temp: Vec<_> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
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
    let seeds: Vec<_> = seeds_line.split(' ').skip(1).map(|x| x.parse::<i64>().unwrap()).collect();
    //println!("seeds: {:?}", seeds);

    let mut sections = input.split(':');
    let asdf = sections.clone().skip(2).next().unwrap();
    let asdf = asdf.trim_start();
    let mut seed_to_soil: Vec<(i64, i64, i64)> = parse_map(asdf);
    //println!("seed_to_soil: {:?}", seed_to_soil);
    output.push(seed_to_soil);

    let asdf = sections.clone().skip(3).next().unwrap();
    let asdf = asdf.trim_start();
    let mut soil_to_fert = parse_map(asdf);
    //println!("soil_to_fert: {:?}", soil_to_fert);
    output.push(soil_to_fert);

    let asdf = sections.clone().skip(4).next().unwrap();
    let asdf = asdf.trim_start();
    let mut fert_to_water = parse_map(asdf);
    //println!("fert_to_water: {:?}", fert_to_water);
    output.push(fert_to_water);

    let asdf = sections.clone().skip(5).next().unwrap();
    let asdf = asdf.trim_start();
    let mut water_to_light = parse_map(asdf);
    //println!("water_to_light: {:?}", water_to_light);
    output.push(water_to_light);

    let asdf = sections.clone().skip(6).next().unwrap();
    let asdf = asdf.trim_start();
    let mut light_to_temp = parse_map(asdf);
    //println!("light_to_temp: {:?}", light_to_temp);
    output.push(light_to_temp);

    let asdf = sections.clone().skip(7).next().unwrap();
    let asdf = asdf.trim_start();
    let mut temp_to_humid = parse_map(asdf);
    //println!("temp_to_humid: {:?}", temp_to_humid);
    output.push(temp_to_humid);

    let asdf = sections.clone().skip(8).next().unwrap();
    let asdf = asdf.trim_start();
    let mut humid_to_loc = parse_map(asdf);
    //println!("humid_to_loc: {:?}", humid_to_loc);
    output.push(humid_to_loc);

    (seeds, output)
}

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
    let mut locs: Vec<_> = seeds.iter().map(|&x| forward(x, &rules)).collect();
    *locs.iter().min().unwrap()
}

fn backward(seed: i64, rules: &[Vec<(i64, i64, i64)>]) -> i64 {
    let mut nseed = seed;

    // Layers of conversions
    for _conv in rules {

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
pub fn part2(input: &Input) -> i64 {
    let (seeds, rules) = input;

    let mut seed_pairs: Vec<(i64, i64)> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        seed_pairs.push((seeds[i], seeds[i+1]));
        i += 2;
    }
    println!("seed_pairs: {:?}", seed_pairs);

    // Get crit points go backwards
    let rev_rules: Vec<_> = rules.iter().cloned().rev().collect();
    let mut rev_seeds: Vec<_> = rev_rules[0].iter().map(|x| (x.0, x.0 + x.2)).collect();
    println!("rev_seeds: {:?}", rev_seeds);

    // Critical seeds
    let mut crits = vec![];
    let l = rules.len();
    for (i, rule) in rules.iter().enumerate() {
        for pair in rule {
            println!("l: {}", l);
            println!("i: {}", i);
            println!("rev_rules.len(): {}", rev_rules.len());
            crits.push(backward(pair.0, &rev_rules[l - (i+1)..]));
            crits.push(backward(pair.0 + pair.2, &rev_rules[l - (i+1)..]));
        }
    }

    println!("crits: {:?}", crits);

    // Critical locations
    let mut crit_locs = vec![];
    let l = rules.len();
    for (i, rule) in rev_rules.iter().enumerate() {
        for pair in rule {
            crit_locs.push(forward(pair.0, &rules[l - i..]));
            crit_locs.push(forward(pair.0 + pair.2, &rules[l - i..]));
        }
    }
    println!("crit_locs: {:?}", crit_locs);

    let mut minidx = 0;
    let mut minloc = crit_locs[0];
    for i in 1..crit_locs.len() {
        if crit_locs[i] < minloc {
            minloc = crit_locs[i];
            minidx = i;
        }
    }

    crits[minidx]
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
