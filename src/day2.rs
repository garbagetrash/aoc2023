type Input = Vec<Vec<(usize, usize, usize)>>;

#[aoc_generator(day2)]
pub fn load_input(input: &str) -> Input {
    let mut games = vec![];
    for line in input.lines() {
        let mut line_iter = line.split(':');
        let obs = line_iter.nth(1).unwrap();
        let gobs_iter = obs.split(';');

        let mut observations = vec![];
        for _obs in gobs_iter {
            let _obs = _obs.trim();
            let value_split = _obs.split(", ");
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for v in value_split {
                let value = v.split(' ').next().unwrap().parse::<usize>().unwrap();
                let color = v.split(' ').nth(1).unwrap();
                if color == "red" {
                    red = value;
                } else if color == "green" {
                    green = value;
                } else if color == "blue" {
                    blue = value;
                }
            }
            observations.push((red, green, blue));
        }

        games.push(observations);
    }
    games
}

#[aoc(day2, part1)]
pub fn part1(input: &Input) -> usize {
    let limit_red = 12;
    let limit_green = 13;
    let limit_blue = 14;
    let mut output = 0;
    for (i, game) in input.iter().enumerate() {
        let mut possible = true;
        for obs in game {
            if obs.0 > limit_red {
                possible = false;
                break;
            }
            if obs.1 > limit_green {
                possible = false;
                break;
            }
            if obs.2 > limit_blue {
                possible = false;
                break;
            }
        }

        if possible {
            output += i + 1;
        }
    }
    output
}

#[aoc(day2, part2)]
pub fn part2(input: &Input) -> usize {
    let mut output = 0;
    for game in input {
        let mut maxred = 0;
        let mut maxgreen = 0;
        let mut maxblue = 0;
        for obs in game {
            if obs.0 > maxred {
                maxred = obs.0;
            }
            if obs.1 > maxgreen {
                maxgreen = obs.1;
            }
            if obs.2 > maxblue {
                maxblue = obs.2;
            }
        }

        let power = maxred * maxgreen * maxblue;
        output += power;
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/02a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/02a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 2286);
    }
}
