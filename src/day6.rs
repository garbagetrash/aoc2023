// Vector of Rows of (numbers, symbols)
type Input = Vec<(usize, usize)>;

#[aoc_generator(day6)]
pub fn load_input(input: &str) -> Input {
    let lines_arr: Vec<_> = input.lines().collect();
    let times: Vec<_> = lines_arr[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let dists: Vec<_> = lines_arr[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    times.into_iter().zip(dists).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> usize {
    let mut output = 1;
    for (time, dist) in input {
        // d = (t - v) * v
        // 0 <= v <= t
        let mut cnt = 0;
        for v in 0..*time {
            if (*time - v) * v > *dist {
                cnt += 1;
            }
        }
        output *= cnt;
    }
    output
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    let time = input
        .iter()
        .cloned()
        .map(|x| x.0.to_string())
        .fold(String::new(), |acc, x| acc + &x)
        .parse::<usize>()
        .unwrap();
    let dist = input
        .iter()
        .cloned()
        .map(|x| x.1.to_string())
        .fold(String::new(), |acc, x| acc + &x)
        .parse::<usize>()
        .unwrap();

    // d = (t - v) * v
    // 0 <= v <= t
    let mut cnt = 0;
    for v in 0..time {
        if (time - v) * v > dist {
            cnt += 1;
        }
    }
    cnt
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/06a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/06a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 71503);
    }
}
