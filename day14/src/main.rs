use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input.trim()));
    println!("Problem 2: {}", problem2(input.trim()));
}

#[derive(Debug, Clone)]
struct Stats {
    first_triplet: Option<char>,
    quints: HashSet<char>,
}

impl Stats {
    fn from_input(input: &str, stretch: bool, n: usize) -> Stats {
        let mut first_triplet = None;
        let mut quints = HashSet::new();

        let md5_rounds = if stretch { 2016 } else { 0 };
        let init = format!("{:x}", md5::compute(format!("{}{}", input, n)));
        let hash = (0..md5_rounds).fold(init, |hash, _| format!("{:x}", md5::compute(hash)));
        let chars = hash.chars().collect::<Vec<_>>();
        for i in 0..chars.len() - 2 {
            if i < chars.len() - 4 && chars[i..i + 5].iter().all(|c| *c == chars[i]) {
                quints.insert(chars[i]);
            }
            if first_triplet.is_none() && chars[i..i + 3].iter().all(|c| *c == chars[i]) {
                first_triplet = Some(chars[i]);
            }
        }
        Stats {
            first_triplet,
            quints,
        }
    }
}

struct StatMemo {
    input: String,
    memo: DashMap<usize, Stats>,
    stretch: bool,
}

impl StatMemo {
    fn new(input: &str, stretch: bool) -> StatMemo {
        StatMemo {
            input: input.to_string(),
            memo: DashMap::new(),
            stretch,
        }
    }

    fn get_or_compute(&self, n: usize) -> Stats {
        self.memo
            .entry(n)
            .or_insert_with(|| Stats::from_input(&self.input, self.stretch, n))
            .value()
            .clone()
    }
}

fn problem1(input: &str) -> usize {
    let memo = StatMemo::new(input, false);
    (0..20000).into_par_iter().for_each(|i| {
        memo.get_or_compute(i);
    });

    (0..)
        .filter(|&i| {
            let stats = memo.get_or_compute(i).clone();
            if let Some(c) = stats.first_triplet {
                (i + 1..i + 1000).any(|j| memo.get_or_compute(j).quints.contains(&c))
            } else {
                false
            }
        })
        .take(64)
        .last()
        .unwrap()
}

fn problem2(input: &str) -> usize {
    let memo = StatMemo::new(input, true);
    (0..20000).into_par_iter().for_each(|i| {
        memo.get_or_compute(i);
    });

    (0..)
        .filter(|&i| {
            let stats = memo.get_or_compute(i).clone();
            if let Some(c) = stats.first_triplet {
                (i + 1..i + 1000).any(|j| memo.get_or_compute(j).quints.contains(&c))
            } else {
                false
            }
        })
        .take(64)
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem1(input.trim()), 22728);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem2(input.trim()), 22551);
    }
}
