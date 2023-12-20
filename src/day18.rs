use rayon::prelude::*;
use std::collections::HashMap;

pub type Input = Vec<(char, usize, String)>;

#[aoc_generator(day18)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let temp: Vec<_> = line
            .split(' ')
            .collect();
        let dir = temp[0].chars().next().unwrap();
        let len = temp[1].parse::<usize>().unwrap();
        let hash = temp[2].to_string();
        output.push((dir, len, hash));
    }
    output
}

pub fn create_trench(input: &Input) -> HashMap<(i64, i64), char> {
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    let mut pos = (0, 0);
    map.insert(pos, '#');
    for (dir, len, _hex) in input {
        match dir {
            'U' => {
                for _ in 0..*len {
                    pos.1 -= 1;
                    map.insert(pos, '#');
                }
            },
            'R' => {
                for _ in 0..*len {
                    pos.0 += 1;
                    map.insert(pos, '#');
                }
            },
            'D' => {
                for _ in 0..*len {
                    pos.1 += 1;
                    map.insert(pos, '#');
                }
            },
            'L' => {
                for _ in 0..*len {
                    pos.0 -= 1;
                    map.insert(pos, '#');
                }
           },
            _ => (),
        }
    }
    map
}

pub fn fill_trench(map: &mut HashMap<(i64, i64), char>) {
    let xmin: i64 = *map.keys().map(|(x, _)| x).min().unwrap();
    let ymin: i64 = *map.keys().map(|(_, y)| y).min().unwrap();
    let xmax: i64 = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax: i64 = *map.keys().map(|(_, y)| y).max().unwrap();

    for x in xmin..xmax + 1 {
        let mut toggle = 0;
        for y in ymin..ymax + 1 {
            if map.get(&(x, y)).is_some() {
                toggle ^= 1;
            }
            if toggle == 1 {
                map.insert((x, y), '#');
            }
        }
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &Input) -> usize {
    let map = create_trench(input);
    print_map(&map);
    map.iter().count()
}

#[aoc(day18, part2)]
pub fn part2(input: &Input) -> usize {
    0
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(i64, i64), char>) {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..ymax + 1 {
        for x in 0..121 {
            if let Some(c) = map.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/18a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/18a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 94);
    }
}
