use std::collections::HashMap;

type Input = Vec<Vec<char>>;

#[aoc_generator(day14)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let row = line.chars().collect();
        output.push(row);
    }
    output
}

fn tilt_north(map: &[Vec<char>]) -> Input {
    let nrows = map.len();
    let ncols = map[0].len();
    let mut output = vec![vec!['.'; ncols]; nrows];

    // Walk down columns, gather 'O' to top.
    // Reset the "top" every '#'.
    for x in 0..ncols {
        let mut top = 0;
        let mut nstones = 0;
        for y in 0..nrows {
            match map[y][x] {
                '.' => (),
                'O' => nstones += 1,
                '#' => {
                    output[y][x] = '#';
                    // Fill in stones here
                    for i in 0..nstones {
                        output[top + i][x] = 'O';
                    }
                    top = y + 1;
                    nstones = 0;
                }
                _ => {}
            }
        }

        // Push up stones one last time before next column
        // Fill in stones here
        for i in 0..nstones {
            output[top + i][x] = 'O';
        }
    }

    output
}

fn count_score(map: &[Vec<char>]) -> usize {
    let mut output = 0;
    let nrows = map.len();
    for (i, row) in map.iter().enumerate() {
        let nrocks = row.iter().filter(|c| **c == 'O').count();
        let value = nrows - i;
        output += value * nrocks;
    }
    output
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> usize {
    let tilted = tilt_north(input);
    count_score(&tilted)
}

fn tilt_west(map: &[Vec<char>]) -> Input {
    let nrows = map.len();
    let ncols = map[0].len();
    let mut output = vec![vec!['.'; ncols]; nrows];

    // Walk across rows (left to right), gather 'O' to left/west.
    // Reset the "wall" every '#'.
    for y in 0..nrows {
        let mut wall = 0;
        let mut nstones = 0;
        for x in 0..ncols {
            match map[y][x] {
                '.' => (),
                'O' => nstones += 1,
                '#' => {
                    output[y][x] = '#';
                    // Fill in stones here
                    for i in 0..nstones {
                        output[y][wall + i] = 'O';
                    }
                    wall = x + 1;
                    nstones = 0;
                }
                _ => {}
            }
        }

        // Push stones one last time before next row
        // Fill in stones here
        for i in 0..nstones {
            output[y][wall + i] = 'O';
        }
    }

    output
}

fn tilt_south(map: &[Vec<char>]) -> Input {
    let nrows = map.len();
    let ncols = map[0].len();
    let mut output = vec![vec!['.'; ncols]; nrows];

    // Walk down columns, gather 'O' to top.
    // Reset the "top" every '#'.
    for x in 0..ncols {
        let mut top = nrows - 1;
        let mut nstones = 0;
        for y in (0..nrows).rev() {
            match map[y][x] {
                '.' => (),
                'O' => nstones += 1,
                '#' => {
                    output[y][x] = '#';
                    // Fill in stones here
                    for i in 0..nstones {
                        output[top - i][x] = 'O';
                    }
                    if y > 0 {
                        top = y - 1;
                    } else {
                        top = 0;
                    }
                    nstones = 0;
                }
                _ => {}
            }
        }

        // Push up stones one last time before next column
        // Fill in stones here
        for i in 0..nstones {
            output[top - i][x] = 'O';
        }
    }

    output
}

fn tilt_east(map: &[Vec<char>]) -> Input {
    let nrows = map.len();
    let ncols = map[0].len();
    let mut output = vec![vec!['.'; ncols]; nrows];

    // Walk across rows (left to right), gather 'O' to left/west.
    // Reset the "wall" every '#'.
    for y in 0..nrows {
        let mut wall = ncols - 1;
        let mut nstones = 0;
        for x in (0..ncols).rev() {
            match map[y][x] {
                '.' => (),
                'O' => nstones += 1,
                '#' => {
                    output[y][x] = '#';
                    // Fill in stones here
                    for i in 0..nstones {
                        output[y][wall - i] = 'O';
                    }
                    if x > 0 {
                        wall = x - 1;
                    } else {
                        wall = 0;
                    }
                    nstones = 0;
                }
                _ => {}
            }
        }

        // Push stones one last time before next row
        // Fill in stones here
        for i in 0..nstones {
            output[y][wall - i] = 'O';
        }
    }

    output
}

fn do_cycle(map: &[Vec<char>]) -> Input {
    // Cycle is north, west, south, east
    let map = tilt_north(map);
    let map = tilt_west(&map);
    let map = tilt_south(&map);
    tilt_east(&map)
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> usize {
    let mut map = input.clone();
    let mut statemap: HashMap<Input, usize> = HashMap::new();
    let mut t: usize = 1;
    loop {
        map = do_cycle(&map);
        //print_map(&map);
        if let Some(last_t) = statemap.insert(map.clone(), t) {
            println!(
                "Hit state cycle at t: {}, last encountered at t: {}",
                t, last_t
            );
            let cycle_length = t - last_t;
            let idx = (1_000_000_000 - last_t) % cycle_length;
            let final_state = statemap
                .iter()
                .find(|(_k, v)| **v == idx + last_t)
                .map(|(k, _v)| k)
                .unwrap();
            return count_score(final_state);
        }
        t += 1;
    }
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
        let input = read_to_string("input/2023/14a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 136);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/14a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 64);
    }
}
