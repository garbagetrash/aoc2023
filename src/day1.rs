fn check(asdf: &str, dumb: bool) -> Option<u32> {
    //println!("asdf: {}", asdf);
    if dumb {
        if asdf.len() >= 3 && &asdf[..3] == "one" {
            return Some(1);
        } else if asdf.len() >= 3 && &asdf[..3] == "two" {
            return Some(2);
        } else if asdf.len() >= 5 && &asdf[..5] == "three" {
            return Some(3);
        } else if asdf.len() >= 4 && &asdf[..4] == "four" {
            return Some(4);
        } else if asdf.len() >= 4 && &asdf[..4] == "five" {
            return Some(5);
        } else if asdf.len() >= 3 && &asdf[..3] == "six" {
            return Some(6);
        } else if asdf.len() >= 5 && &asdf[..5] == "seven" {
            return Some(7);
        } else if asdf.len() >= 5 && &asdf[..5] == "eight" {
            return Some(8);
        } else if asdf.len() >= 4 && &asdf[..4] == "nine" {
            return Some(9);
        }
    }

    asdf.chars().next().unwrap().to_digit(10)
}

#[aoc_generator(day1)]
pub fn load_input(input: &str) -> String {
    input.to_string()
}

fn parse(input: &str, dumb: bool) -> Vec<u32> {
    let mut output = vec![];
    for _line in input.lines() {
        let mut digits = vec![];
        for i in 0.._line.len() {
            if let Some(value) = check(&_line[i..], dumb) {
                digits.push(value);
            }
        }
        //println!("{:?}", digits);
        output.push(10 * digits.first().unwrap() + digits.last().unwrap());
    }
    output
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    parse(input, false).into_iter().map(|x| x as usize).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    parse(input, true).into_iter().map(|x| x as usize).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/01b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 281);
    }
}
