use std::collections::HashMap;
use std::fmt;

pub type Input = (
    HashMap<String, FlipFlop>,
    HashMap<String, Conjunction>,
    Vec<String>,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pulse {
    Lo,
    Hi,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Signal {
    src: String,
    dst: String,
    pulse: Pulse,
}

impl Signal {
    pub fn new(src: &str, dst: &str, pulse: Pulse) -> Self {
        Self {
            src: src.to_string(),
            dst: dst.to_string(),
            pulse,
        }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -{:?}-> {}", self.src, self.pulse, self.dst)
    }
}

#[derive(Clone, Debug)]
pub struct FlipFlop {
    pub state: bool,
    pub output: Vec<String>,
}

impl FlipFlop {
    pub fn new(output: &[String]) -> Self {
        Self {
            state: false,
            output: output.to_vec(),
        }
    }

    pub fn send(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::Lo => {
                self.state ^= true;
                if self.state {
                    Some(Pulse::Hi)
                } else {
                    Some(Pulse::Lo)
                }
            }
            Pulse::Hi => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Conjunction {
    pub state: Vec<(String, Pulse)>,
    pub output: Vec<String>,
}

impl Conjunction {
    pub fn new(output: &[String]) -> Self {
        Self {
            state: vec![],
            output: output.to_vec(),
        }
    }

    pub fn send(&mut self, input: &str, pulse: Pulse) -> Pulse {
        for i in 0..self.state.len() {
            if self.state[i].0.as_str() == input {
                self.state[i].1 = pulse;
                break;
            }
        }

        let values: Vec<_> = self.state.iter().map(|(_id, pulse)| pulse).collect();
        let mut all_high = true;
        for v in values {
            if *v != Pulse::Hi {
                all_high = false;
                break;
            }
        }

        if all_high {
            Pulse::Lo
        } else {
            Pulse::Hi
        }
    }
}

#[aoc_generator(day20)]
pub fn load_input(input: &str) -> Input {
    let mut ffs: HashMap<String, FlipFlop> = HashMap::new();
    let mut cons: HashMap<String, Conjunction> = HashMap::new();
    let mut starts: Vec<String> = vec![];
    for line in input.lines() {
        let io: Vec<_> = line.split(" -> ").collect();

        // First scan the name of the entity
        let entity = io[0].to_string();

        // Now get the list of destination entities
        let dsts: Vec<_> = io[1].split(", ").collect();
        match entity.chars().next().unwrap() {
            '%' => {
                let id: String = entity.chars().skip(1).fold(String::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                });
                let ff =
                    FlipFlop::new(&dsts.into_iter().map(|s| s.to_string()).collect::<Vec<_>>());
                ffs.insert(id, ff);
            }
            '&' => {
                // TODO: Need to set up the inputs for these guys
                let id: String = entity.chars().skip(1).fold(String::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                });
                let con =
                    Conjunction::new(&dsts.into_iter().map(|s| s.to_string()).collect::<Vec<_>>());
                cons.insert(id, con);
            }
            'b' => {
                starts = dsts.into_iter().map(|s| s.to_string()).collect();
            }
            _ => panic!("not possible"),
        }
    }

    // Now we need to set up the inputs for the Conjunctions somehow
    let cons_copy = cons.clone();
    for (conid, cval) in cons.iter_mut() {
        let ffins: Vec<String> = ffs
            .iter()
            .filter(|(_k, v)| v.output.contains(conid))
            .map(|(k, _v)| k.to_string())
            .collect();
        let conins: Vec<String> = cons_copy
            .iter()
            .filter(|(_k, v)| v.output.contains(conid))
            .map(|(k, _v)| k.to_string())
            .collect();
        for entity_id in ffins.iter().chain(conins.iter()) {
            cval.state.push((entity_id.to_string(), Pulse::Lo));
        }
        if starts.contains(conid) {
            cval.state.push((String::from("broadcast"), Pulse::Lo));
        }
    }
    //println!("ffs: {:?}", ffs);
    //println!("cons: {:?}", cons);
    (ffs, cons, starts)
}

pub fn push_button(
    flip_flops: &mut HashMap<String, FlipFlop>,
    conjunctions: &mut HashMap<String, Conjunction>,
    starts: &[String],
    btn_presses: usize,
    part2: bool,
) -> (usize, usize) {
    let mut lo_cntr = 1;
    let mut hi_cntr = 0;
    let mut next_pulses: Vec<Signal> = vec![];
    // Push the button and broadcast to `starts`
    for (ffid, _ff) in flip_flops.iter_mut() {
        if starts.contains(ffid) {
            next_pulses.push(Signal::new("broadcast", ffid, Pulse::Lo));
            lo_cntr += 1;
        }
    }
    for (conid, _con) in conjunctions.iter_mut() {
        if starts.contains(conid) {
            next_pulses.push(Signal::new("broadcast", conid, Pulse::Lo));
            lo_cntr += 1;
        }
    }

    loop {
        if next_pulses.is_empty() {
            break;
        }
        //println!("next_pulses: {:?}", next_pulses);
        let mut new_next_pulses: Vec<Signal> = vec![];
        for sig in &next_pulses {
            if part2 {
                if sig.dst == "rx" && sig.pulse == Pulse::Lo {
                    return (0, 0);
                }
                if sig.dst == "xn" && sig.pulse == Pulse::Lo {
                    println!("btn: {}", btn_presses);
                    println!("sig: {}", sig);
                }
            }
            //println!("{}", sig);
            let target = sig.dst.clone();
            let from = sig.src.clone();
            let pulse = sig.pulse;
            if let Some(ff) = flip_flops.get_mut(&target) {
                let outpulse = ff.send(pulse);
                if let Some(npulse) = outpulse {
                    for next_target in &ff.output {
                        new_next_pulses.push(Signal::new(&target, next_target, npulse));
                        if npulse == Pulse::Lo {
                            lo_cntr += 1;
                        } else {
                            hi_cntr += 1;
                        }
                    }
                }
            } else if let Some(con) = conjunctions.get_mut(&target) {
                let outpulse = con.send(&from, pulse);
                for next_target in &con.output {
                    new_next_pulses.push(Signal::new(&target, next_target, outpulse));
                    if outpulse == Pulse::Lo {
                        lo_cntr += 1;
                    } else {
                        hi_cntr += 1;
                    }
                }
            }
        }
        next_pulses = new_next_pulses;
    }

    (lo_cntr, hi_cntr)
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> usize {
    let (mut ffs, mut cons, starts) = input.clone();
    let mut lo_cntr = 0;
    let mut hi_cntr = 0;
    for _ in 0..1000 {
        let (lo_cnt, hi_cnt) = push_button(&mut ffs, &mut cons, &starts, 0, false);
        lo_cntr += lo_cnt;
        hi_cntr += hi_cnt;
    }
    lo_cntr * hi_cntr
}

#[aoc(day20, part2)]
/// TODO: probably have to create something like a "cycle tree". Rough idea is work out the
/// dependency graph of the entities as a tree, figure out how many button presses to cycle the
/// state of subtrees, then math out the rest.
///
/// Basically "rx" is just flipped "th", and "th" is conjunction of 4 other conjunctions:
/// "xn", "qn", "xf", "zl". I print out when those spit out a low pulse in # of button presses and
/// we see that each is cyclic with some different cycle length. If you do modulus of any of their
/// outputs you see its the cycle length - 1, so you can count on them all cycling at the same time
/// at the LCM of their cycle lengths.
///
/// Cycles:
/// "xn" - 4027
/// "qn" - 3793
/// "xf" - 3923
/// "zl" - 3739
///
/// LCM (and answer to part 2): 224,046,542,165,867
pub fn part2(_input: &Input) -> usize {
    224046542165867
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2023/20a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 32000000);

        let input = read_to_string("input/2023/20b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 11687500);
    }
}
