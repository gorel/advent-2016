use std::fmt;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2:\n{}", problem2(input));
}

const SCREEN_ROWS: usize = 6;
const SCREEN_COLS: usize = 50;

struct Screen {
    data: Vec<Vec<bool>>,
}

impl Screen {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![false; cols]; rows],
        }
    }

    fn exec(&mut self, cmd: &str) {
        match cmd.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["rect", axb] => {
                let parts = axb.split('x').collect::<Vec<&str>>();
                let a = parts[0].parse().unwrap();
                let b = parts[1].parse().unwrap();
                self.rect(a, b)
            }
            ["rotate", "row", a, "by", b] => {
                let row = a.split('=').collect::<Vec<&str>>().as_slice()[1]
                    .parse()
                    .unwrap();
                let n = b.parse().unwrap();
                self.rotate_row(row, n)
            }
            ["rotate", "column", a, "by", b] => {
                let col = a.split('=').collect::<Vec<&str>>().as_slice()[1]
                    .parse()
                    .unwrap();
                let n = b.parse().unwrap();
                self.rotate_col(col, n)
            }
            _ => panic!("Invalid command: {}", cmd),
        }
    }

    fn rect(&mut self, a: usize, b: usize) {
        for row in 0..b {
            for col in 0..a {
                self.data[row][col] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, n: usize) {
        let mut new_data = vec![false; 50];
        for col in 0..50 {
            new_data[(col + n) % 50] = self.data[row][col];
        }
        self.data[row] = new_data;
    }

    fn rotate_col(&mut self, col: usize, n: usize) {
        let mut new_data = vec![false; 6];
        for row in 0..6 {
            new_data[(row + n) % 6] = self.data[row][col];
        }
        for row in 0..6 {
            self.data[row][col] = new_data[row];
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for &cell in row {
                write!(f, "{}", if cell { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn problem1(input: &str) -> isize {
    let mut screen = Screen::new(SCREEN_ROWS, SCREEN_COLS);
    input.lines().for_each(|line| screen.exec(line));
    screen
        .data
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum::<usize>() as isize
}

fn problem2(input: &str) -> String {
    let mut screen = Screen::new(SCREEN_ROWS, SCREEN_COLS);
    input.lines().for_each(|line| screen.exec(line));
    screen.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem1(input), 6);
    }

    #[test]
    fn test_problem2() {}
}
