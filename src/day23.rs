use aoc_helpers::tree::Tree;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub type Input = HashMap<(usize, usize), char>;

#[aoc_generator(day23)]
pub fn load_input(input: &str) -> Input {
    let mut output = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            output.insert((x, y), c);
        }
    }
    output
}

#[allow(dead_code)]
fn print_map(highlight: &[(usize, usize)], map: &Input) {
    let xmax = map.keys().map(|(x, _y)| *x).max().unwrap();
    let ymax = map.keys().map(|(_x, y)| *y).max().unwrap();
    for y in 0..ymax + 1 {
        for x in 0..xmax + 1 {
            let mut color = false;
            if highlight.contains(&(x, y)) {
                color = true;
            }
            let c = map.get(&(x, y)).unwrap();
            if color {
                print!("\x1b[91m{}\x1b[0m", *c);
            } else {
                print!("{}", *c);
            }
        }
        println!();
    }
}

pub fn get_neighbors(point: (usize, usize), map: &Input, part2: bool) -> Vec<(usize, usize)> {
    // Handle slopes right away
    let p = *map.get(&point).unwrap();
    if !part2 {
        if p == '<' {
            return vec![(point.0 - 1, point.1)];
        } else if p == '>' {
            return vec![(point.0 + 1, point.1)];
        } else if p == '^' {
            return vec![(point.0, point.1 - 1)];
        } else if p == 'v' {
            return vec![(point.0, point.1 + 1)];
        }
    }

    let xmax = map.keys().map(|(x, _y)| *x).max().unwrap();
    let ymax = map.keys().map(|(_x, y)| *y).max().unwrap();
    let mut candidates = vec![];
    if point.0 > 0 {
        candidates.push((point.0 - 1, point.1));
    }
    if point.0 < xmax {
        candidates.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        candidates.push((point.0, point.1 - 1));
    }
    if point.1 < ymax {
        candidates.push((point.0, point.1 + 1));
    }

    let mut output = vec![];
    for cand in candidates {
        let c = map.get(&cand).unwrap();
        if part2 {
            if *c != '#' {
                output.push(cand);
            }
        } else {
            if *c == '.' {
                output.push(cand);
            } else if *c == '<' && point.0 > cand.0 {
                output.push(cand);
            } else if *c == '>' && point.0 < cand.0 {
                output.push(cand);
            } else if *c == '^' && point.1 > cand.1 {
                output.push(cand);
            } else if *c == 'v' && point.1 < cand.1 {
                output.push(cand);
            }
        }
    }
    output
}

pub fn find_save_points(input: &Input) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let start = (1, 0);
    let mut frontier = vec![start];
    let mut explored: Vec<(usize, usize)> = vec![start];
    let mut save_points: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    loop {
        if frontier.len() == 0 {
            break;
        }

        let mut new_frontier = vec![];
        for pt in &frontier {
            let neighbors: Vec<(usize, usize)> = get_neighbors(*pt, input, false)
                .iter()
                .filter(|n| !explored.contains(n) && !frontier.contains(n))
                .copied()
                .collect();
            if neighbors.len() > 1 {
                // This is a "save point", a diverging path.
                save_points.insert(*pt, neighbors.clone());
            }
            for n in neighbors {
                if !explored.contains(&n) {
                    explored.push(n);
                    if !frontier.contains(&n) {
                        new_frontier.push(n);
                    }
                }
            }
        }
        frontier = new_frontier;
    }
    save_points
}

pub fn find_save_points2(input: &Input) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let points: Vec<_> = input.iter().filter(|(k, &v)| v == '.').collect();

    let mut output = HashMap::new();
    for (pt, _) in points {
        let neighbors = get_neighbors(*pt, input, true);
        if neighbors.len() > 2 {
            output.insert(*pt, neighbors);
        }
    }
    output
}

// length includes the starting point and the next save point.
pub fn path_length(start: (usize, usize), input: &Input) -> (usize, (usize, usize)) {
    let mut frontier = vec![start];
    let mut explored: Vec<(usize, usize)> = vec![start];
    let mut last = start;
    loop {
        if frontier.len() == 0 {
            break;
        }

        let mut new_frontier = vec![];
        for pt in &frontier {
            let neighbors: Vec<(usize, usize)> = get_neighbors(*pt, input, false)
                .iter()
                .filter(|n| !explored.contains(n))
                .copied()
                .collect();
            if neighbors.len() > 1 {
                // This is a "save point", a diverging path.
                return (explored.len(), *pt);
            }
            for n in neighbors {
                if !frontier.contains(&n) {
                    explored.push(n);
                    new_frontier.push(n);
                }
            }
        }
        last = frontier[0];
        frontier = new_frontier;
    }
    (explored.len(), last)
}

