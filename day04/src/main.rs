use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug)]
struct Room {
    name: String,
    sector: isize,
    checksum: String,
}

impl Room {
    fn new(name: String, sector: isize, checksum: String) -> Room {
        Room {
            name,
            sector,
            checksum,
        }
    }

    fn parse(line: &str) -> Room {
        let parts: Vec<&str> = line[0..line.len() - 1]
            .split(|c| c == '-' || c == '[')
            .collect();
        let name = parts[0..parts.len() - 2].join("-");
        let sector = parts[parts.len() - 2].parse::<isize>().unwrap();
        let checksum = parts[parts.len() - 1].to_string();
        Room::new(name, sector, checksum)
    }

    fn validate(&self) -> bool {
        let mut freq = HashMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            let counter = freq.entry(c).or_insert(0);
            *counter += 1;
        }
        let mut freq_vec: Vec<(&char, &i32)> = freq.iter().collect();
        freq_vec.sort_by(|a, b| {
            // First sort by frequency, descending
            // Then sort by character, ascending
            let freq_cmp = b.1.cmp(a.1);
            if freq_cmp == std::cmp::Ordering::Equal {
                a.0.cmp(b.0)
            } else {
                freq_cmp
            }
        });
        let top_5: Vec<(&char, &i32)> = freq_vec.into_iter().take(5).collect();
        let checksum: String = top_5.iter().map(|(c, _)| **c).collect();
        checksum == self.checksum
    }

    fn decrypt(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let offset = self.sector % 26;
                    let mut new_char = c as u8 + offset as u8;
                    if new_char > b'z' {
                        new_char -= 26;
                    }
                    new_char as char
                }
            })
            .collect()
    }
}

fn problem1(input: &str) -> isize {
    input.lines().fold(0, |acc, line| {
        let room = Room::parse(line);
        acc + if room.validate() { room.sector } else { 0 }
    })
}

fn problem2(input: &str) -> isize {
    input
        .lines()
        .find_map(|line| {
            let room = Room::parse(line);
            if room.validate() {
                let decrypted = room.decrypt();
                if decrypted.contains("north") {
                    return Some(room.sector);
                }
            }
            None
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input =
            "aaaaa-bbb-z-y-x-1[abxyz]\na-b-c-d-e-f-g-h-3[abcde]\ntotally-real-room-7[decoy]\n";
        assert_eq!(problem1(input), 4);
    }
}
