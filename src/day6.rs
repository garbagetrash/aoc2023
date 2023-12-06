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
pub fn part2(input: &Input) -> i64 {
    let time = input
        .iter()
        .cloned()
        .map(|x| x.0.to_string())
        .fold(String::new(), |acc, x| acc + &x)
        .parse::<i64>()
        .unwrap();
    let dist = input
        .iter()
        .cloned()
        .map(|x| x.1.to_string())
        .fold(String::new(), |acc, x| acc + &x)
        .parse::<i64>()
        .unwrap();

    // d = (t - v) * v
    // 0 <= v <= t
    // 0 = -v^2 + tv - d
    // Solve for roots, take the |r0 - r1| since function is concave
    // roots = (-b +/- sqrt{b^2 - 4ac}) / 2a
    // a = -1, b = t, c = -d

    let ac4 = 4 * dist;
    let root1 = (-time as f64 + ((time * time - ac4) as f64).sqrt()) / -2.0;
    let root2 = (-time as f64 - ((time * time - ac4) as f64).sqrt()) / -2.0;
    (root1.ceil() as i64 - root2.ceil() as i64).abs()
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
