use rayon::prelude::*;
use std::collections::HashMap;

pub type Input = Vec<Vec<u8>>;

#[aoc_generator(day17)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let temp: Vec<_> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect();
        output.push(temp);
    }
    output
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct State {
    position: (usize, usize),
    direction: Direction,
}

impl State {
    pub fn new(position: (usize, usize), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Path {
    last_state: Option<State>,
    cost: usize,
}

impl Path {
    pub fn new() -> Self {
        Self {
            last_state: None,
            cost: 0,
        }
    }

    pub fn push(&mut self, state: State, new_cost: usize) {
        self.last_state = Some(state);
        self.cost += new_cost;
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

// Returns optional new position and cost of the move
pub fn try_move(
    position: (usize, usize),
    direction: Direction,
    n: usize,
    map: &[Vec<u8>],
) -> Option<((usize, usize), usize)> {
    let xmax = map[0].len();
    let ymax = map.len();

    match direction {
        Direction::North => {
            if position.1 < n {
                None
            } else {
                let mut cost = 0;
                for i in 1..n + 1 {
                    cost += map[position.1 - i][position.0] as usize;
                }
                Some(((position.0, position.1 - n), cost))
            }
        }
        Direction::East => {
            if position.0 + n >= xmax {
                None
            } else {
                let mut cost = 0;
                for i in 1..n + 1 {
                    cost += map[position.1][position.0 + i] as usize;
                }
                Some(((position.0 + n, position.1), cost))
            }
        }
        Direction::South => {
            if position.1 + n >= ymax {
                None
            } else {
                let mut cost = 0;
                for i in 1..n + 1 {
                    cost += map[position.1 + i][position.0] as usize;
                }
                Some(((position.0, position.1 + n), cost))
            }
        }
        Direction::West => {
            if position.0 < n {
                None
            } else {
                let mut cost = 0;
                for i in 1..n + 1 {
                    cost += map[position.1][position.0 - i] as usize;
                }
                Some(((position.0 - n, position.1), cost))
            }
        }
    }
}

pub fn next_states(state: State, map: &[Vec<u8>], part2: bool) -> Vec<(State, usize)> {
    let mut next_states = vec![];

    let dists = if part2 {
        vec![4, 5, 6, 7, 8, 9, 10]
    } else {
        vec![1, 2, 3]
    };

    // Can turn left or right
    match state.direction {
        Direction::North => {
            let dirs = vec![Direction::West, Direction::East];
            for dir in dirs {
                for dist in &dists {
                    if let Some((new_pos, cost)) = try_move(state.position, dir, *dist, map) {
                        next_states.push((State::new(new_pos, dir), cost));
                    }
                }
            }
        }
        Direction::East => {
            let dirs = vec![Direction::North, Direction::South];
            for dir in dirs {
                for dist in &dists {
                    if let Some((new_pos, cost)) = try_move(state.position, dir, *dist, map) {
                        next_states.push((State::new(new_pos, dir), cost));
                    }
                }
            }
        }
        Direction::South => {
            let dirs = vec![Direction::East, Direction::West];
            for dir in dirs {
                for dist in &dists {
                    if let Some((new_pos, cost)) = try_move(state.position, dir, *dist, map) {
                        next_states.push((State::new(new_pos, dir), cost));
                    }
                }
            }
        }
        Direction::West => {
            let dirs = vec![Direction::South, Direction::North];
            for dir in dirs {
                for dist in &dists {
                    if let Some((new_pos, cost)) = try_move(state.position, dir, *dist, map) {
                        next_states.push((State::new(new_pos, dir), cost));
                    }
                }
            }
        }
    }
    next_states
}

pub fn solve(input: &Input, part2: bool) -> usize {
    let map = input.clone();
    let xmax = map[0].len();
    let ymax = map.len();
    let start0 = State::new((0, 0), Direction::East);
    let start1 = State::new((0, 0), Direction::South);
    let mut next_states_and_costs = next_states(start0, &map, part2);
    next_states_and_costs.append(&mut next_states(start1, &map, part2));

    // Keep track of the minimum cost path to get to every (x, y)
    let mut minpaths: HashMap<State, Path> = HashMap::new();
    for (state, cost) in next_states_and_costs {
        let mut path = Path::new();
        path.push(state, cost);
        minpaths.insert(state, path);
    }

    let mut last_minpath_state;
    let mut niters: usize = 0;
    loop {
        println!("iter: {}", niters);
        last_minpath_state = minpaths.clone();
        let candidate_paths: Vec<_> = minpaths
            .par_iter()
            .flat_map(|(_key, path)| {
                //for path in minpaths.values() {
                // Where can this path go to?
                let state = path.last_state.unwrap();
                let next_states_and_costs = next_states(state, &map, part2);

                // Keep track of all these (maybe) new paths to see if any are worth keeping
                let mut output = vec![];
                for (new_state, new_cost) in next_states_and_costs {
                    let mut candidate_path = *path;
                    candidate_path.push(new_state, new_cost);
                    output.push(candidate_path);
                }
                output
            })
            .collect();

        // Insert new minpaths into minpaths
        for cpath in candidate_paths {
            let cstate = cpath.last_state.unwrap();
            if let Some(existing_path) = minpaths.get(&cstate) {
                // Compare if new path has lower cost
                if existing_path.cost > cpath.cost {
                    // If the new path is better, keep it!
                    minpaths.insert(cstate, cpath);
                }
            } else {
                minpaths.insert(cstate, cpath);
            }
        }
        if minpaths == last_minpath_state {
            break;
        } else {
            niters += 1;
        }
    }

    let endstate0 = State::new((xmax - 1, ymax - 1), Direction::East);
    let endstate1 = State::new((xmax - 1, ymax - 1), Direction::South);
    let mut solution = 0;
    if let Some(best_path) = minpaths.get(&endstate0) {
        solution = best_path.cost;
    }

    if let Some(best_path) = minpaths.get(&endstate1) {
        if best_path.cost < solution {
            solution = best_path.cost;
        }
    }
    solution
}

#[aoc(day17, part1)]
pub fn part1(input: &Input) -> usize {
    solve(input, false)
}

#[aoc(day17, part2)]
pub fn part2(input: &Input) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/17a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 102);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/17a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 94);

        let input = read_to_string("input/2023/17b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 71);
    }
}
