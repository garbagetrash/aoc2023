use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub type Input = HashMap<(usize, usize), char>;

#[aoc_generator(day23)]
pub fn load_input(input: &str) -> Input {
    let mut output = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            output.insert((x, y), c);
        }
    }
    output
}

#[allow(dead_code)]
fn print_map(map: &Input) {
    let xmax = map.keys().map(|(x, _y)| *x).max().unwrap();
    let ymax = map.keys().map(|(_x, y)| *y).max().unwrap();
    for y in 0..ymax + 1 {
        for x in 0..xmax + 1 {
            let c = map.get(&(x, y)).unwrap();
            print!("{}", c);
        }
        println!();
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &Input) -> usize {
    print_map(input);
    0
}

#[aoc(day23, part2)]
pub fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/23a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/23a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 7);
    }
}
