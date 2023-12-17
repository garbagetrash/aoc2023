use aoc_helpers::graph::Graph;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

type Input = (HashMap<(i64, i64), (char, Uuid)>, Graph<(i64, i64, Tile)>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn add_edge(
    x: i64,
    y: i64,
    map: &HashMap<(i64, i64), (char, Uuid)>,
    graph: &mut Graph<(i64, i64, Tile)>,
    dir: Direction,
) {
    // We just assume this is valid here
    let &(_, node1) = map.get(&(x, y)).unwrap();

    match dir {
        Direction::North => {
            // above
            if y > 0 {
                if let Some(&(t, node2)) = map.get(&(x, y - 1)) {
                    if t == '|' || t == '7' || t == 'F' {
                        graph.add_edge(node1, node2);
                    }
                }
            }
        }
        Direction::South => {
            // below
            if let Some(&(t, node2)) = map.get(&(x, y + 1)) {
                if t == '|' || t == 'L' || t == 'J' {
                    graph.add_edge(node1, node2);
                }
            }
        }
        Direction::East => {
            // right
            if let Some(&(t, node2)) = map.get(&(x + 1, y)) {
                if t == '-' || t == 'J' || t == '7' {
                    graph.add_edge(node1, node2);
                }
            }
        }
        Direction::West => {
            // left
            if x > 0 {
                if let Some(&(t, node2)) = map.get(&(x - 1, y)) {
                    if t == '-' || t == 'L' || t == 'F' {
                        graph.add_edge(node1, node2);
                    }
                }
            }
        }
    }
}

fn add_edge_corner(
    x: i64,
    y: i64,
    map: &HashMap<(i64, i64), (char, Uuid)>,
    graph: &mut Graph<(i64, i64, Tile)>,
    dir: Direction,
) {
    // We just assume this is valid here
    let &(_, node1) = map.get(&(x, y)).unwrap();

    match dir {
        // F-7
        // |.|
        // L-J
        Direction::North => {
            // above
            if y > 0 {
                if let Some(&(t, node2)) = map.get(&(x, y - 1)) {
                    if t == '|' || t == 'L' || t == 'F' || t == '.' {
                        graph.add_edge(node1, node2);
                    }
                }
            }
        }
        Direction::South => {
            // below
            if let Some(&(t, node2)) = map.get(&(x, y + 1)) {
                if t == '|' || t == 'L' || t == 'F' || t == '.' {
                    graph.add_edge(node1, node2);
                }
            }
        }
        Direction::East => {
            // right
            if let Some(&(t, node2)) = map.get(&(x + 1, y)) {
                if t == '-' || t == 'F' || t == '7' || t == '.' {
                    graph.add_edge(node1, node2);
                }
            }
        }
        Direction::West => {
            // left
            if x > 0 {
                if let Some(&(t, node2)) = map.get(&(x - 1, y)) {
                    if t == '-' || t == '7' || t == 'F' || t == '.' {
                        graph.add_edge(node1, node2);
                    }
                }
            }
        }
    }
}

fn add_edges_using_map(g: &mut Graph<(i64, i64, Tile)>, map: &HashMap<(i64, i64), (char, Uuid)>) {
    // Add edges
    for (k, v) in map.iter() {
        let &(x, y) = k;
        let &(c, _) = v;
        match c {
            '|' => {
                // Vertical
                add_edge(x, y, map, g, Direction::North);
                add_edge(x, y, map, g, Direction::South);
            }
            '-' => {
                // Horizontal
                add_edge(x, y, map, g, Direction::East);
                add_edge(x, y, map, g, Direction::West);
            }
            'L' => {
                // 90 deg. bend connecting North and East
                // above
                add_edge(x, y, map, g, Direction::North);
                add_edge(x, y, map, g, Direction::East);
            }
            'J' => {
                // 90 deg. bend connecting North and West
                // above
                add_edge(x, y, map, g, Direction::North);
                add_edge(x, y, map, g, Direction::West);
            }
            '7' => {
                // 90 deg. bend connecting South and West
                // above
                add_edge(x, y, map, g, Direction::South);
                add_edge(x, y, map, g, Direction::West);
            }
            'F' => {
                // 90 deg. bend connecting South and East
                // above
                add_edge(x, y, map, g, Direction::South);
                add_edge(x, y, map, g, Direction::East);
            }
            '.' => (),
            'S' => {
                // TODO: Have to figure out what this is somehow
                add_edge(x, y, map, g, Direction::North);
                add_edge(x, y, map, g, Direction::South);
                add_edge(x, y, map, g, Direction::East);
                add_edge(x, y, map, g, Direction::West);
            }
            _ => (),
        };
    }
}

fn add_edges_using_map_corners(
    g: &mut Graph<(i64, i64, Tile)>,
    map: &HashMap<(i64, i64), (char, Uuid)>,
) {
    // Add edges
    for (k, v) in map.iter() {
        let &(x, y) = k;
        let &(c, _) = v;
        // F-7
        // | |
        // L-J
        match c {
            '|' => {
                // Vertical
                add_edge_corner(x, y, map, g, Direction::North);
                add_edge_corner(x, y, map, g, Direction::South);
                add_edge_corner(x, y, map, g, Direction::West);
            }
            '-' => {
                // Horizontal
                add_edge_corner(x, y, map, g, Direction::North);
                add_edge_corner(x, y, map, g, Direction::East);
                add_edge_corner(x, y, map, g, Direction::West);
            }
            'L' => {
                // 90 deg. bend connecting North and East
                // above
                add_edge_corner(x, y, map, g, Direction::North);
                add_edge_corner(x, y, map, g, Direction::South);
                add_edge_corner(x, y, map, g, Direction::West);
            }
            'J' => {
                // 90 deg. bend connecting North and West
                // above
                add_edge_corner(x, y, map, g, Direction::North);
                add_edge_corner(x, y, map, g, Direction::West);
            }
            '7' => {
                // 90 deg. bend connecting South and West
                // above
                add_edge_corner(x, y, map, g, Direction::North);
                add_edge_corner(x, y, map, g, Direction::West);
                add_edge_corner(x, y, map, g, Direction::East);
            }
            'F' => {
                // 90 deg. bend connecting South and East
                // above
                add_edge_corner(x, y, map, g, Direction::North);
                add_edge_corner(x, y, map, g, Direction::South);
                add_edge_corner(x, y, map, g, Direction::West);
                add_edge_corner(x, y, map, g, Direction::East);
            }
            '.' => {
                add_edge_corner(x, y, map, g, Direction::North);
                add_edge_corner(x, y, map, g, Direction::South);
                add_edge_corner(x, y, map, g, Direction::East);
                add_edge_corner(x, y, map, g, Direction::West);
            }
            'S' => {
                // TODO: Have to figure out what this is somehow
            }
            _ => (),
        };
    }
}

#[aoc_generator(day10)]
pub fn load_input(input: &str) -> Input {
    let mut map: HashMap<(i64, i64), (char, Uuid)> = HashMap::new();
    let mut g = Graph::<(i64, i64, Tile)>::new();

    // Add nodes
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let maybe_node = match c {
                '|' => Some(g.add_node_with_value((x as i64, y as i64, Tile::V))),
                '-' => Some(g.add_node_with_value((x as i64, y as i64, Tile::H))),
                'L' => Some(g.add_node_with_value((x as i64, y as i64, Tile::NE))),
                'J' => Some(g.add_node_with_value((x as i64, y as i64, Tile::NW))),
                '7' => Some(g.add_node_with_value((x as i64, y as i64, Tile::SW))),
                'F' => Some(g.add_node_with_value((x as i64, y as i64, Tile::SE))),
                '.' => Some(g.add_node_with_value((x as i64, y as i64, Tile::G))),
                'S' => Some(g.add_node_with_value((x as i64, y as i64, Tile::S))),
                _ => None,
            };
            if let Some(node) = maybe_node {
                map.insert((x as i64, y as i64), (c, node));
            }
        }
    }
    (map, g)
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> i64 {
    let map = input.0.clone();
    let mut graph = input.1.clone();
    add_edges_using_map(&mut graph, &map);
    let start = graph.nodes.iter().find(|n| n.value.2 == Tile::S).unwrap();

    let mut frontier: HashSet<Uuid> = graph.get_node_neighbors(start.id);
    let mut explored: HashSet<Uuid> = HashSet::new();
    let mut distmap: HashMap<(i64, i64), i64> = HashMap::new();
    let mut dist = 1;
    loop {
        if frontier.is_empty() {
            break;
        }
        let mut new_frontier = HashSet::new();
        for id in &frontier {
            let node = graph.get_node_from_id(*id).unwrap();
            distmap.insert((node.value.0, node.value.1), dist);

            // While we're here, search neighbors of this frontier node for _new_ frontier
            for nid in &graph.get_node_neighbors(*id) {
                if !explored.contains(nid) && !frontier.contains(nid) {
                    // If `nid` not in `explored` or current `frontier` then it is a hit
                    new_frontier.insert(*nid);
                }
            }
        }
        dist += 1;

        // Move old `frontier` into `explored`
        explored = explored.into_iter().chain(frontier).collect();

        // Set `new_frontier` to `frontier` for next iteration
        frontier = new_frontier.clone();
    }

    *distmap.values().max().unwrap()
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '|' => Tile::V,
        '-' => Tile::H,
        'L' => Tile::NE,
        'J' => Tile::NW,
        '7' => Tile::SW,
        'F' => Tile::SE,
        '.' => Tile::G,
        'S' => Tile::S,
        _ => panic!("Invalid char"),
    }
}

