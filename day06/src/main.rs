use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> String {
    let len = input.lines().next().unwrap().len();
    (0..len)
        .map(|i| {
            let mut freq = HashMap::new();
            input.lines().for_each(|line| {
                let c = line.chars().nth(i).unwrap();
                *freq.entry(c).or_insert(0) += 1;
            });
            freq.iter().max_by_key(|a| a.1).unwrap().0.to_string()
        })
        .collect()
}

fn problem2(input: &str) -> String {
    let len = input.lines().next().unwrap().len();
    (0..len)
        .map(|i| {
            let mut freq = HashMap::new();
            input.lines().for_each(|line| {
                let c = line.chars().nth(i).unwrap();
                *freq.entry(c).or_insert(0) += 1;
            });
            freq.iter().min_by_key(|a| a.1).unwrap().0.to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem1(input), "easter");
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem2(input), "advent");
    }
}
