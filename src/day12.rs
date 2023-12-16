use rayon::prelude::*;
use std::collections::HashMap;

type Input = Vec<(String, Vec<usize>)>;

#[aoc_generator(day12)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let mut part_iter = line.split(' ');
        let arrangement = part_iter.next().unwrap().to_string();
        let right = part_iter.next().unwrap();
        let segments: Vec<usize> = right
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        output.push((arrangement, segments));
    }
    output
}

fn build_chunk(n: usize) -> String {
    let mut output = String::from(".");
    for _ in 0..n {
        output.push('#');
    }
    output.push('.');
    output
}

// Checks if every char in `arr` beginning at the nth is either `?` or matches the corresponding
// char in `seq`.
fn check_seq(n: usize, seq: &str, arr: &str) -> bool {
    arr.chars()
        .skip(n)
        .take(seq.len())
        .zip(seq.chars())
        .all(|(c1, c2)| c1 == '?' || c1 == c2)
}

fn valid_points(seq: &str, arr: &str) -> Vec<u8> {
    if seq.len() > arr.len() {
        return vec![];
    }
    let imax = arr.len() - seq.len() + 1;
    let mut output = vec![];
    for i in 0..imax {
        if check_seq(i, seq, arr) {
            output.push(i as u8);
        }
    }
    output
}

fn _solve(chunks: &[String], arr: &str, path: &[u8]) -> Vec<Vec<u8>> {
    /*
    println!("\nsolve() Call:");
    println!("chunks: {:?}", chunks);
    println!("            0123456789");
    println!("chunks[0]: {:?}", chunks[0]);
    println!("arr:       {:?}", arr);
    */
    let vpoints = valid_points(&chunks[0], &arr);
    /*
    for vp in &vpoints {
        let mut msg = String::from("            ");
        for _ in 0..*vp {
            msg.push(' ');
        }
        msg.push_str(&chunks[0]);
        println!("{}", msg);
    }
    */
    //println!("vpoints: {:?}", vpoints);
    if chunks.len() > 1 {
        vpoints
            .par_iter()
            .flat_map(|vp| {
                let this_path = vec![*vp];
                let offset = vp + chunks[0].len() as u8 - 1;
                let mut fwd_path = path.to_vec();
                fwd_path.push(*vp);
                let mut arrs = _solve(&chunks[1..], &arr[offset as usize..], &fwd_path);
                let mut intermediate = vec![];
                for a in arrs {
                    let mut tmp: Vec<u8> = this_path.clone();
                    tmp.append(&mut a.clone());
                    intermediate.push(tmp);
                }
                intermediate
            })
            .collect()
    } else {
        // No other chunks to check
        let mut output = vec![];
        for vp in vpoints {
            output.push(vec![vp]);
        }
        return output;
    }
}

fn create_sequence(chunks: &[String], path: &[u8]) -> String {
    if chunks.len() != path.len() {
        panic!("these need to be the same length");
    }
    let mut output = String::new();
    for (c, p) in chunks.iter().zip(path.iter()) {
        for i in 0..*p {
            output.push('.');
        }
        output.push_str(c);
        output.pop();
    }
    output
}

fn solve(chunks: &[String], arr: &str, path: &[u8]) -> usize {
    let maybe_paths = _solve(chunks, arr, path);

    let mut output = 0;
    //println!("Input:     {}", arr);
    for _path in maybe_paths {
        // Filter out the ones that don't fully cover the input `arr` properly.
        let candidate = create_sequence(chunks, &_path);
        //println!("candidate: {}", candidate);
        let mut good = true;
        for (i, c) in arr.chars().enumerate() {
            if c == '#' {
                // Verify candidate covers '#'s
                if candidate.chars().nth(i) != Some('#') {
                    good = false;
                }
            }
        }
        if good {
            output += 1;
        }
    }
    output
}

#[aoc(day12, part1)]
// 9888 too high
pub fn part1(input: &Input) -> usize {
    let mut output = 0;
    for (arr, seg) in input {
        println!();
        println!("#########################################################");
        println!("Next input:");
        let chunks: Vec<_> = seg.iter().map(|n| build_chunk(*n)).collect();
        println!("chunks: {:?}", chunks);
        let mut aug_arr = arr.clone();
        aug_arr.insert(0, '.');
        aug_arr.push('.');
        println!("aug_arr: {}", aug_arr);

        output += solve(&chunks, &aug_arr, &[]);

        println!();
    }
    output
}

fn unfold(arr: &str) -> String {
    let mut output = String::new();
    for i in 0..5 {
        output.push_str(arr);
        if i < 4 {
            output.push('?');
        }
    }
    output
}

#[aoc(day12, part2)]
pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|(arr, _seg)| {
            let arr = unfold(arr);
            let mut seg = vec![];
            for _ in 0..5 {
                let mut temp = _seg.clone();
                seg.append(&mut temp);
            }
            let chunks: Vec<_> = seg.iter().map(|n| build_chunk(*n)).collect();
            let mut aug_arr = arr.clone();
            aug_arr.insert(0, '.');
            aug_arr.push('.');

            let output = solve(&chunks, &aug_arr, &[]);
            println!("{}", output);
            output
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 21);

        let input = load_input(".?##?.??#.. 2,2");
        assert_eq!(part1(&input), 1);

        // TODO: Bug is not all '#' in input are getting covered by chunks
        let input = load_input("..??#?????.#??. 3,1");
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 525152);
    }
}
