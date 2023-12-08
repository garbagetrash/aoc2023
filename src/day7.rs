use std::collections::{HashMap, HashSet, VecDeque};
use scan_fmt::scan_fmt;
use std::cmp::Ordering;

type Input = Vec<(Hand, i64)>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand {
    hand: String,
}

// 13 cards
// 7 hand types

fn card_to_num(card: char) -> i64 {
    match card {
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
    }
}

fn hand_value(hand: &str) -> i64 {
    let mut output = 0;
    for (i, card) in hand.chars().enumerate() {
        let value = card_to_num(card);
        output += 14_i64.pow(4 - i as u32) * value;
    }
    output
}

fn hand_value2(hand: &str) -> i64 {
    let mut output = 0;
    for (i, card) in hand.chars().enumerate() {
        let mut value = card_to_num(card);
        if value == 10 {
            value = 0;
        }
        output += 14_i64.pow(4 - i as u32) * value;
    }
    output
}

// 7 - 5 of a kind
// 6 - 4 of a kind
// 5 - full house
// 4 - 3 of a kind
// 3 - 2 pair
// 2 - 1 pair
// 1 - high card
fn determine_hand(hand: &str, part2: bool) -> i64 {
    let mut values: HashMap<char, i64> = HashMap::new();
    for c in hand.chars() {
        if let Some(num) = values.get_mut(&c) {
            *num += 1;
        } else {
            values.insert(c, 1);
        }
    }

    if values.len() == 1 {
        // Five of a kind
        7
    } else if values.len() == 2 {
        // Full house or 4 of a kind
        let cnt1 = *values.iter().next().map(|x| x.1).unwrap();
        if cnt1 == 4 || cnt1 == 1 {
            6
        } else {
            5
        }
    } else if values.len() == 3 {
        // 3 of a kind or 2 pair
        3
    } else if values.len() == 4 {
        // 1 pair
        2
    } else {
        // High card
        1
    }
}

fn get_strength(hand: &str, dumb: bool) -> i64 {
    let mut values: HashMap<char, i64> = HashMap::new();
    for c in hand.chars() {
        if let Some(num) = values.get_mut(&c) {
            *num += 1;
        } else {
            values.insert(c, 1);
        }
    }

    let mut raw_hand_value;
    if !dumb {
        raw_hand_value = hand_value(hand);
    } else {
        raw_hand_value = hand_value2(hand);
    }

    for (k, &v) in &values {
        if v == 5 {
            // 5 of a kind
            return 14_i64.pow(11) + raw_hand_value;
        }
    }

    for (k, &v) in &values {
        if v == 4 {
            // 4 of a kind
            return 14_i64.pow(10) + raw_hand_value;
        }
    }

    if values.iter().len() == 2 {
        for (k, v) in values {
            if v == 1 || v == 4 {
                break;
            }
        }
        // full house
        return 14_i64.pow(9) + raw_hand_value;
    }

    for (k, &v) in &values {
        if v == 3 {
            // 3 of a kind
            return 14_i64.pow(8) + raw_hand_value;
        }
    }

    if values.iter().len() == 3 {
        for (k, &v) in &values {
            if v == 1 || v == 2 {
                break;
            }
        }
        // two pair
        return 14_i64.pow(7) + raw_hand_value;
    }

    if values.iter().len() == 4 {
        // one pair
        return 14_i64.pow(6) + raw_hand_value;
    }

    return raw_hand_value;
}

fn swap_jokers(hands: &[String]) -> Vec<String> {
    let mut output = vec![];
    let cards = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    println!("hands preswap: {:?}", hands);
    for hand in hands {
        let mut j_replaced = false;
        for (i, c) in hand.chars().enumerate() {
            if c == 'J' {
                for card in &cards {
                    let mut temphand = hand.to_string();
                    temphand.replace_range(i..i+1, &card.to_string());
                    output.push(temphand);
                }
                j_replaced = true;
                break;
            }
        }
        if !j_replaced {
            output.push(hand.to_string());
        }
    }
    println!("hands postswap: {:?}", output);
    output
}

fn strobe_j_values(hand: &str) -> i64 {
    let mut idxs = vec![];
    for (i, c) in hand.chars().enumerate() {
        if c == 'J' {
            idxs.push(i);
        }
    }

    let mut hands = vec![hand.to_string()];
    for _ in idxs {
        hands = swap_jokers(&hands);
    }
    println!("{:?}", hands);
    hands.iter().map(|h| get_strength(h, true)).max().unwrap()
}

fn get_strength2(hand: &str) -> i64 {
    return strobe_j_values(hand);
}

#[aoc_generator(day7)]
pub fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let derp: Vec<_> = line.split(' ').collect();
        output.push((Hand{ hand: derp[0].to_string() }, derp[1].parse::<i64>().unwrap()));
    }
    output
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut output = 0;
    let mut hands: Vec<_> = input.iter().map(|x| x.0.clone()).collect();
    hands.sort_by(|a, b| get_strength(&a.hand, false).cmp(&get_strength(&b.hand, false)));
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
    hands.sort_by(|a, b| get_strength2(&a.hand).cmp(&get_strength2(&b.hand)));
    for (hand, bid) in input {
        println!("hand: {:?}", hand.hand);
        println!("bid: {}", bid);
        let rank = hands.iter().position(|x| x == hand).unwrap() as i64 + 1;
        println!("rank: {}", rank);
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
        let hand = String::from("JA345");
        let other = String::from("22456");
        assert!(get_strength2(&hand) < get_strength2(&other));
    }
}
