type Input = Vec<Vec<i64>>;

#[aoc_generator(day9)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let temp = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        output.push(temp);
    }
    output
}

fn diff(x: &[i64]) -> Vec<i64> {
    let mut output = vec![];
    for i in 1..x.len() {
        output.push(x[i] - x[i - 1]);
    }
    output
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut output = vec![];
    for xarr in input {
        let mut dx_list = vec![];
        let mut dx = xarr.clone();
        let mut order = 0;
        let mut coefs: Vec<i64> = vec![];
        while dx.iter().any(|&x| x != 0) {
            dx_list.push(dx.clone());
            coefs.push(*dx.iter().last().unwrap());
            dx = diff(&dx);
            order += 1;
        }
        order -= 1;

        let ans = (0..order + 1).map(|i| coefs[order - i]).sum();
        output.push(ans);
    }
    output.iter().sum::<i64>()
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut output = vec![];
    for xarr in input {
        let mut dx_list = vec![];
        let mut dx = xarr.clone();
        let mut order = 0;
        let mut coefs: Vec<i64> = vec![];
        while dx.iter().any(|&x| x != 0) {
            dx_list.push(dx.clone());
            coefs.push(*dx.first().unwrap());
            dx = diff(&dx);
            order += 1;
        }
        order -= 1;

        let mut last = 0;
        for i in 0..order + 1 {
            let c = coefs[order - i];
            last = c - last;
        }
        let ans = last;
        output.push(ans);
    }
    output.iter().sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/09a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/09a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 2);
    }
}
