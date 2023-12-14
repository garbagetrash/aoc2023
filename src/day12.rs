use std::collections::HashMap;

type Input = Vec<(String, Vec<usize>)>;

#[aoc_generator(day12)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let mut part_iter = line.split(' ');
        let arrangement = part_iter.next().unwrap().to_string();
        let right = part_iter.next().unwrap();
        let segments: Vec<usize> = right.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        output.push((arrangement, segments));
    }
    output
}

fn build_chunk(n: usize) -> String {
    let mut output = String::from(".");
    for _ in 0..n {
        output.push('#');
    }
    output.push('.');
    output
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> usize {
    let mut output = 0;
    for (arr, seg) in input {
        let chunks: Vec<_> = seg.iter().map(|n| build_chunk(*n)).collect();
        println!("chunks: {:?}", chunks);
        let mut aug_arr = arr.clone();
        aug_arr.insert(0, '.');
        aug_arr.push('.');
        println!("aug_arr: {}", aug_arr);
        println!();
    }
    output
}

#[aoc(day12, part2)]
pub fn part2(input: &Input) -> usize {
    0
}

#[allow(dead_code)]
fn print_map(map: &[Vec<char>]) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 136);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 64);
    }
}
