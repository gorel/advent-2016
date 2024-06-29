use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct IP {
    supernets: Vec<String>,
    hypernets: Vec<String>,
}

impl IP {
    fn new(s: &str) -> IP {
        let mut supernets = Vec::new();
        let mut hypernets = Vec::new();
        let mut cur = String::new();
        for c in s.chars() {
            match c {
                '[' => {
                    supernets.push(cur);
                    cur = String::new();
                }
                ']' => {
                    hypernets.push(cur);
                    cur = String::new();
                }
                _ => cur.push(c),
            }
        }
        supernets.push(cur);
        IP {
            supernets,
            hypernets,
        }
    }

    fn is_abba(&self, s: &str) -> bool {
        for i in 0..s.len() - 3 {
            let chars = s[i..i + 4].chars().collect::<Vec<char>>();
            let (a, b, c, d) = (chars[0], chars[1], chars[2], chars[3]);
            if a == d && b == c && a != b {
                return true;
            }
        }
        false
    }

    fn has_abba(&self) -> bool {
        let in_supernet = self.supernets.iter().any(|s| self.is_abba(s));
        let in_hypernet = self.hypernets.iter().any(|s| self.is_abba(s));
        in_supernet && !in_hypernet
    }

    fn abas(&self, vecs: &Vec<String>) -> HashSet<String> {
        let mut res = HashSet::new();
        for supernet in vecs {
            for i in 0..supernet.len() - 2 {
                let chars = supernet[i..i + 3].chars().collect::<Vec<char>>();
                let (a, b, c) = (chars[0], chars[1], chars[2]);
                if a == c && a != b {
                    res.insert(format!("{}{}{}", a, b, c));
                }
            }
        }
        res
    }

    fn has_ssl(&self) -> bool {
        let abas = self.abas(&self.supernets);
        let babs = self.abas(&self.hypernets);
        abas.iter().any(|aba| {
            let (x, y) = (aba.chars().nth(0).unwrap(), aba.chars().nth(1).unwrap());
            let bab = format!("{}{}{}", y, x, y);
            babs.contains(&bab)
        })
    }
}

fn problem1(input: &str) -> isize {
    input
        .lines()
        .map(IP::new)
        .fold(0, |acc, ip| acc + ip.has_abba() as isize)
}

fn problem2(input: &str) -> isize {
    input
        .lines()
        .map(IP::new)
        .fold(0, |acc, ip| acc + (ip.has_ssl()) as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem1(input), 2);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample2.txt");
        assert_eq!(problem2(input), 3);
    }
}
