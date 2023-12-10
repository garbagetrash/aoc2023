use num::Integer;
use std::collections::HashMap;

type Input = (Vec<char>, HashMap<String, (String, String)>);

#[aoc_generator(day8)]
pub fn load_input(input: &str) -> Input {
    let instructions = input.lines().next().unwrap().chars().collect();
    let mut mapping = HashMap::new();
    for line in input.lines().skip(2) {
        let temp: Vec<_> = line.split(' ').collect();
        let node = temp[0].to_string();
        let left = temp[2]
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .to_string();
        let right = temp[3].strip_suffix(')').unwrap().to_string();
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

fn find_cycle_lengths(
    instructions: &[char],
    map: &HashMap<String, (String, String)>,
) -> Vec<usize> {
    let end_nodes: Vec<String> = map.keys().cloned().filter(|k| k.ends_with('Z')).collect();

    let mut lengths = vec![];
    for start in end_nodes {
        let mut fwdmap: HashMap<(String, usize), String> = HashMap::new();
        let mut node = start.to_string();
        let mut idx = 0;
        loop {
            let last_node = node.clone();
            if let Some(n) = map.get(&node) {
                let dir = instructions[idx];
                if dir == 'L' {
                    node = n.0.clone();
                } else {
                    node = n.1.to_string();
                }
            } else {
                panic!("couldn't find this node in map!");
            }

            // `node` points to next node now, and `last_node` points to current
            if fwdmap
                .insert((last_node.clone(), idx), node.clone())
                .is_some()
            {
                // if already present, break
                break;
            }

            idx += 1;
            if idx >= instructions.len() {
                idx = 0;
            }
        }

        lengths.push(fwdmap.len());
    }
    lengths
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    let (inst, map) = input;
    let cycle_lengths = find_cycle_lengths(inst, map);
    let lcm05 = cycle_lengths.iter().fold(1, |a, b| a.lcm(b));
    lcm05
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
