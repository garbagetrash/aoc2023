// Vector of Rows of (numbers, symbols)
type Input = Vec<(Vec<i64>, Vec<i64>)>;

#[aoc_generator(day4)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let mut liter = line.split(':').nth(1).unwrap().split('|');
        let winning = liter.next().unwrap();
        let mine = liter.next().unwrap();
        let mut _winning = vec![];
        for x in winning.split(' ') {
            if let Ok(num) = x.parse::<i64>() {
                _winning.push(num);
            }
        }
        let mut _mine = vec![];
        for x in mine.split(' ') {
            if let Ok(num) = x.parse::<i64>() {
                _mine.push(num);
            }
        }

        output.push((_winning, _mine));
    }

    output
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut total = 0;
    for card in input {
        let mut cnt = 0;
        for num in &card.1 {
            if card.0.contains(num) {
                cnt += 1;
            }
        }
        if cnt > 0 {
            total += 2_i64.pow(cnt - 1);
        }
    }
    total
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut copies = vec![1; input.len()];
    for (i, card) in input.iter().enumerate() {
        let mut cnt = 0;
        for num in &card.1 {
            if card.0.contains(num) {
                cnt += 1;
            }
        }

        for j in 0..cnt {
            copies[i + j + 1] += copies[i];
        }
    }
    copies.iter().sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/04a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/04a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 30);
    }
}
