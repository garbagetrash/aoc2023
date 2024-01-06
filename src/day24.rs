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

    /// Evaluate position with parameterized t.
    pub fn evaluate(&self, t: f64) -> (f64, f64, f64) {
        let x = self.vel.0 as f64 * t + self.pos.0 as f64;
        let y = self.vel.1 as f64 * t + self.pos.1 as f64;
        let z = self.vel.2 as f64 * t + self.pos.2 as f64;
        (x, y, z)
    }

    /// TODO: Part 1 wants _future_ intersections, disregard past.
    pub fn intersect_in_bounds(&self, other: &Snowflake, low: i64, high: i64) -> bool {
        // Verify if its even possible for X's to intersect in area in the future
        if (self.pos.0 < low && self.vel.0 < 0) || (self.pos.0 > high && self.vel.0 > 0) || (other.pos.0 < low && other.vel.0 < 0) || (other.pos.0 > high && other.vel.0 > 0) {
            return false;
        }
        // Verify if its even possible for Y's to intersect in area in the future
        if (self.pos.1 < low && self.vel.1 < 0) || (self.pos.1 > high && self.vel.1 > 0) || (other.pos.1 < low && other.vel.1 < 0) || (other.pos.1 > high && other.vel.1 > 0) {
            return false;
        }

        let t0_self = (low - self.pos.0) as f64 / self.vel.0 as f64;
        let t0_other = (low - other.pos.0) as f64 / other.vel.0 as f64;
        let t1_self = (high - self.pos.0) as f64 / self.vel.0 as f64;
        let t1_other = (high - other.pos.0) as f64 / other.vel.0 as f64;

        println!("t0_self: {}", t0_self);
        println!("t0_other: {}", t0_other);
        println!("t1_self: {}", t1_self);
        println!("t1_other: {}", t1_other);

        let (_, y1_self, _) = self.evaluate(t1_self);
        let (_, y1_other, _) = other.evaluate(t1_other);

        println!("y1_self: {}", y1_self);
        println!("y1_other: {}", y1_other);

        if (self.pos.1 > other.pos.1 && y1_self <= y1_other) || (self.pos.1 < other.pos.1 && y1_self >= y1_other) {
            true
        } else {
            false
        }
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

pub fn solve_part1(input: &Input, low: i64, high: i64) -> usize {
    let n_flakes = input.len();
    let mut output = 0;
    for i in 0..n_flakes {
        let flake1 = input[i];
        for j in i + 1..n_flakes {
            let flake2 = input[j];
            let mut intersected = false;
            if flake1.intersect_in_bounds(&flake2, low, high) {
                output += 1;
                intersected = true;
            }
            println!("{:?}, {:?}, {:?}", flake1, flake2, intersected);
        }
    }
    output
}

#[aoc(day24, part1)]
pub fn part1(input: &Input) -> usize {
    solve_part1(input, 200000000000000, 400000000000000)
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
        assert_eq!(solve_part1(&input, 7, 27), 2);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/24a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 7);
    }
}
