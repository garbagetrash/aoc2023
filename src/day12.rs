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
    // Don't skip any '#'s
    if let Some(idx) = arr.find('#') {
        if n + 1 > idx {
            return false;
        }
    }
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

fn enumerate_idxs(chunks: &[String], arr: &str) -> usize {
    let minchunklen = chunks.iter().map(|c| c.len() - 1).sum::<usize>();
    arr.len() - minchunklen
}

fn asdf(chunks: &[String], arr: &str) -> usize {
    let space = enumerate_idxs(chunks, arr);
    let mut output = 0;

    /*
    println!();
    println!("asdf");
    println!("arr: {}", arr);
    println!("chunks: {:?}", chunks);
    println!();
    */

    for i in 0..space {
        //println!("i: {}", i);
        if chunks.len() == 1 {
            // Last chunk means start counting
            if check_seq(i, &chunks[0], &arr) {
                // Now make sure we're covering all _remaining_ '#'s as well...
                if check_remaining_hashes(i, &chunks[0], &arr) {
                    output += 1;
                    //println!("hit");
                }
            }
        } else {
            // More than 1 chunk means sum up all valid combos for each chunk index
            if check_seq(i, &chunks[0], &arr) {
                // If we get here then _this_ chunk is valid, so what about chunks farther on?
                let offset = i + chunks[0].len() - 1;
                output += asdf(&chunks[1..], &arr[offset..]);
            }
        }
    }
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
            let value = asdf(&chunks, &aug_arr);
            /*
            println!("arr:      {}", arr);
            println!("aug_arr:  {}", aug_arr);
            println!("chunks: {:?}", chunks);
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

#[aoc(day12, part2)]
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
            let value = asdf(&chunks, &aug_arr);
            /*
            println!("arr:      {}", arr);
            println!("aug_arr:  {}", aug_arr);
            println!("chunks: {:?}", chunks);
            println!("value: {}", value);
            println!();
            println!("##################################################################");
            println!();
            */
            //cntr += 1;
            //println!("cntr: {}", cntr);
            println!("asdf");
            value
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
