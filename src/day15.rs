type Input = String;

#[aoc_generator(day15)]
pub fn load_input(input: &str) -> Input {
    String::from(input)
}

fn hash(token: &str) -> usize {
    let mut value: usize = 0;
    for c in token.bytes() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> usize {
    input.trim_end().split(',').map(hash).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    let tokens: Vec<_> = input.trim_end().split(',').collect();

    for t in tokens {
        if t.ends_with(&['-']) {
            // Remove lens
            let sides: Vec<_> = t.split('-').collect();
            let label = sides[0].to_string();
            let boxidx = hash(&label);
            let mut replace_idx = None;
            for (i, (l, _)) in boxes[boxidx].iter().enumerate() {
                if l == &label {
                    replace_idx = Some(i);
                }
            }
            if let Some(ridx) = replace_idx {
                boxes[boxidx].remove(ridx);
            }
        } else {
            // Place/replace lens
            let sides: Vec<_> = t.split('=').collect();
            let label = sides[0].to_string();
            let boxidx = hash(&label);
            let value = sides[1].parse::<usize>().unwrap();
            let contents = (label.clone(), value);

            let mut replace_idx = None;
            for (i, (l, _)) in boxes[boxidx].iter().enumerate() {
                if l == &label {
                    replace_idx = Some(i);
                }
            }

            if let Some(ridx) = replace_idx {
                // Replace
                if let Some(cont) = boxes[boxidx].get_mut(ridx) {
                    *cont = contents;
                }
            } else {
                // Emplace
                boxes[boxidx].push(contents);
            }
        }
    }

    let mut lenses = vec![];
    for (i, contents) in boxes.iter().enumerate() {
        let num1 = i + 1;
        for (j, cont) in contents.iter().enumerate() {
            lenses.push(num1 * (j + 1) * cont.1);
        }
    }
    lenses.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/15a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/15a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 145);
    }
}