// Draw '.' all around border of original input like:
// ..............
// . Input Here .
// ..............
fn extended_input(input: &Input, maybe_main_loop: Option<Vec<(i64, i64)>>) -> Input {
    let (map, _) = input;

    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();

    let mut newmap = HashMap::new();
    let mut newgraph = Graph::<(i64, i64, Tile)>::new();
    for (k, v) in map {
        if let Some(main_loop) = maybe_main_loop.clone() {
            if main_loop.contains(&(k.0 + 1, k.1 + 1)) {
                let node = newgraph.add_node_with_value((k.0 + 1, k.1 + 1, char_to_tile(v.0)));
                newmap.insert((k.0 + 1, k.1 + 1), (v.0, node));
            } else {
                // If not part of the main_loop just replace it with '.'
                let node = newgraph.add_node_with_value((k.0 + 1, k.1 + 1, Tile::G));
                newmap.insert((k.0 + 1, k.1 + 1), ('.', node));
            }
        } else {
            let node = newgraph.add_node_with_value((k.0 + 1, k.1 + 1, char_to_tile(v.0)));
            newmap.insert((k.0 + 1, k.1 + 1), (v.0, node));
        }
    }

    // Northern edge
    for x in 0..xmax + 3 {
        let node = newgraph.add_node_with_value((x, 0, Tile::G));
        newmap.insert((x, 0), ('.', node));
    }

    // Southern edge
    for x in 0..xmax + 3 {
        let node = newgraph.add_node_with_value((x, ymax + 2, Tile::G));
        newmap.insert((x, ymax + 2), ('.', node));
    }

    // Western edge
    for y in 1..ymax + 2 {
        let node = newgraph.add_node_with_value((0, y, Tile::G));
        newmap.insert((0, y), ('.', node));
    }

    // Eastern edge
    for y in 1..ymax + 2 {
        let node = newgraph.add_node_with_value((xmax + 2, y, Tile::G));
        newmap.insert((xmax + 2, y), ('.', node));
    }

    // Add back in the edges
    if maybe_main_loop.is_some() {
        add_edges_using_map_corners(&mut newgraph, &newmap);
    } else {
        add_edges_using_map(&mut newgraph, &newmap);
    }
    (newmap, newgraph)
}

