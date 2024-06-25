use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

const DIRS: [char; 4] = ['N', 'E', 'S', 'W'];

fn problem1(input: &str) -> i32 {
    let (_, dx, dy) = input.split(',').fold(('N', 0, 0), |(dir, x, y), line| {
        let (turn, dist) = line.trim().split_at(1);
        let dir = match turn {
            "R" => DIRS[(DIRS.iter().position(|&d| d == dir).unwrap() + 1) % 4],
            "L" => DIRS[(DIRS.iter().position(|&d| d == dir).unwrap() + 3) % 4],
            _ => panic!("Invalid"),
        };
        let dist = dist.parse::<i32>().unwrap();
        match dir {
            'N' => (dir, x, y + dist),
            'S' => (dir, x, y - dist),
            'E' => (dir, x + dist, y),
            'W' => (dir, x - dist, y),
            _ => panic!("Invalid"),
        }
    });
    dx.abs() + dy.abs()
}

fn problem2(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let (mut dir, mut x, mut y): (char, i32, i32) = ('N', 0, 0);
    for instruction in input.split(',') {
        let (turn, dist) = instruction.trim().split_at(1);
        dir = match turn {
            "R" => DIRS[(DIRS.iter().position(|&d| d == dir).unwrap() + 1) % 4],
            "L" => DIRS[(DIRS.iter().position(|&d| d == dir).unwrap() + 3) % 4],
            _ => panic!("Invalid"),
        };
        let dist = dist.parse::<i32>().unwrap();
        for _ in 1..=dist {
            (x, y) = match dir {
                'N' => (x, y + 1),
                'S' => (x, y - 1),
                'E' => (x + 1, y),
                'W' => (x - 1, y),
                _ => panic!("Invalid"),
            };
            if seen.contains(&(x, y)) {
                return x.abs() + y.abs();
            }
            seen.insert((x, y));
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let tests_with_expected = vec![("R2, L3", 5), ("R2, R2, R2", 2), ("R5, L5, R5, R3", 12)];
        for (input, expected) in tests_with_expected {
            assert_eq!(problem1(input), expected);
        }
    }

    #[test]
    fn test_problem2() {
        let input = "R8, R4, R4, R8";
        let actual = problem2(input);
        assert_eq!(actual, 4);
    }
}
