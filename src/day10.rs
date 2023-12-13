use std::collections::{HashMap, HashSet};
use aoc_helpers::graph::Graph;
use uuid::Uuid;

type Input = (HashMap<(i64, i64), (char, Uuid)>, Graph::<(i64, i64, Tile)>);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn add_edge(x: i64, y: i64, map: &HashMap<(i64, i64), (char, Uuid)>, graph: &mut Graph::<(i64, i64, Tile)>, dir: Direction) {
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
        },
        Direction::South => {
            // below
            if let Some(&(t, node2)) = map.get(&(x, y + 1)) {
                if t == '|' || t == 'L' || t == 'J' {
                    graph.add_edge(node1, node2);
                }
            }
        },
        Direction::East => {
            // right
            if let Some(&(t, node2)) = map.get(&(x + 1, y)) {
                if t == '-' || t == 'J' || t == '7' {
                    graph.add_edge(node1, node2);
                }
            }
        },
        Direction::West => {
            // left 
            if x > 0 {
                if let Some(&(t, node2)) = map.get(&(x - 1, y)) {
                    if t == '-' || t == 'L' || t == 'F' {
                        graph.add_edge(node1, node2);
                    }
                }
            }
        },
    }
}

fn add_edges_using_map(g: &mut Graph::<(i64, i64, Tile)>, map: &HashMap<(i64, i64), (char, Uuid)>) {
    // Add edges
    for (k, v) in map.iter() {
        let &(x, y) = k;
        let &(c, _) = v;
        match c {
            '|' => {
                // Vertical
                add_edge(x, y, &map, g, Direction::North);
                add_edge(x, y, &map, g, Direction::South);
            },
            '-' => {
                // Horizontal
                add_edge(x, y, &map, g, Direction::East);
                add_edge(x, y, &map, g, Direction::West);
            },
            'L' => {
                // 90 deg. bend connecting North and East
                // above
                add_edge(x, y, &map, g, Direction::North);
                add_edge(x, y, &map, g, Direction::East);
            },
            'J' => {
                // 90 deg. bend connecting North and West
                // above
                add_edge(x, y, &map, g, Direction::North);
                add_edge(x, y, &map, g, Direction::West);
            },
            '7' => {
                // 90 deg. bend connecting South and West
                // above
                add_edge(x, y, &map, g, Direction::South);
                add_edge(x, y, &map, g, Direction::West);
            },
            'F' => {
                // 90 deg. bend connecting South and East
                // above
                add_edge(x, y, &map, g, Direction::South);
                add_edge(x, y, &map, g, Direction::East);
            },
            '.' => (),
            'S' => {
                // TODO: Have to figure out what this is somehow
                add_edge(x, y, &map, g, Direction::North);
                add_edge(x, y, &map, g, Direction::South);
                add_edge(x, y, &map, g, Direction::East);
                add_edge(x, y, &map, g, Direction::West);
            },
            _ => (),
        };
    }
}

fn add_edge2(x: i64, y: i64, map: &HashMap<(i64, i64), (char, Uuid)>, graph: &mut Graph::<(i64, i64, Tile)>, dir: Direction) {
    // We just assume this is valid here
    let &(_, node1) = map.get(&(x, y)).unwrap();

    match dir {
        Direction::North => {
            // above
            if y > 0 {
                if let Some(&(t, node2)) = map.get(&(x, y - 1)) {
                    if t != '-' {
                        graph.add_edge(node1, node2);
                    }
                }
            }
        },
        Direction::South => {
            // below
            if let Some(&(t, node2)) = map.get(&(x, y + 1)) {
                if t != '-' {
                    graph.add_edge(node1, node2);
                }
            }
        },
        Direction::East => {
            // right
            if let Some(&(t, node2)) = map.get(&(x + 1, y)) {
                if t != '|' {
                    graph.add_edge(node1, node2);
                }
            }
        },
        Direction::West => {
            // left 
            if x > 0 {
                if let Some(&(t, node2)) = map.get(&(x - 1, y)) {
                    if t != '|' {
                        graph.add_edge(node1, node2);
                    }
                }
            }
        },
    }
}