// Instead of graph of centers, what about graph of corners?
#[aoc(day10, part2)]
pub fn part2(input: &Input) -> i64 {
    // Solve part 1 to get the loop...
    let main_loop: Vec<_>;
    {
        let (_, graph) = extended_input(input, None);
        let start = graph.nodes.iter().find(|n| n.value.2 == Tile::S).unwrap();

        let mut frontier: HashSet<Uuid> = graph.get_node_neighbors(start.id);
        let mut explored: HashSet<Uuid> = HashSet::new();
        let mut distmap: HashMap<(i64, i64), i64> = HashMap::new();
        let mut dist = 1;
        loop {
            if frontier.is_empty() {
                break;
            }
            let mut new_frontier = HashSet::new();
            for id in &frontier {
                let node = graph.get_node_from_id(*id).unwrap();
                distmap.insert((node.value.0, node.value.1), dist);

                // While we're here, search neighbors of this frontier node for _new_ frontier
                for nid in &graph.get_node_neighbors(*id) {
                    if !explored.contains(nid) && !frontier.contains(nid) {
                        // If `nid` not in `explored` or current `frontier` then it is a hit
                        new_frontier.insert(*nid);
                    }
                }
            }
            dist += 1;

            // Move old `frontier` into `explored`
            explored = explored.into_iter().chain(frontier).collect();

            // Set `new_frontier` to `frontier` for next iteration
            frontier = new_frontier.clone();
        }
        // Verify part 1 still good
        main_loop = explored
            .iter()
            .map(|u| {
                let node = graph.get_node_from_id(*u).unwrap();
                (node.value.0, node.value.1)
            })
            .collect();
        /*
        println!("Main Loop:");
        print_map_board(&main_loop, &map);
        println!();
        */
    }

    // Lets extend the border by 1 in all directions with '.', then start a water filling from
    // (0, 0)
    let (map, graph) = extended_input(input, Some(main_loop.clone()));
    /*
    println!("Extended Input:");
    print_board(&map);
    println!();
    */
    let start = graph
        .nodes
        .iter()
        .find(|n| n.value.0 == 0 && n.value.1 == 0)
        .unwrap();

    //println!("graph: {:?}", graph.edges);
    let mut frontier: HashSet<Uuid> = graph.get_node_neighbors(start.id);
    //println!("start neighbors: {:?}", frontier);
    let mut explored: HashSet<Uuid> = HashSet::new();
    loop {
        //println!("frontier.len(): {}", frontier.len());
        if frontier.is_empty() {
            break;
        }
        let mut new_frontier = HashSet::new();
        for id in &frontier {
            // Search neighbors of this frontier node for _new_ frontier
            for nid in &graph.get_node_neighbors(*id) {
                if !explored.contains(nid) && !frontier.contains(nid) {
                    // If `nid` not in `explored` or current `frontier` then it is a hit
                    new_frontier.insert(*nid);
                }
            }
        }

        // Move old `frontier` into `explored`
        explored = explored.into_iter().chain(frontier).collect();

        // Set `new_frontier` to `frontier` for next iteration
        frontier = new_frontier.clone();
        /*
        {
            let asdf: Vec<_> = explored
                .iter()
                .map(|u| {
                    let node = graph.get_node_from_id(*u).unwrap();
                    (node.value.0, node.value.1)
                })
                .collect();
            // DEBUG
            println!("Exterior Map:");
            print_map_board(&asdf, &map);
            println!();
        }
        */
    }

    let asdf: Vec<_> = explored
        .iter()
        .map(|u| {
            let node = graph.get_node_from_id(*u).unwrap();
            (node.value.0, node.value.1)
        })
        .collect();
    /*
    {
        // DEBUG
        println!("Exterior Map:");
        print_map_board(&asdf, &map);
        println!();
    }
    */

    let main_loop: HashSet<(i64, i64)> = main_loop.into_iter().collect();
    let exterior: HashSet<(i64, i64)> = asdf.into_iter().collect();

    let answer_complement: Vec<_> = main_loop.union(&exterior).copied().collect();
    /*
    {
        // DEBUG
        println!("Union Map:");
        print_map_board(&answer_complement, &map);
        println!();
    }
    */

    let xmax = map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = map.keys().map(|(_, y)| y).max().unwrap();

    (xmax + 1) * (ymax + 1) - answer_complement.len() as i64
}

#[allow(dead_code)]
fn print_map_board(map: &[(i64, i64)], tilemap: &HashMap<(i64, i64), (char, Uuid)>) {
    let xmax = *tilemap.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *tilemap.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..ymax + 1 {
        for x in 0..xmax + 1 {
            let mut color = false;
            if map.contains(&(x, y)) {
                color = true;
            }
            let (cc, _) = tilemap.get(&(x, y)).unwrap();
            if color {
                print!("\x1b[91m{}\x1b[0m", *cc);
            } else {
                print!("{}", *cc);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_board(tilemap: &HashMap<(i64, i64), (char, Uuid)>) {
    let xmax = *tilemap.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *tilemap.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..ymax + 1 {
        for x in 0..xmax + 1 {
            let (c, _) = tilemap.get(&(x, y)).unwrap();
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/10a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 4);

        let input = read_to_string("input/2023/10b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/10c.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 4);

        let input = read_to_string("input/2023/10d.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 4);

        let input = read_to_string("input/2023/10e.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 8);

        let input = read_to_string("input/2023/10f.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 10);
    }
}
