use std::collections::HashMap;

type Input = Vec<HashMap<(i64, i64), char>>;

#[aoc_generator(day13)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    let mut map = HashMap::new();
    let mut y = 0;
    for line in input.lines() {
        if line.is_empty() {
            output.push(map);
            map = HashMap::new();
            y = 0;
        } else {
            for (x, c) in line.chars().enumerate() {
                map.insert((x as i64, y as i64), c);
            }
            y += 1;
        }
    }
    output.push(map);
    output
}

fn find_reflect_x(map: &HashMap<(i64, i64), char>) -> Option<i64> {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let mut candidates = vec![];
    for x in 0..xmax {
        let mut bail = false;
        for y in 0..ymax + 1 {
            let c1 = map.get(&(x, y));
            let c2 = map.get(&(x + 1, y));
            if c1 != c2 {
                bail = true;
                break;
            }
        }
        if !bail {
            candidates.push(x);
        }
    }

    // At this point we have _candidates_ for total reflections by just checking a single pair of
    // columns. Lets run them through the whole test.
    //println!("x-reflect candidates: {:?}", candidates);

    for cand in candidates {
        let mut bail = false;
        let mut minx = cand + 1;
        if xmax - cand < minx {
            minx = xmax - cand;
        }

        for i in 0..minx {
            let x1 = cand - i;
            let x2 = cand + i + 1;
            for y in 0..ymax + 1 {
                let c1 = map.get(&(x1, y));
                let c2 = map.get(&(x2, y));
                if c1 != c2 {
                    bail = true;
                    break;
                }
            }
            if bail {
                break;
            }
        }
        if !bail {
            return Some(cand);
        }
    }
    None
}

fn find_reflect_y(map: &HashMap<(i64, i64), char>) -> Option<i64> {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let mut candidates = vec![];
    for y in 0..ymax {
        let mut bail = false;
        for x in 0..xmax + 1 {
            let c1 = map.get(&(x, y));
            let c2 = map.get(&(x, y + 1));
            if c1 != c2 {
                bail = true;
                break;
            }
        }
        if !bail {
            candidates.push(y);
        }
    }

    // At this point we have _candidates_ for total reflections by just checking a single pair of
    // columns. Lets run them through the whole test.
    //println!("y-reflect candidates: {:?}", candidates);

    for cand in candidates {
        let mut bail = false;
        let mut miny = cand + 1;
        if ymax - cand < miny {
            miny = ymax - cand;
        }

        for i in 0..miny {
            let y1 = cand - i;
            let y2 = cand + i + 1;
            for x in 0..xmax + 1 {
                let c1 = map.get(&(x, y1));
                let c2 = map.get(&(x, y2));
                if c1 != c2 {
                    //println!("y1: {}, y2: {}", y1, y2);
                    bail = true;
                    break;
                }
            }
            if bail {
                break;
            }
        }
        if !bail {
            return Some(cand);
        }
    }
    None
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut output = 0;
    for map in input {
        let mut done = false;
        if let Some(xvalue) = find_reflect_x(map) {
            //println!("xvalue: {:?}", xvalue);
            output += xvalue + 1;
            done = true;
        }
        if let Some(yvalue) = find_reflect_y(map) {
            //println!("yvalue: {:?}", yvalue);
            output += 100 * (yvalue + 1);
            done = true;
        }

        if !done {
            println!();
            print_map(map);
        }
    }
    output
}

fn calc_metric_x(x: i64, map: &HashMap<(i64, i64), char>) -> i64 {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let mut metric = 0;

    let mut steps = x + 1;
    if xmax - x < steps {
        steps = xmax - x;
    }

    for i in 0..steps {
        for y in 0..ymax + 1 {
            let c1 = map.get(&(x - i, y));
            let c2 = map.get(&(x + i + 1, y));
            if c1 != c2 {
                metric += 1;
            }
        }
    }
    metric
}

fn calc_metric_y(y: i64, map: &HashMap<(i64, i64), char>) -> i64 {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let mut metric = 0;

    let mut steps = y + 1;
    if ymax - y < steps {
        steps = ymax - y;
    }

    for i in 0..steps {
        for x in 0..xmax + 1 {
            let c1 = map.get(&(x, y - i));
            let c2 = map.get(&(x, y + i + 1));
            if c1 != c2 {
                metric += 1;
            }
        }
    }
    metric
}

fn x_metrics(map: &HashMap<(i64, i64), char>) -> Vec<i64> {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    (0..xmax).map(|x| calc_metric_x(x, map)).collect()
}

fn y_metrics(map: &HashMap<(i64, i64), char>) -> Vec<i64> {
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    (0..ymax).map(|y| calc_metric_y(y, map)).collect()
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut output = 0;
    for map in input {
        let xmetrics = x_metrics(map);
        //println!("xmetrics: {:?}", xmetrics);
        let ymetrics = y_metrics(map);
        //println!("ymetrics: {:?}", ymetrics);

        // TODO: Create newmap OR maybe just cheat a bit...?
        let mut done = false;
        for (xx, x) in xmetrics.iter().enumerate() {
            if *x == 1 {
                //println!("xx: {}", xx);
                output += xx as i64 + 1;
                done = true;
            }
        }
        for (yy, y) in ymetrics.iter().enumerate() {
            if *y == 1 {
                //println!("yy: {}", yy);
                output += 100 * (yy as i64 + 1);
                done = true;
            }
        }

        if !done {
            println!();
            print_map(map);
        }
    }
    output
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(i64, i64), char>) {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..ymax + 1 {
        for x in 0..xmax + 1 {
            let c = map.get(&(x, y)).unwrap();
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
        let input = read_to_string("input/2023/13a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 405);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/13a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 400);
    }
}