fn add_edges_outside_using_map(g: &mut Graph::<(i64, i64, Tile)>, map: &HashMap<(i64, i64), (char, Uuid)>) {
    // Add edges
    for (k, v) in map.iter() {
        let &(x, y) = k;
        let &(c, _) = v;
        match c {
            '|' => {
                // Vertical
                add_edge2(x, y, &map, g, Direction::North);
                add_edge2(x, y, &map, g, Direction::South);
            },
            '-' => {
                // Horizontal
                add_edge2(x, y, &map, g, Direction::East);
                add_edge2(x, y, &map, g, Direction::West);
            },
            _ => {
                // The only ones that actually _block_ outside pipes are the H/V
                add_edge2(x, y, &map, g, Direction::North);
                add_edge2(x, y, &map, g, Direction::South);
                add_edge2(x, y, &map, g, Direction::East);
                add_edge2(x, y, &map, g, Direction::West);
            },
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
    add_edges_using_map(&mut g, &map);
    (map, g)
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> i64 {
    let (_map, graph) = input;
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
            let node = graph.get_node(*id).unwrap();
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
        explored = explored.into_iter().chain(frontier.into_iter()).collect();

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
fn extended_input(input: &Input) -> Input {
    let (map, graph) = input;

    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();

    let mut newmap = HashMap::new();
    let mut newgraph = Graph::<(i64, i64, Tile)>::new();
    for (k, v) in map {
        let node = newgraph.add_node_with_value((k.0 + 1, k.1 + 1, char_to_tile(v.0)));
        newmap.insert((k.0 + 1, k.1 + 1), (v.0, node));
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
    add_edges_using_map(&mut newgraph, &newmap);

    (newmap, newgraph)
}

fn add_nonloop_edges(graph: &mut Graph::<(i64, i64, Tile)>, nonloop_ids: &[Uuid]) {
    let mainloop_ids: Vec<_> = graph.nodes.iter().filter(|n| !nonloop_ids.contains(&n.id)).map(|n| n.id).collect();
    let xmax = graph.nodes.iter().map(|n| n.value.0).max().unwrap();
    let ymax = graph.nodes.iter().map(|n| n.value.1).max().unwrap();
    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for x in 0..xmax + 1 {
        for y in 0..ymax + 1 {
            //add_nonloop_edge();
            let node1 = graph.nodes.iter().find(|n| n.value.0 == x && n.value.1 == y).cloned().unwrap();
            if nonloop_ids.contains(&node1.id) {
                for dir in &dirs {
                    let xx = x as i64 + dir.0;
                    let yy = y as i64 + dir.1;
                    if xx >= 0 && yy >= 0 {
                        if let Some(node2) = graph.nodes.iter().find(|n| n.value.0 == xx as i64 && n.value.1 == yy as i64) {
                            if nonloop_ids.contains(&node2.id) {
                                graph.add_edge(node1.id, node2.id);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn waterfill(node1: Uuid, graph: &Graph::<(i64, i64, Tile)>) -> Vec<Uuid> {
    let mut frontier: HashSet<Uuid> = graph.get_node_neighbors(node1);
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
        explored = explored.into_iter().chain(frontier.into_iter()).collect();

        // Set `new_frontier` to `frontier` for next iteration
        frontier = new_frontier.clone();
    }
    explored.into_iter().collect()
}

fn replace_nonloop(nonloop_ids: &[Uuid], graph: &mut Graph::<(i64, i64, Tile)>) {
    for id in nonloop_ids {
        if let Some(node) = graph.get_node_mut(*id) {
            node.value.2 = Tile::G;
        }
    }
}

fn create_corners_graph(map: &HashMap<(i64, i64), (char, Uuid)>) -> Graph::<(i64, i64)> {
    // Define upper left corner of tile (0, 0) to be corner (0, 0). then:
    //
    // (0, 0) ------- (1, 0)
    //   |              |
    //   |  Tile (0, 0) |
    //   |              |
    // (0, 1) ------- (1, 1)
    let mut output = Graph::<(i64, i64)>::new();
    output.add_node_with_value((0, 0));

    // Get all the corner nodes in there
    let xmax = map.keys().map(|k| k.0).max().unwrap();
    let ymax = map.keys().map(|k| k.1).max().unwrap();
    for x in 0..xmax+2 {
        for y in 0..ymax+2 {
            output.add_node_with_value((x, y));
        }
    }

    // Pick start as (0, 0) (upper left corner)
    let start = output.nodes.iter().find(|n| n.value.0 == 0 && n.value.1 == 0).unwrap();

    let dirs: Vec<(i64, i64)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    // Fill in edges
    for (k, v) in map {
        let tile1 = v.0;
        for dir in &dirs {
            if let Some((k2, v2)) = map.get(&(k.0 + dir.0, k.1 + dir.1)) {
                node2 = output.get_node
                match tile1 {
                    Tile::G => {
                        match v2.0 {
                            '.' => output.add_edge(
                        }
                    },
                }
            }
        }
    }
    /*
    for x in 0..xmax+2 {
        for y in 0..ymax+2 {
            output.add_node_with_value((x, y));
        }
    }
    */

    output
}

// Instead of graph of centers, what about graph of corners?
#[aoc(day10, part2)]
// 860 is too high
pub fn part2(input: &Input) -> i64 {
    // Lets extend the border by 1 in all directions with '.', then start a water filling from
    // (0, 0)
    let (map, mut graph) = extended_input(input);
    print_board(&map);
    let start = graph.nodes.iter().find(|n| n.value.2 == Tile::S).unwrap();

    let mut frontier: HashSet<Uuid> = graph.get_node_neighbors(start.id);
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
        explored = explored.into_iter().chain(frontier.into_iter()).collect();

        // Set `new_frontier` to `frontier` for next iteration
        frontier = new_frontier.clone();
    }

    // Everything in `explored` at this point is part of the loop
    let not_loop: Vec<_> = graph.nodes.iter().filter(|n| {
        !explored.contains(&n.id)
    }).cloned().collect();

    /*
    {
        // DEBUG
        let not_loop_pos: Vec<_> = not_loop.iter().map(|x| (x.value.0, x.value.1)).collect();
        println!();
        print_map_board(&not_loop_pos, &map);
    }
    */

    let nonloop_ids: Vec<_> = not_loop.iter().map(|x| x.id).collect();
    replace_nonloop(&nonloop_ids, &mut graph);
    add_nonloop_edges(&mut graph, &nonloop_ids);

    let outside = graph.nodes.iter().find(|n| n.value.0 == 0 && n.value.1 == 0).unwrap();
    let exterior = waterfill(outside.id, &graph);

    let mut interior: Vec<_> = nonloop_ids.into_iter().filter(|x| !exterior.contains(&x)).collect();
    let interior_nodes: Vec<_> = interior.iter().map(|&u| graph.get_node(u).unwrap()).collect();

    {
        // DEBUG
        let pos: Vec<_> = interior_nodes.iter().map(|x| (x.value.0, x.value.1)).collect();
        println!();
        print_map_board(&pos, &map);
    }

    interior.len() as i64
}

#[allow(dead_code)]
fn print_map_board(map: &[(i64, i64)], tilemap: &HashMap<(i64, i64), (char, Uuid)>) {
    let xmax = *tilemap.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *tilemap.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..ymax+1 {
        for x in 0..xmax+1 {
            let c;
            if map.contains(&(x, y)) {
                c = '#';
            } else {
                let (cc, _) = tilemap.get(&(x, y)).unwrap();
                c = *cc;
            }
            print!("{}", c);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_board(tilemap: &HashMap<(i64, i64), (char, Uuid)>) {
    let xmax = *tilemap.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *tilemap.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..ymax+1 {
        for x in 0..xmax+1 {
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
