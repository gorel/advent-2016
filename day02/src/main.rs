fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> String {
    let (mut x, mut y) = (1, 1);
    input
        .lines()
        .map(|line| {
            line.chars().for_each(|c| match c {
                'U' => y = (y - 1).max(0),
                'D' => y = (y + 1).min(2),
                'L' => x = (x - 1).max(0),
                'R' => x = (x + 1).min(2),
                _ => panic!("Invalid direction"),
            });
            (y * 3 + x + 1).to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}

fn inbounds(x: isize, y: isize) -> bool {
    match x {
        0 => y == 2,
        1 => y >= 1 && y <= 3,
        2 => y >= 0 && y <= 4,
        3 => y >= 1 && y <= 3,
        4 => y == 2,
        _ => false,
    }
}

fn at(row: isize, col: isize) -> &'static str {
    match (row, col) {
        (0, 2) => "1",
        (1, 1) => "2",
        (1, 2) => "3",
        (1, 3) => "4",
        (2, 0) => "5",
        (2, 1) => "6",
        (2, 2) => "7",
        (2, 3) => "8",
        (2, 4) => "9",
        (3, 1) => "A",
        (3, 2) => "B",
        (3, 3) => "C",
        (4, 2) => "D",
        _ => panic!("Invalid coordinates"),
    }
}

fn problem2(input: &str) -> String {
    let (mut row, mut col) = (2, 0);
    input
        .lines()
        .map(|line| {
            line.chars().for_each(|c| match c {
                'U' => row = if inbounds(row - 1, col) { row - 1 } else { row },
                'D' => row = if inbounds(row + 1, col) { row + 1 } else { row },
                'L' => col = if inbounds(row, col - 1) { col - 1 } else { col },
                'R' => col = if inbounds(row, col + 1) { col + 1 } else { col },
                _ => panic!("Invalid direction"),
            });
            at(row, col).to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        let result = problem1(input);
        assert_eq!(result, "1985");
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
        let result = problem2(input);
        assert_eq!(result, "5DB3");
    }
}
