pub type Input = Vec<Snowflake>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Snowflake {
    pub pos: (f64, f64, f64),
    pub vel: (f64, f64, f64),
}

impl Snowflake {
    pub fn new(pos: (f64, f64, f64), vel: (f64, f64, f64)) -> Self {
        Self { pos, vel }
    }

    /// Evaluate position with parameterized t.
    pub fn evaluate(&self, t: f64) -> (f64, f64, f64) {
        let x = self.vel.0 * t + self.pos.0;
        let y = self.vel.1 * t + self.pos.1;
        let z = self.vel.2 * t + self.pos.2;
        (x, y, z)
    }

    pub fn intersection_xy(&self, other: &Snowflake) -> Option<(f64, f64)> {
        let t1 = (self.vel.1 * ((other.pos.0 - self.pos.0) / self.vel.0) + self.pos.1
            - other.pos.1)
            / (other.vel.1 * (1_f64 - (self.vel.1 * other.vel.0) / (other.vel.1 * self.vel.0)));
        //println!("t1: {}", t1);
        if t1 < 0.0 {
            return None;
        }
        let t0 = (other.vel.0 * t1 + other.pos.0 - self.pos.0) / self.vel.0;
        //println!("t0: {}", t0);
        if t0 < 0.0 {
            return None;
        }
        let p0 = self.evaluate(t0);
        Some((p0.0, p0.1))
    }

    /// TODO: Part 1 wants _future_ intersections, disregard past.
    pub fn intersect_in_bounds(&self, other: &Snowflake, low: f64, high: f64) -> bool {
        // Verify if its even possible for X's to intersect in area in the future
        if (self.pos.0 < low && self.vel.0 < 0.0)
            || (self.pos.0 > high && self.vel.0 > 0.0)
            || (other.pos.0 < low && other.vel.0 < 0.0)
            || (other.pos.0 > high && other.vel.0 > 0.0)
        {
            return false;
        }
        // Verify if its even possible for Y's to intersect in area in the future
        if (self.pos.1 < low && self.vel.1 < 0.0)
            || (self.pos.1 > high && self.vel.1 > 0.0)
            || (other.pos.1 < low && other.vel.1 < 0.0)
            || (other.pos.1 > high && other.vel.1 > 0.0)
        {
            return false;
        }

        let p_intersect = self.intersection_xy(other);

        if let Some(point) = p_intersect {
            point.0 > low && point.0 < high && point.1 > low && point.1 < high
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
        let pos: Vec<f64> = temp
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect();
        let pos = (pos[0], pos[1], pos[2]);
        let vel: Vec<f64> = temp
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect();
        let vel = (vel[0], vel[1], vel[2]);
        flakes.push(Snowflake::new(pos, vel));
    }
    flakes
}

pub fn solve_part1(input: &Input, low: f64, high: f64) -> usize {
    let n_flakes = input.len();
    let mut output = 0;
    for i in 0..n_flakes {
        let flake1 = input[i];
        for flake2 in &input[i + 1..] {
            if flake1.intersect_in_bounds(flake2, low, high) {
                output += 1;
            }
        }
    }
    output
}

#[aoc(day24, part1)]
pub fn part1(input: &Input) -> usize {
    solve_part1(input, 200000000000000.0, 400000000000000.0)
}

#[aoc(day24, part2)]
pub fn part2(_input: &Input) -> i64 {
    // Solve this stupid system of equations with sympy, see `day24.py`
    808107741406756
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/24a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(solve_part1(&input, 7_f64, 27_f64), 2);
    }
}
