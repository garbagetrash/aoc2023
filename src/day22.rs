use std::collections::{HashMap, HashSet};

pub type Input = Vec<Brick>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Brick {
    pub side1: (i64, i64, i64),
    pub side2: (i64, i64, i64),
}

impl Brick {
    pub fn new(side1: (i64, i64, i64), side2: (i64, i64, i64)) -> Self {
        Self { side1, side2 }
    }

    pub fn points(&self) -> Vec<(i64, i64, i64)> {
        if self.side1.0 != self.side2.0 {
            let (minx, maxx) = if self.side1.0 < self.side2.0 {
                (self.side1.0, self.side2.0)
            } else {
                (self.side2.0, self.side1.0)
            };
            (minx..maxx).map(|x| (x, self.side1.1, self.side1.2)).collect()
        } else if self.side1.1 != self.side2.1 {
            let (miny, maxy) = if self.side1.1 < self.side2.1 {
                (self.side1.1, self.side2.1)
            } else {
                (self.side2.1, self.side1.1)
            };
            (miny..maxy).map(|y| (self.side1.0, y, self.side1.2)).collect()
        } else if self.side1.2 != self.side2.2 {
            let (minz, maxz) = if self.side1.2 < self.side2.2 {
                (self.side1.2, self.side2.2)
            } else {
                (self.side2.2, self.side1.2)
            };
            (minz..maxz).map(|z| (self.side1.0, self.side1.1, z)).collect()
        } else {
            vec![self.side1]
        }
    }

    pub fn points_below(&self) -> Option<Vec<(i64, i64, i64)>> {
        let points = self.points();
        let output: Vec<(i64, i64, i64)> = points.iter().copied().map(|(x, y, z)| (x, y, z - 1)).collect();
        if output.iter().map(|(_, _, z)| z).any(|&z| z < 1) {
            None
        } else {
            Some(output)
        }
    }

    pub fn drop_one(&mut self) {
        self.side1.2 -= 1;
        self.side2.2 -= 1;
    }
}

#[aoc_generator(day22)]
pub fn load_input(input: &str) -> Input {
    let mut bricks = vec![];
    for line in input.lines() {
        let mut temp = line.split('~');
        let dim1: Vec<i64> = temp.next().unwrap().split(',').map(|c| c.parse::<i64>().unwrap()).collect();
        let dim1 = (dim1[0], dim1[1], dim1[2]);
        let dim2: Vec<i64> = temp.next().unwrap().split(',').map(|c| c.parse::<i64>().unwrap()).collect();
        let dim2 = (dim2[0], dim2[1], dim2[2]);
        bricks.push(Brick::new(dim1, dim2));
    }
    bricks
}

#[allow(dead_code)]
fn print_map_level(bricks: &[Brick], z: i64) {
    let pts_occupied: HashSet<(i64, i64, i64)> = bricks.iter().flat_map(|brick| brick.points()).collect();
    let xmax = 10;
    let ymax = 10;
    for y in 0..ymax {
        for x in 0..xmax {
            if pts_occupied.contains(&(x, y, z)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn fall(bricks: &mut Vec<Brick>) {
    loop {
        let pts_occupied: HashSet<(i64, i64, i64)> = bricks.iter().flat_map(|brick| brick.points()).collect();
        let mut idxs = vec![];
        for (i, brick) in bricks.iter().enumerate() {
            if let Some(pts_below) = brick.points_below() {
                if !pts_below.iter().any(|p| pts_occupied.contains(p)) {
                    // If all points below are unoccupied, then we can move this brick down by 1
                    idxs.push(i);
                }
            }
        }

        if idxs.len() == 0 {
            // Done dropping all bricks
            break;
        }

        idxs.sort();

        for i in idxs.iter().rev() {
            bricks[*i].drop_one();
        }
    }
}

pub fn bricks_below(brick: Brick, bricks: &[Brick]) -> Vec<Brick> {
    let mut output = vec![];
    if let Some(pts_below) = brick.points_below() {
        for brick2 in bricks {
            for pt in brick2.points() {
                if pts_below.contains(&pt) {
                    // brick2 is below brick
                    output.push(*brick2);
                }
            }
        }
    }
    output
}

#[aoc(day22, part1)]
// 617 is too high
pub fn part1(input: &Input) -> usize {
    let mut bricks = input.clone();
    // First let all the bricks fall to resting position
    fall(&mut bricks);

    for z in 0..10 {
        println!("z = {}", z);
        print_map_level(&bricks, z as i64);
        println!();
    }

    // TODO: Next, determine which bricks rest on other bricks, and if a brick can be removed
    // safely.
    let all: HashSet<Brick> = bricks.iter().copied().collect();
    let mut necessary: HashSet<Brick> = HashSet::new();
    for brick in &bricks {
        let below = bricks_below(*brick, &bricks);
        if below.len() == 1 {
            necessary.insert(below[0]);
        }
    }
    all.difference(&necessary).collect::<Vec<_>>().len()
}

#[aoc(day22, part2)]
pub fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/22a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/22a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 50);
    }
}
