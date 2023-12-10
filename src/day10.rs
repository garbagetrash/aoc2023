use num::Integer;
use std::collections::HashMap;
use aoc_helpers::graph::{Connected, Graph};

type Input = Graph::<(usize, usize, Tile)>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

#[aoc_generator(day10)]
pub fn load_input(input: &str) -> Input {
    let mut map = HashMap::new();
    let mut g = Graph::<(usize, usize, Tile)>::new();

    // Add nodes
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' => {g.add_node_with_value((x, y, Tile::V));},
                '-' => {g.add_node_with_value((x, y, Tile::H));},
                'L' => {g.add_node_with_value((x, y, Tile::NE));},
                'J' => {g.add_node_with_value((x, y, Tile::NW));},
                '7' => {g.add_node_with_value((x, y, Tile::SW));},
                'F' => {g.add_node_with_value((x, y, Tile::SE));},
                '.' => {g.add_node_with_value((x, y, Tile::G));},
                'S' => {g.add_node_with_value((x, y, Tile::S));},
                _ => (),
            };
            map.insert((x, y), c);
        }
    }

    // Add edges
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' => {
                    // above
                    if let Some(&t) = map.get(&(x, y - 1)) {
                        if t == '|' || t == '7' || t == 'F' {
                            //g.add_edge();
                        }
                    }

                    // below
                    if let Some(&t) = map.get(&(x, y + 1)) {
                        if t == '|' || t == 'L' || t == 'J' {
                            //g.add_edge();
                        }
                    }
                },
                '-' => {g.add_node_with_value((x, y, Tile::H));},
                'L' => {g.add_node_with_value((x, y, Tile::NE));},
                'J' => {g.add_node_with_value((x, y, Tile::NW));},
                '7' => {g.add_node_with_value((x, y, Tile::SW));},
                'F' => {g.add_node_with_value((x, y, Tile::SE));},
                '.' => {g.add_node_with_value((x, y, Tile::G));},
                'S' => {g.add_node_with_value((x, y, Tile::S));},
                _ => (),
            };
        }
    }

    g
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> i64 {
    0
}

#[aoc(day10, part2)]
pub fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/10a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/10b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 6);
    }
}
