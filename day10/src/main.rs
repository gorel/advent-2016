use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input, &[17, 61]));
    println!("Problem 2: {}", problem2(input));
}

lazy_static! {
    static ref VALUE_RE: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    static ref INSTRUCTION_RE: Regex =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)")
            .unwrap();
}

#[derive(Clone, Default, Debug)]
struct BotOrOutput {
    vals: Vec<i32>,
    low_target: usize,
    high_target: usize,
    ready: bool,
}

impl BotOrOutput {
    fn rcv(&mut self, val: i32) {
        self.vals.push(val);
        self.vals.sort();
        self.ready = self.vals.len() == 2;
    }
}

struct Solution {
    target_idx: Option<usize>,
    bots: HashMap<usize, BotOrOutput>,
}

fn simulate(input: &str, target: &[i32]) -> Solution {
    let mut bots: HashMap<usize, BotOrOutput> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("value") {
            // value 5 goes to bot 2
            let caps = VALUE_RE.captures(line).unwrap();
            let value: i32 = caps[1].parse().unwrap();
            let idx: usize = caps[2].parse().unwrap();
            bots.entry(idx).or_default().rcv(value);
        } else {
            let caps = INSTRUCTION_RE.captures(line).unwrap();
            let from: usize = caps[1].parse().unwrap();
            bots.entry(from).or_default().low_target = match &caps[2] {
                "output" => 1000 + caps[3].parse::<usize>().unwrap(),
                _ => caps[3].parse::<usize>().unwrap(),
            };
            bots.entry(from).or_default().high_target = match &caps[4] {
                "output" => 1000 + caps[5].parse::<usize>().unwrap(),
                _ => caps[5].parse::<usize>().unwrap(),
            };
        }
    }

    let mut ready_bots: Vec<usize> = bots
        .iter()
        .filter_map(|(i, b)| if b.ready { Some(*i) } else { None })
        .collect();

    let mut target_idx = None;
    while ready_bots.len() > 0 {
        let idx = ready_bots.pop().unwrap();
        if idx <= 1000 && bots[&idx].vals == target {
            target_idx = Some(idx);
        }
        let high_target = bots[&idx].high_target;
        let high_value = bots.get_mut(&idx).unwrap().vals.pop().unwrap();
        let low_target = bots[&idx].low_target;
        let low_value = bots.get_mut(&idx).unwrap().vals.pop().unwrap();
        bots.get_mut(&idx).unwrap().ready = false;

        bots.entry(low_target).or_default().rcv(low_value);
        bots.entry(high_target).or_default().rcv(high_value);

        if low_target <= 1000 && bots[&low_target].vals.len() == 2 {
            ready_bots.push(low_target);
        }
        if high_target <= 1000 && bots[&high_target].vals.len() == 2 {
            ready_bots.push(high_target);
        }
    }
    Solution { target_idx, bots }
}

fn problem1(input: &str, target: &[i32]) -> usize {
    simulate(input, target).target_idx.unwrap()
}

fn problem2(input: &str) -> isize {
    simulate(input, &[0, 0])
        .bots
        .iter()
        .filter(|(i, _)| *i >= &1000 && *i <= &1002)
        .map(|(_, b)| b.vals[0])
        .product::<i32>() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem1(input, &[2, 5]), 2);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem2(input), 5 * 2 * 3);
    }
}
