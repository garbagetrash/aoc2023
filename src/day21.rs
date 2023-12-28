use std::collections::HashMap;

pub type Input = HashMap<(i64, i64), char>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn check_position_free(pos: (i64, i64), map: &HashMap<(i64, i64), char>) -> bool {
    if let Some(n) = map.get(&pos) {
        *n == '.' || *n == 'S'
    } else {
        false
    }
}

pub fn get_neighbors(pos: (i64, i64), map: &HashMap<(i64, i64), char>) -> Vec<(i64, i64)> {
    let mut neighbors = vec![];
    let positions = vec![
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
    ];
    for candidate_position in positions {
        if check_position_free(candidate_position, map) {
            neighbors.push(candidate_position);
        }
    }
    neighbors
}

#[aoc_generator(day21)]
pub fn load_input(input: &str) -> Input {
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i64, y as i64), c);
        }
    }
    map
}

pub fn solve_part1(input: &Input, nsteps: usize) -> usize {
    let map = input.clone();

    // Create a neighbors cache
    let mut neighbors_cache: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for (pos, tile) in map.iter() {
        if *tile == '.' || *tile == 'S' {
            neighbors_cache.insert(*pos, get_neighbors(*pos, &map));
        }
    }

    let start_position = map
        .iter()
        .filter(|(_k, &v)| v == 'S')
        .map(|(k, _)| k)
        .collect::<Vec<_>>()[0];
    let mut even_positions: Vec<(i64, i64)> = vec![*start_position];
    let mut odd_positions: Vec<(i64, i64)> = vec![];
    for i in 0..nsteps {
        //println!("i: {}", i);

        if i % 2 == 0 {
            // Even positions
            for pos in &even_positions {
                let neighbors = neighbors_cache.get(pos).unwrap();
                for n in neighbors {
                    if !odd_positions.contains(n) {
                        odd_positions.push(*n);
                    }
                }
            }
        } else {
            // Odd positions
            for pos in &odd_positions {
                let neighbors = neighbors_cache.get(pos).unwrap();
                for n in neighbors {
                    if !even_positions.contains(n) {
                        even_positions.push(*n);
                    }
                }
            }
        }
    }
    //print_map(&even_positions, &map);
    //println!("even_positions: {:?}", even_positions);
    even_positions.len()
}