#[aoc(day23, part1)]
pub fn part1(input: &Input) -> usize {
    // Explore the graph once keeping track of "save points"
    let save_points = find_save_points(input);

    //print_map(&vec![], input);

    // Start path tree with the path to the first save point
    let (length, dst) = path_length((1, 0), input);
    let mut path_tree = Tree::with_head(((1, 0), dst, length));

    let mut cids = vec![0];
    loop {
        if cids.len() == 0 {
            break;
        }

        let mut new_cids = vec![];
        for pid in cids {
            let dst = path_tree.nodes[pid].value.1;
            let tree_length = path_tree.nodes[pid].value.2;
            if let Some(paths) = save_points.get(&dst) {
                for start in paths {
                    let (length, next_dst) = path_length(*start, input);
                    let cid = path_tree
                        .add_child_to_node((*start, next_dst, tree_length + length), pid)
                        .unwrap();
                    new_cids.push(cid);
                }
            }
        }
        cids = new_cids;
    }

    *path_tree
        .leaf_values()
        .iter()
        .map(|(_, _, l)| l)
        .max()
        .unwrap()
        - 1
}

pub fn path_length2(
    start: (usize, usize),
    save_points: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    input: &Input,
) -> (usize, (usize, usize)) {
    let mut frontier = vec![start];
    let mut explored: Vec<(usize, usize)> = vec![start];
    let mut last = start;
    let mut ends = vec![];
    let mut bail = false;
    let start_map = input
        .iter()
        .filter(|(k, &v)| k.1 == 0 && v == '.')
        .map(|(k, _v)| k)
        .collect::<Vec<_>>()[0];
    let ymax = input.keys().map(|(_x, y)| *y).max().unwrap();
    let end_map = input
        .iter()
        .filter(|(k, &v)| k.1 == ymax && v == '.')
        .map(|(k, _v)| k)
        .collect::<Vec<_>>()[0];
    loop {
        if frontier.len() == 0 {
            break;
        }

        let mut new_frontier = vec![];
        for pt in &frontier {
            if save_points.contains_key(pt) || pt == start_map || pt == end_map {
                // This is a "save point", a diverging path.
                ends.push(*pt);
                if ends.len() >= 2 {
                    bail = true;
                }
            } else {
                let neighbors: Vec<(usize, usize)> = get_neighbors(*pt, input, true)
                    .iter()
                    .filter(|n| !explored.contains(n))
                    .copied()
                    .collect();
                for n in neighbors {
                    if !frontier.contains(&n) {
                        explored.push(n);
                        new_frontier.push(n);
                    }
                }
            }
        }
        if bail {
            break;
        }
        last = frontier[0];
        frontier = new_frontier;
    }

    // "ends" should have 2 save_points
    for end in &ends {
        if (end.0 as i64 - start.0 as i64).abs() + (end.1 as i64 - start.1 as i64).abs() > 1 {
            last = *end;
            return (explored.len() - 1, last);
        }
    }
    println!("ends: {:?}", ends);
    println!("start: {:?}", start);
    panic!("path_length2 failed");
}

#[aoc(day23, part2)]
// 7061 is too high
pub fn part2(input: &Input) -> usize {
    // Explore the graph once keeping track of "save points"
    let save_points = find_save_points2(input);

    //print_map(&vec![], input);

    //println!("save_points.len(): {}", save_points.len());
    //println!("save_points: {:?}", save_points);
    //
    let mut path_length2_cache: HashMap<(usize, usize), (usize, (usize, usize))> = HashMap::new();
    for (sp, starts) in &save_points {
        for n in starts {
            let (length, next_dst) = path_length2(*n, &save_points, input);
            path_length2_cache.insert(*n, (length, next_dst));
        }
    }
    //println!("path_length2_cache.len(): {}", path_length2_cache.len());

    // Start path tree with the path to the first save point
    let (length, dst) = path_length2((1, 1), &save_points, input);
    let mut path_tree = Tree::with_head(((1, 0), dst, length, vec![(1, 0), dst]));

    let mut cids = vec![0];
    loop {
        if cids.len() == 0 {
            break;
        }

        let mut new_cids = vec![];
        for pid in cids {
            let dst = path_tree.nodes[pid].value.1;
            let tree_length = path_tree.nodes[pid].value.2;
            if let Some(paths) = save_points.get(&dst) {
                //println!();
                //println!("path: {:?}", path_tree.nodes[pid].value.3);
                //println!("choices: {:?}", paths);
                for start in paths {
                    //println!("start: {:?}", start);
                    //let (length, next_dst) = path_length2(*start, &save_points, input);
                    let (length, next_dst) = path_length2_cache.get(start).unwrap();
                    //println!("destination: {:?}", next_dst);
                    let mut explored = path_tree.nodes[pid].value.3.clone();

                    if !explored.contains(next_dst) {
                        // If this path hasn't gone here before, add it as a branching child
                        //println!("next_dst: {:?}", next_dst);
                        explored.push(*next_dst);
                        let cid = path_tree
                            .add_child_to_node(
                                (*start, *next_dst, tree_length + length, explored),
                                pid,
                            )
                            .unwrap();
                        new_cids.push(cid);
                    }
                }
            }
        }
        cids = new_cids;
    }

    //println!("path_tree: {:?}", path_tree);
    let ymax = input.keys().map(|(_x, y)| *y).max().unwrap();
    let end_point = input
        .iter()
        .filter(|(k, &v)| k.1 == ymax && v == '.')
        .map(|(k, _v)| k)
        .collect::<Vec<_>>()[0];
    *path_tree
        .leaf_values()
        .iter()
        .filter(|(_, dst, _, _)| dst == end_point)
        .map(|(_, _, l, _)| l)
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/23a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 94);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/23a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 154);
    }
}
