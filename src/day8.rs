use std::collections::{HashMap, HashSet, VecDeque};
use scan_fmt::scan_fmt;

type Input = (Vec<char>, HashMap<String, (String, String)>);


#[aoc_generator(day8)]
pub fn load_input(input: &str) -> Input {
    let instructions = input.lines().next().unwrap().chars().collect();
    let mut mapping = HashMap::new();
    for line in input.lines().skip(2) {
        let temp: Vec<_> = line.split(' ').collect();
        let node = temp[0].to_string();
        let left = temp[2].strip_prefix("(").unwrap().strip_suffix(",").unwrap().to_string();
        let right = temp[3].strip_suffix(")").unwrap().to_string();
        mapping.insert(node, (left, right));
    }
    (instructions, mapping)
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> i64 {
    let (inst, map) = input;
    let mut node = String::from("AAA");
    let mut idx = 0;
    let mut cnt = 0;
    loop {
        //println!("node: {}", node);
        if let Some(n) = map.get(&node) {
            let dir = inst[idx];
            if dir == 'L' {
                node = n.0.clone();
            } else {
                node = n.1.to_string();
            }
        } else {
            panic!("asdf");
        }
        idx += 1;
        if idx >= inst.len() {
            idx = 0;
        }
        cnt += 1;

        if node == "ZZZ" {
            break;
        }
    }
    cnt
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    let (inst, map) = input;
    let mut start_nodes: Vec<_> = map.keys().filter(|k| k.chars().last().unwrap() == 'A').collect();
    println!("start_nodes: {:?}", start_nodes);

    let mut end_nodes: Vec<_> = map.keys().filter(|k| k.chars().last().unwrap() == 'Z').collect();
    println!("end_nodes: {:?}", end_nodes);

    let mut fwdmap: HashMap<(String, usize), String> = HashMap::new();
    let mut node = String::from("AAA");
    let mut idx = 0;
    loop {
        //println!("node: {}", node);
        let last_node = node.clone();
        if let Some(n) = map.get(&node) {
            let dir = inst[idx];
            if dir == 'L' {
                node = n.0.clone();
            } else {
                node = n.1.to_string();
            }
        } else {
            panic!("asdf");
        }

        if let Some(_) = fwdmap.insert((last_node.clone(), idx), node.clone()) {
            // if already present, break
            break;
        }

        idx += 1;
        if idx >= inst.len() {
            idx = 0;
        }
    }

    println!("fwdmap.len(): {}", fwdmap.len());
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/08a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/08b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 6);
    }
}
