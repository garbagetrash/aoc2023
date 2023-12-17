use rayon::prelude::*;
use std::collections::HashSet;

pub type Input = Vec<Vec<char>>;

#[aoc_generator(day16)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let temp: Vec<_> = line.chars().collect();
        output.push(temp);
    }
    output
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub type LS = ((i64, i64), Direction);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LightState {
    position: (i64, i64),
    direction: Direction,
    prior_states: HashSet<((i64, i64), Direction)>,
}

impl LightState {
    pub fn new(position: (i64, i64), direction: Direction) -> Self {
        let mut prior_states = HashSet::new();
        prior_states.insert((position, direction));
        Self {
            position,
            direction,
            prior_states,
        }
    }

    pub fn get_state(&self) -> LS {
        (self.position, self.direction)
    }

    // Returns a bool indicating if the new state is still valid or if this one is done (or
    // cycling)
    pub fn step(
        &mut self,
        energized: &mut HashSet<(i64, i64)>,
        map: &[Vec<char>],
    ) -> (bool, Option<LightState>) {
        let xmax = map[0].len() as i64;
        let ymax = map.len() as i64;
        let mut next_pos = self.position;
        energized.insert(self.position);

        // Look at current position in map to decide if we have to do something fancy or not
        let tile = map[self.position.1 as usize][self.position.0 as usize];
        let mut new_state = None;
        match tile {
            '.' => (),
            '/' => {
                // mirror
                let next_dir = match self.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                self.direction = next_dir;
            }
            '\\' => {
                // mirror
                let next_dir = match self.direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
                self.direction = next_dir;
            }
            '|' => {
                // splitter
                let next_dir = match self.direction {
                    Direction::North => Direction::North,
                    Direction::East => Direction::North,
                    Direction::South => Direction::South,
                    Direction::West => Direction::North,
                };
                self.direction = next_dir;

                // Spawn a new `LightState` going South at self.position
                new_state = Some(LightState::new(self.position, Direction::South));
            }
            '-' => {
                // splitter
                let next_dir = match self.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::East,
                    Direction::South => Direction::East,
                    Direction::West => Direction::West,
                };
                self.direction = next_dir;

                // Spawn a new `LightState` going West at self.position
                new_state = Some(LightState::new(self.position, Direction::West));
            }
            _ => panic!("Invalid tile"),
        }

        // Now that we're oriented in the correct direction, move
        match self.direction {
            Direction::North => next_pos.1 -= 1,
            Direction::East => next_pos.0 += 1,
            Direction::South => next_pos.1 += 1,
            Direction::West => next_pos.0 -= 1,
        }

        // Are we still in bounds?
        if next_pos.0 >= xmax || next_pos.0 < 0 || next_pos.1 >= ymax || next_pos.1 < 0 {
            (false, new_state)
        } else {
            self.position = next_pos;
            if self.prior_states.contains(&(self.position, self.direction)) {
                // If we've been here before, cycling so bail
                (false, new_state)
            } else {
                // Keep track of this new state to detect cycling
                self.prior_states.insert((self.position, self.direction));
                (true, new_state)
            }
        }
    }
}

pub fn simulate(starter: LightState, input: &Input) -> usize {
    let starter_state = starter.get_state();
    let mut energized: HashSet<(i64, i64)> = HashSet::new();
    let mut lights = vec![starter];
    let mut created_lights: HashSet<LS> = HashSet::new();
    created_lights.insert(starter_state);

    loop {
        if lights.is_empty() {
            // Done
            break;
        }

        // Take 1 step
        let mut new_lights = vec![];
        let mut done_lights = vec![];
        for (i, light) in lights.iter_mut().enumerate() {
            let (valid, maybe_new_light) = light.step(&mut energized, input);
            if let Some(new_light) = maybe_new_light {
                let new_state = new_light.get_state();
                if !created_lights.contains(&new_state) {
                    // Have not created this before
                    created_lights.insert(new_state);
                    new_lights.push(new_light);
                }
            }
            if !valid {
                done_lights.push(i);
            }
        }
        // Remove done Lights
        done_lights.sort();
        for idx in done_lights.iter().rev() {
            lights.remove(*idx);
        }

        // Add new lights if any
        lights.append(&mut new_lights);
    }

    energized.len()
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> usize {
    let starter = LightState::new((0, 0), Direction::East);
    simulate(starter, input)
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> usize {
    let mut start_states = vec![];
    let xmax = input[0].len() as i64;
    let ymax = input.len() as i64;
    for x in 0..xmax as usize {
        start_states.push(LightState::new((x as i64, 0), Direction::South));
        start_states.push(LightState::new((x as i64, ymax - 1), Direction::North));
    }
    for y in 0..ymax as usize {
        start_states.push(LightState::new((0, y as i64), Direction::East));
        start_states.push(LightState::new((xmax - 1, y as i64), Direction::West));
    }
    start_states
        .par_iter()
        .map(|s| simulate(s.clone(), input))
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn print_map(position: (i64, i64), input: &Input) {
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if (x as i64, y as i64) == position {
                print!("#");
            } else {
                print!("{}", c);
            }
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
        let input = read_to_string("input/2023/16a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/16a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 51);
    }
}
