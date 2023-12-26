use aoc_helpers::interval::Interval;
use aoc_helpers::tree::Tree;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RuleType {
    LessThan,
    GreaterThan,
    GoTo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
                    let value = item.get(rule.check.as_ref().unwrap());
                    if value < rule.value.unwrap() {
                        current = rule.goto.clone();
                        break;
                    }
                }
                RuleType::GreaterThan => {
                    let value = item.get(rule.check.as_ref().unwrap());
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
        if check_accepted(*item, rules) {
            output += item.rating();
        }
    }
    output
}

#[derive(Copy, Clone, Debug)]
pub struct XmasInterval {
    pub x: Option<Interval>,
    pub m: Option<Interval>,
    pub a: Option<Interval>,
    pub s: Option<Interval>,
}

impl fmt::Display for XmasInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x.is_some() && self.m.is_some() && self.a.is_some() && self.s.is_some() {
            write!(
                f,
                "x: {}, m: {}, a: {}, s: {}",
                self.x.unwrap(),
                self.m.unwrap(),
                self.a.unwrap(),
                self.s.unwrap()
            )
        } else {
            write!(f, "XMAS: Empty")
        }
    }
}

impl Default for XmasInterval {
    fn default() -> Self {
        Self::new()
    }
}

impl XmasInterval {
    pub fn new() -> Self {
        Self {
            x: Some(Interval::new(1, 4000)),
            m: Some(Interval::new(1, 4000)),
            a: Some(Interval::new(1, 4000)),
            s: Some(Interval::new(1, 4000)),
        }
    }

    pub fn num_ways(&self) -> usize {
        if self.x.is_some() && self.m.is_some() && self.a.is_some() && self.s.is_some() {
            self.x.unwrap().len()
                * self.m.unwrap().len()
                * self.a.unwrap().len()
                * self.s.unwrap().len()
        } else {
            0
        }
    }

    pub fn apply_restriction(&mut self, field: char, restriction: Interval) {
        match field {
            'x' => {
                if self.x.is_some() {
                    let temp = self.x.unwrap().difference(restriction);
                    if !temp.is_empty() {
                        self.x = Some(temp[0]);
                    } else {
                        self.x = None;
                    }
                }
            }
            'm' => {
                if self.m.is_some() {
                    let temp = self.m.unwrap().difference(restriction);
                    if !temp.is_empty() {
                        self.m = Some(temp[0]);
                    } else {
                        self.m = None;
                    }
                }
            }
            'a' => {
                if self.a.is_some() {
                    let temp = self.a.unwrap().difference(restriction);
                    if !temp.is_empty() {
                        self.a = Some(temp[0]);
                    } else {
                        self.a = None;
                    }
                }
            }
            's' => {
                if self.s.is_some() {
                    let temp = self.s.unwrap().difference(restriction);
                    if !temp.is_empty() {
                        self.s = Some(temp[0]);
                    } else {
                        self.s = None;
                    }
                }
            }
            _ => panic!("invalid check"),
        }
    }
}

// Do to the construction of the problem we never have to worry about 1 Interval becoming two after
// differencing out the middle.
pub fn backpropagation(idx: usize, rule_tree: &Tree<(Rule, Option<bool>)>) -> XmasInterval {
    let path = rule_tree.path_to_node(idx).unwrap();
    let mut output = XmasInterval::new();
    for idx in path {
        let (rule, take_path) = &rule_tree.nodes[idx].value;
        match rule.rule_type {
            RuleType::LessThan => {
                let restriction = if take_path.unwrap() {
                    Interval::new(rule.value.unwrap() as i64, 4000)
                } else {
                    Interval::new(1, rule.value.unwrap() as i64 - 1)
                };
                output.apply_restriction(
                    rule.check.clone().unwrap().chars().next().unwrap(),
                    restriction,
                );
            }
            RuleType::GreaterThan => {
                let restriction = if take_path.unwrap() {
                    Interval::new(1, rule.value.unwrap() as i64)
                } else {
                    Interval::new(rule.value.unwrap() as i64 + 1, 4000)
                };
                output.apply_restriction(
                    rule.check.clone().unwrap().chars().next().unwrap(),
                    restriction,
                );
            }
            RuleType::GoTo => (),
        }
    }
    output
}

#[aoc(day19, part2)]
// Each attribute can be in [1..4000] inclusive. How many accepted combos?
pub fn part2(input: &Input) -> usize {
    let (rules, _items) = input;
    let mut rule_tree = Tree::with_head((Rule::new(None, RuleType::GoTo, None, "in"), None));
    let mut next_rules = vec![(0, String::from("in"))];
    loop {
        if next_rules.is_empty() {
            break;
        }
        let mut new_next_rules = vec![];
        for (_pid, current_rule) in &next_rules {
            if current_rule == "A" || current_rule == "R" {
                continue;
            }
            let mut pid = *_pid;
            let rule_list = rules.get(current_rule).unwrap();
            for rule in rule_list {
                if rule.rule_type == RuleType::GoTo {
                    let cid = rule_tree
                        .add_child_to_node((rule.clone(), None), pid)
                        .unwrap();
                    new_next_rules.push((cid, rule.goto.clone()));
                    pid = cid;
                } else {
                    let cid1 = rule_tree
                        .add_child_to_node((rule.clone(), Some(false)), pid)
                        .unwrap();
                    let cid2 = rule_tree
                        .add_child_to_node((rule.clone(), Some(true)), pid)
                        .unwrap();
                    new_next_rules.push((cid2, rule.goto.clone()));
                    pid = cid1; // If we continue along this path, then we didn't take the path and
                                // pid should be this cid, not the next one
                }
            }
        }
        next_rules = new_next_rules;
    }
    // We have a tree built with '.path_to_node()' now... search for "A"s, and create Intervals and
    // backpropagate.
    let mut output = 0;
    for idx in &rule_tree.leaves() {
        let (rule, _take_path) = &rule_tree.nodes[*idx].value;
        if rule.goto == "A" {
            // Create interval and backpropagate
            let interval = backpropagation(*idx, &rule_tree);
            output += interval.num_ways();
        }
    }
    output
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
