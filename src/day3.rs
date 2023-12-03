// Vector of Rows of (numbers, symbols)
type Input = Vec<(Vec<(i64, i64)>, Vec<(i64, char)>)>;

#[aoc_generator(day3)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let mut numbers = vec![];
        let mut symbols = vec![];
        let mut current_number = String::new();
        let mut in_number = false;
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                current_number.push(c);
                in_number = true;
            } else if in_number {
                numbers.push((
                    i as i64 - current_number.len() as i64,
                    current_number.parse::<i64>().unwrap(),
                ));
                in_number = false;
                current_number = String::new();
            }

            if i == line.len() - 1 && in_number {
                numbers.push((
                    (i + 1) as i64 - current_number.len() as i64,
                    current_number.parse::<i64>().unwrap(),
                ));
                in_number = false;
                current_number = String::new();
            }

            if !c.is_ascii_digit() && c != '.' {
                symbols.push((i as i64, c));
            }
        }

        output.push((numbers, symbols));
    }
    output
}

#[aoc(day3, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut valid = vec![];
    for i in 0..input.len() {
        for number in &input[i].0 {
            let mut bail = false;
            let number_start_col = number.0;
            let number_len = number.1.to_string().len() as i64;

            // Check row above for symbols
            if i > 0 {
                for symbol in &input[i - 1].1 {
                    let symbol_col = symbol.0;
                    if symbol_col >= number_start_col - 1
                        && symbol_col <= number_start_col + number_len
                    {
                        valid.push(number.1);
                        bail = true;
                        break;
                    }
                }
            }
            if bail {
                continue;
            };

            // Check row below for symbols
            if i < input.len() - 1 {
                for symbol in &input[i + 1].1 {
                    let symbol_col = symbol.0;
                    if symbol_col >= number_start_col - 1
                        && symbol_col <= number_start_col + number_len
                    {
                        valid.push(number.1);
                        bail = true;
                    }
                    if bail {
                        break;
                    };
                }
            }
            if bail {
                continue;
            };

            // Check current row for symbols
            for symbol in &input[i].1 {
                let symbol_col = symbol.0;
                if symbol_col == number_start_col - 1 || symbol_col == number_start_col + number_len
                {
                    valid.push(number.1);
                }
            }
        }
    }
    valid.iter().sum::<i64>()
}

#[aoc(day3, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut ratios = vec![];
    for i in 0..input.len() {
        for symbol in input[i].1.iter().filter(|x| x.1 == '*') {
            let mut bail = false;
            let symbol_col = symbol.0;
            let mut valid = vec![];

            // Check row above for numbers
            if i > 0 {
                for number in &input[i - 1].0 {
                    let number_start_col = number.0;
                    let number_len = number.1.to_string().len() as i64;
                    if symbol_col >= number_start_col - 1
                        && symbol_col <= number_start_col + number_len
                    {
                        valid.push(number.1);

                        if valid.len() == 2 {
                            ratios.push(valid[0] * valid[1]);
                            bail = true;
                            break;
                        }
                    }
                }
            }
            if bail {
                continue;
            };

            // Check row below for numbers
            if i < input.len() - 1 {
                for number in &input[i + 1].0 {
                    let number_start_col = number.0;
                    let number_len = number.1.to_string().len() as i64;
                    if symbol_col >= number_start_col - 1
                        && symbol_col <= number_start_col + number_len
                    {
                        valid.push(number.1);
                        if valid.len() == 2 {
                            ratios.push(valid[0] * valid[1]);
                            bail = true;
                            break;
                        }
                    }
                }
            }
            if bail {
                continue;
            };

            // Check current row for numbers
            for number in &input[i].0 {
                let number_start_col = number.0;
                let number_len = number.1.to_string().len() as i64;
                if symbol_col == number_start_col - 1 || symbol_col == number_start_col + number_len
                {
                    valid.push(number.1);
                    if valid.len() == 2 {
                        ratios.push(valid[0] * valid[1]);
                        break;
                    }
                }
            }
        }
    }
    ratios.iter().sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/03a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/03a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 467835);
    }
}
