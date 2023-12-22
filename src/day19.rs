use aoc_helpers::interval::Interval;
use rayon::prelude::*;
use std::collections::HashMap;

pub type Input = (HashMap<String, Vec<Rule>>, Vec<Item>);

#[derive(Copy, Clone, Debug)]
pub struct Item {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Item {
    pub fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    pub fn get(&self, attr: &str) -> usize {
        match attr {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("invalid attribute"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RuleType {
    LessThan,
    GreaterThan,
    GoTo,
}

#[derive(Clone, Debug)]
pub struct Rule {
    check: Option<String>,
    rule_type: RuleType,
    value: Option<usize>,
    goto: String,
}

impl Rule {
    pub fn new(
        check: Option<String>,
        rule_type: RuleType,
        value: Option<usize>,
        goto: &str,
    ) -> Self {
        Self {
            check,
            rule_type,
            value,
            goto: goto.to_string(),
        }
    }
}

#[aoc_generator(day19)]
pub fn load_input(input: &str) -> Input {
    let mut parts = false;
    let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut items = vec![];
    for line in input.lines() {
        if parts {
            // Handle the Items
            let temp = line.split([',', '=']).collect::<Vec<_>>();
            let x = temp[1].parse::<usize>().unwrap();
            let m = temp[3].parse::<usize>().unwrap();
            let a = temp[5].parse::<usize>().unwrap();
            let s = temp[7].trim_end_matches('}').parse::<usize>().unwrap();
            items.push(Item { x, m, a, s });
        } else {
            if line.is_empty() {
                // Now switch to parts
                parts = true;
            } else {
                // Parse the Rules
                let temp = line.split('{').collect::<Vec<_>>();
                let name = temp[0].to_string();
                let linerules = temp[1].split(',').collect::<Vec<_>>();
                let mut _rules = vec![];
                for rule in linerules {
                    let temp2 = rule.split(':').collect::<Vec<_>>();
                    if temp2.len() > 1 {
                        let goto = temp2[1];
                        let mut rule_type = RuleType::GreaterThan;
                        if temp2[0].find('<').is_some() {
                            // Less than rule
                            rule_type = RuleType::LessThan;
                        }
                        let temp3 = temp2[0].split(['<', '>']).collect::<Vec<_>>();
                        let check = Some(temp3[0].to_string());
                        let value = Some(temp3[1].parse::<usize>().unwrap());
                        _rules.push(Rule::new(check, rule_type, value, goto));
                    } else {
                        // Length 1 rule is just a goto
                        let goto = temp2[0].trim_end_matches('}');
                        _rules.push(Rule::new(None, RuleType::GoTo, None, goto));
                    }
                }
                rules.insert(name, _rules);
            }
        }
    }
    (rules, items)
}

pub fn check_accepted(item: Item, rules: &HashMap<String, Vec<Rule>>) -> bool {
    let mut current = String::from("in");
    loop {
        let ruleset = rules.get(&current).unwrap();
        for rule in ruleset {
            match rule.rule_type {
                RuleType::LessThan => {
                    let value = item.get(&rule.check.as_ref().unwrap());
                    if value < rule.value.unwrap() {
                        current = rule.goto.clone();
                        break;
                    }
                }
                RuleType::GreaterThan => {
                    let value = item.get(&rule.check.as_ref().unwrap());
                    if value > rule.value.unwrap() {
                        current = rule.goto.clone();
                        break;
                    }
                }
                RuleType::GoTo => {
                    current = rule.goto.clone();
                    break;
                }
            }
        }

        if current == "A" {
            return true;
        } else if current == "R" {
            return false;
        }
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> usize {
    let (rules, items) = input;
    let mut output = 0;
    for item in items {
        if check_accepted(*item, &rules) {
            output += item.rating();
        }
    }
    output
}

#[aoc(day19, part2)]
// Each attribute can be in [1..4000] inclusive. How many accepted combos?
pub fn part2(input: &Input) -> usize {
    let (rules, _items) = input;
    let full = Interval::new(1, 4000);
    let arules = rules
        .values()
        .flat_map(|v| {
            for rule in v.iter().rev() {
                if rule.goto == String::from("A") {
                    // Start a new Interval here, propagate back to "in"
                    // TODO: Probably have to create a tree/DAG to trace back Leaf Nodes with "A"
                    // to the "in" Node.
                }
            }
        })
        .collect::<Vec<_>>();
    println!("arules: {:?}", arules);
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/19a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 19114);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2023/19a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 167409079868000);
    }
}
