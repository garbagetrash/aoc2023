use std::io::stdin;
use std::io::Read;
use rayon::prelude::*;

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

// A "chunk" is some number of '#'s with a '.' on both sides.
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
// NOTE: This is called like a million-billion times, any speedups here will payoff greatly.
fn check_seq(n: usize, seq: &str, arr: &str) -> bool {
    arr.chars()
        .skip(n)
        .take(seq.len())
        .zip(seq.chars())
        .all(|(c1, c2)| c1 == '?' || c1 == c2)
}

fn check_remaining_hashes(n: usize, seq: &str, arr: &str) -> bool {
    // Don't skip any '#'s
    if let Some(ridx) = arr.rfind('#') {
        let last_hash_idx = n + seq.len() - 2;
        if last_hash_idx < ridx {
            return false;
        }
    }
    true
}

// The concept here is that a chunk of #'s will never be a valid match on leading '.', only on '?'
// or '#', so just skip to those.
fn start_lower_bound(arr: &str) -> usize {
    if let Some(idx) = arr.find(&['?', '#']) {
        if idx > 0 {
            return idx - 1;
        } else {
            return 0;
        }
    }
    arr.len() - 1
}

// This function is calculating an upper bound on the number of spaces we actually have to scan by
// looking at the length of the totall array less the length of the sum of the chunks.
fn enumerate_idxs(chunks: &[String], arr: &str) -> usize {
    let minchunklen = chunks.iter().map(|c| c.len()).sum::<usize>() - (chunks.len() - 1);
    let mut output = arr.len() - minchunklen + 1;
    if let Some(first_hash) = arr.find('#') {
        // Can't skip any explicit '#'s, so this is an upper bound when present.
        if first_hash < output {
            output = first_hash;
        }
    }
    output
}

fn solve(chunks: &[String], arr: &str) -> usize {

    // Get a lower bound on the starting index.
    let i_lower = start_lower_bound(arr);

    // This grabs an upper bound of the number of spaces we need to check.
    let space = enumerate_idxs(chunks, arr);

    /*
    println!("i_lower: {}", i_lower);
    println!("space: {}", space);
    */

    let output = (i_lower..space).into_par_iter().map(|i| {
        if chunks.len() == 1 {
            // This is the last chunk, is _must_ catch the '#' detected by `i_lower` so we only have to
            // do 1 check here else it's no good. Also it must cover the _remaining_ '#'s as well.
            if check_seq(i, &chunks[0], &arr) && check_remaining_hashes(i, &chunks[0], &arr) {
                1
            } else {
                0
            }
        } else {
            // More than 1 chunk means sum up all valid combos for each chunk index
            if check_seq(i, &chunks[0], &arr) {
                // If we get here then _this_ chunk is valid, so what about chunks farther on?
                let offset = i + chunks[0].len() - 1;
                solve(&chunks[1..], &arr[offset..])
            } else {
                0
            }
        }
    }).sum();
    //println!("return {}", output);
    output
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(arr, seg)| {
            let chunks: Vec<_> = seg.iter().map(|n| build_chunk(*n)).collect();
            let mut aug_arr = arr.clone();
            aug_arr.insert(0, '.');
            aug_arr.push('.');
            /*
            println!();
            println!("##################################################################");
            println!();
            println!("arr:      {}", arr);
            println!("aug_arr:  {}", aug_arr);
            println!("chunks: {:?}", chunks);
            */
            let value = solve(&chunks, &aug_arr);
            /*
            println!("value: {}", value);
            println!();
            println!("##################################################################");
            println!();
            */
            value
        })
        .sum()
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

fn solve2(chunks: &[String], arr: &str) -> usize {
    let first = solve(chunks, arr);

    let minchunklen = chunks.iter().map(|c| c.len()).sum::<usize>() - (chunks.len() - 1);
    if arr.chars().nth(1).unwrap() == '#' || arr.chars().nth(arr.len() - 2).unwrap() == '#' {
        if minchunklen == arr.len() {
            return first.pow(5);
        }
    }

    let mut alt0 = arr.to_string();
    alt0.insert(arr.len() - 2, '?');

    let mut alt1 = arr.to_string();
    alt1.insert(1, '?');

    let alt0_cnt = solve(chunks, &alt0);
    let alt1_cnt = solve(chunks, &alt1);
    if alt0_cnt > alt1_cnt {
        first * alt0_cnt.pow(4)
    } else {
        first * alt1_cnt.pow(4)
    }
}

#[aoc(day12, part2)]
// WARNING: > 12 hour runtime.
pub fn part2(input: &Input) -> usize {
    let mut cntr = 0;
    input
        .par_iter()
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
            let value = solve(&chunks, &aug_arr);
            value
        })
        .sum()
    /*
    input
        .iter()
        .map(|(arr, seg)| {
            let chunks: Vec<_> = seg.iter().map(|n| build_chunk(*n)).collect();
            let mut aug_arr = arr.clone();
            aug_arr.insert(0, '.');
            aug_arr.push('.');
            let value = solve2(&chunks, &aug_arr);
            value
        })
        .sum()
    */
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = load_input("???.### 1,1,3");
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 21);

        let input = load_input(".?##?.??#.. 2,2");
        assert_eq!(part1(&input), 1);

        let input = load_input("..??#?????.#??. 3,1");
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_second() {
        let input = load_input("???.### 1,1,3");
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 525152);
    }
}
