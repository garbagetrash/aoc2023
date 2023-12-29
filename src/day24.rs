use rayon::prelude::*;
use std::collections::HashSet;

pub type Input = Vec<Snowflake>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Snowflake {
    pub pos: (i64, i64, i64),
    pub vel: (i64, i64, i64),
}

impl Snowflake {
    pub fn new(pos: (i64, i64, i64), vel: (i64, i64, i64)) -> Self {
        Self { pos, vel }
    }

    pub fn dydx(&self) -> f64 {
        self.vel.1 as f64 / self.vel.0 as f64
    }

    pub fn dzdx(&self) -> f64 {
        self.vel.2 as f64 / self.vel.0 as f64
    }

    /// Evaluate position with parameterized x.
    pub fn evaluate(&self, x: i64) -> (f64, f64) {
        let diff = x - self.pos.0;
        let y = self.dydx() * (diff as f64) + self.pos.1 as f64;
        let z = self.dzdx() * (diff as f64) + self.pos.2 as f64;
        (y, z)
    }

    pub fn intersect_in_bounds(&self, other: &Snowflake, low: i64, high: i64) -> bool {

        false
    }
}

#[aoc_generator(day24)]
pub fn load_input(input: &str) -> Input {
    let mut flakes = vec![];
    for line in input.lines() {
        let mut temp = line.split('@');
        let pos: Vec<i64> = temp.next().unwrap().split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect();
        let pos = (pos[0], pos[1], pos[2]);
        let vel: Vec<i64> = temp.next().unwrap().split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect();
        let vel = (vel[0], vel[1], vel[2]);
        flakes.push(Snowflake::new(pos, vel));
    }
    flakes
}

#[aoc(day24, part1)]
pub fn part1(input: &Input) -> usize {
    let n_flakes = input.len();
    let mut output = 0;
    for i in 0..n_flakes {
        let flake1 = input[i];
        for j in i + 1..n_flakes {
            let flake2 = input[j];
            if flake1.intersect_in_bounds(&flake2, low, high) {
                output += 1;
            }
            println!("{:?}", flake1);
        }
    }
    output
}

#[aoc(day24, part2)]
pub fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/24a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/24a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 7);
    }
}