#[allow(dead_code)]
fn print_map(highlight: &[(i64, i64)], tilemap: &HashMap<(i64, i64), char>) {
    let xmax = *tilemap.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *tilemap.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..ymax + 1 {
        for x in 0..xmax + 1 {
            let mut color = false;
            if highlight.contains(&(x, y)) {
                color = true;
            }
            let cc = tilemap.get(&(x, y)).unwrap();
            if color {
                print!("\x1b[91m{}\x1b[0m", *cc);
            } else {
                print!("{}", *cc);
            }
        }
        println!();
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &Input) -> usize {
    solve_part1(input, 64)
}

#[aoc(day21, part2)]
pub fn part2(input: &Input) -> i64 {
    solve_part2(input, 26501365)
}

pub fn simulate(input: &Input, nsteps: usize, start_position: (i64, i64)) -> Vec<(i64, i64)> {
    let map = input.clone();

    // Create a neighbors cache
    let mut neighbors_cache: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for (pos, tile) in map.iter() {
        if *tile == '.' || *tile == 'S' {
            neighbors_cache.insert(*pos, get_neighbors(*pos, &map));
        }
    }

    let mut even_positions: Vec<(i64, i64)> = vec![start_position];
    let mut odd_positions: Vec<(i64, i64)> = vec![];
    for i in 0..nsteps {
        //println!("i: {}", i);

        if i % 2 == 0 {
            // Even positions
            for pos in &even_positions {
                let neighbors = neighbors_cache.get(pos).unwrap();
                for n in neighbors {
                    //if !even_positions.contains(n) && !odd_positions.contains(n) {
                    if !odd_positions.contains(n) {
                        odd_positions.push(*n);
                    }
                }
            }
        } else {
            // Odd positions
            for pos in &odd_positions {
                let neighbors = neighbors_cache.get(pos).unwrap();
                for n in neighbors {
                    //if !even_positions.contains(n) && !odd_positions.contains(n) {
                    if !even_positions.contains(n) {
                        even_positions.push(*n);
                    }
                }
            }
        }
    }
    //print_map(&even_positions, &map);
    //println!("even_positions: {:?}", even_positions);
    even_positions
}

// 613473117038252 is too high
pub fn solve_part2(input: &Input, nsteps: usize) -> i64 {
    // make an infinite field of diamond patterns, limit to the # of steps (diamond shape because
    // manhattan distance) and then somehow remove all instances that land on rocks... just some
    // modulus math but how to determine how many fields to remove in a go? or maybe just be dumb
    // and actually iterate x+y=bignumber? No, do not just iterate.
    let thenum = nsteps as i64;
    let map = input.clone();
    let xmax = map.keys().map(|(x, _y)| x).max().unwrap();
    let half_field = xmax / 2;
    let field_width = *xmax + 1;
    let n_fields = (thenum - half_field) / field_width;
    let nn = (thenum + 1) / field_width;
    let full_0 = nn.pow(2);
    let full_1 = (nn - 1).pow(2);

    // NOTE: field_mod is going to indicated whether the center of a given field is "on" or not.
    // The starting field has a field_mod = # steps % 2. field_mod of the 4 surrounding fields will
    // be = 1, indicating the center tile is "off".

    // If you do the math you see 26501365 - 65 (half the field width) / 131 (the full field width)
    // yields 202300.0 exactly. The cheeky number is a good indication we're on the right path.
    // Anyways, because the x and y dimensions are equal our field is a perfect square being
    // tessallated in a perfect diamond. We need to calculate the number of rocks in the full field
    // for the vast majority of the tessallated squares, the number of rocks when you shave off a
    // point exactly in the middle of the cardinal directions (right triangles with hypotenuse 65),
    // and the number of rocks when you shave off each corner, and the corners themselves.
    //
    // The number of positions in before removing rocks uses the fact that we have a single point
    // on top, then 2, then 3, ect.. until full width, so summing all those gives us the power
    // series expansion 1+2+3+...+n = n*(n+1)/2.
    let mut output = 0;

    // Handle the middle
    let full_field0 = simulate(input, 260, (0, 0));
    //print_map(&full_field0, &map);
    output += full_0 * full_field0.len() as i64;
    let full_field1 = simulate(input, 260, (0, 1));
    //print_map(&full_field1, &map);
    output += full_1 * full_field1.len() as i64;

    // Handle the four tips
    let northern_tip = simulate(input, 130, (65, 130));
    //print_map(&northern_tip, &map);
    output += northern_tip.len() as i64;

    let southern_tip = simulate(input, 130, (65, 0));
    //print_map(&southern_tip, &map);
    output += southern_tip.len() as i64;

    let eastern_tip = simulate(input, 130, (0, 65));
    //print_map(&eastern_tip, &map);
    output += eastern_tip.len() as i64;

    let western_tip = simulate(input, 130, (130, 65));
    //print_map(&western_tip, &map);
    output += western_tip.len() as i64;

    // Handle the edges
    let southeast0 = simulate(input, 130 + 64, (0, 1));
    //print_map(&southeast0, &map);
    let southeast1 = simulate(input, 65, (0, 0));
    //print_map(&southeast1, &map);
    output += (n_fields - 1) * southeast0.len() as i64;
    output += n_fields * southeast1.len() as i64;

    let northeast0 = simulate(input, 130 + 64, (0, 129));
    let northeast1 = simulate(input, 65, (0, 130));
    //print_map(&northeast1, &map);
    //print_map(&northeast0, &map);
    output += (n_fields - 1) * northeast0.len() as i64;
    output += n_fields * northeast1.len() as i64;

    let southwest0 = simulate(input, 130 + 64, (130, 1));
    //print_map(&southwest0, &map);
    let southwest1 = simulate(input, 65, (130, 0));
    //print_map(&southwest1, &map);
    output += (n_fields - 1) * southwest0.len() as i64;
    output += n_fields * southwest1.len() as i64;

    let northwest0 = simulate(input, 130 + 64, (130, 129));
    let northwest1 = simulate(input, 65, (130, 130));
    //print_map(&northwest1, &map);
    //print_map(&northwest0, &map);
    output += (n_fields - 1) * northwest0.len() as i64;
    output += n_fields * northwest1.len() as i64;

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part1(&input, 6), 16);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part2(&input, 10), 50);

        let input = read_to_string("input/2023/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part2(&input, 50), 1594);

        let input = read_to_string("input/2023/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part2(&input, 100), 6536);

        let input = read_to_string("input/2023/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part2(&input, 500), 167004);

        let input = read_to_string("input/2023/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part2(&input, 1000), 668697);

        let input = read_to_string("input/2023/21a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part2(&input, 5000), 16733044);
    }
}
