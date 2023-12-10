use std::cmp::Ordering;

type Input = Vec<(Hand, i64)>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand {
    hand: String,
    hand_value1: i64,
    hand_value2: i64,
}

impl Hand {
    pub fn new(hand: &str) -> Self {
        Self {
            hand: hand.to_string(),
            hand_value1: determine_hand(hand, false),
            hand_value2: determine_hand(hand, true),
        }
    }
}

// 13 cards
// 7 hand types

fn card_to_num(card: char, part2: bool) -> i64 {
    let mut value = match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    };
    if part2 && value == 10 {
        value = 0;
    }
    value
}

fn first_greater(first: &str, second: &str, part2: bool) -> bool {
    for (f, s) in first.chars().zip(second.chars()) {
        let v1 = card_to_num(f, part2);
        let v2 = card_to_num(s, part2);
        match v1.cmp(&v2) {
            Ordering::Greater => return true,
            Ordering::Less => return false,
            _ => (),
        }
    }
    println!("hand1: {}", first);
    println!("hand2: {}", second);
    panic!("hands equal!");
}

// 7 - 5 of a kind
// 6 - 4 of a kind
// 5 - full house
// 4 - 3 of a kind
// 3 - 2 pair
// 2 - 1 pair
// 1 - high card
fn determine_hand(hand: &str, part2: bool) -> i64 {
    // If part 2, turn jokers into other stuff recursively
    if part2 {
        let hands = swap_jokers(hand);
        let mut bestvalue = determine_hand(&hands[0], false);
        for hand in &hands[1..] {
            let v = determine_hand(hand, false);
            if v > bestvalue {
                bestvalue = v;
            }
        }
        return bestvalue;
    }

    // `values` is count of each of:
    // [A, K, Q, J, T, 9, 8, ... 2]
    // in a given hand
    let mut values: Vec<usize> = vec![0; 13];
    for c in hand.chars() {
        let idx = card_to_num(c, false) as usize - 1;
        values[idx] += 1;
    }

    let n = values.iter().filter(|&x| *x > 0).count();

    if n == 1 {
        // Five of a kind
        7
    } else if n == 2 {
        // Full house or 4 of a kind
        let cnt1 = *values.iter().find(|&&x| x > 0).unwrap();
        if cnt1 == 4 || cnt1 == 1 {
            6
        } else {
            5
        }
    } else if n == 3 {
        // 3 of a kind or 2 pair
        if values.iter().any(|&x| x == 3) {
            // 3 of a kind
            4
        } else {
            // 2 of a kind
            3
        }
    } else if n == 4 {
        // 1 pair
        2
    } else {
        // High card
        1
    }
}

fn compare_hands(first: &Hand, second: &Hand, part2: bool) -> Ordering {
    let mut h1 = first.hand_value1;
    let mut h2 = second.hand_value1;
    if part2 {
        h1 = first.hand_value2;
        h2 = second.hand_value2;
    }
    match h1.cmp(&h2) {
        Ordering::Equal => {
            if first_greater(&first.hand, &second.hand, part2) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    }
}

fn swap_jokers(hand: &str) -> Vec<String> {
    let mut output: Vec<String> = vec![hand.to_string()];
    loop {
        let mut next_idx = -1;
        let mut new = vec![];
        for (i, h) in output.iter().enumerate() {
            let mut bail = false;
            if let Some(idx) = h.chars().position(|c| c == 'J') {
                new = swap_jokers_work(h, idx);
                next_idx = i as i64;
                bail = true;
            }
            if bail {
                break;
            }
        }
        if next_idx == -1 {
            break;
        } else {
            output.remove(next_idx as usize);
            output.append(&mut new);
        }
    }
    output
}

fn swap_jokers_work(hand: &str, idx: usize) -> Vec<String> {
    if hand.contains('J') {
        let cards = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
        let mut output = vec![];
        let c = hand.chars().nth(idx).unwrap();
        if c == 'J' {
            for card in &cards {
                let mut temphand = hand.to_string();
                temphand.replace_range(idx..idx + 1, &card.to_string());
                output.push(temphand);
            }
        }
        output
    } else {
        vec![hand.to_string()]
    }
}

#[aoc_generator(day7)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let derp: Vec<_> = line.split(' ').collect();
        output.push((Hand::new(derp[0]), derp[1].parse::<i64>().unwrap()));
    }
    output
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut output = 0;
    let mut hands: Vec<_> = input.iter().map(|x| x.0.clone()).collect();
    hands.sort_by(|a, b| compare_hands(a, b, false));
    for (hand, bid) in input {
        let rank = hands.iter().position(|x| x == hand).unwrap() as i64 + 1;
        output += rank * bid;
    }
    output
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut output = 0;
    let mut hands: Vec<_> = input.iter().map(|x| x.0.clone()).collect();
    hands.sort_by(|a, b| compare_hands(a, b, true));
    for (hand, bid) in input {
        let rank = hands.iter().position(|x| x == hand).unwrap() as i64 + 1;
        output += rank * bid;
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/07a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/07a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 5905);
    }
}
