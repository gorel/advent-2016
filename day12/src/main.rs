use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}
fn valueof(reg: &str, registers: &HashMap<String, i64>) -> i64 {
    if let Ok(val) = reg.parse::<i64>() {
        val
    } else {
        *registers.get(reg).unwrap_or(&0)
    }
}

fn solve(mut registers: HashMap<String, i64>, instructions: Vec<Vec<&str>>) -> i64 {
    let mut pc = 0;
    while pc < instructions.len() {
        match instructions[pc][..] {
            ["cpy", valreg, reg] => {
                registers.insert(reg.to_string(), valueof(valreg, &registers));
            }
            ["inc", reg] => {
                let cur = valueof(reg, &registers);
                registers.insert(reg.to_string(), cur + 1);
            }
            ["dec", reg] => {
                let cur = valueof(reg, &registers);
                registers.insert(reg.to_string(), cur - 1);
            }
            ["jnz", reg, offset] => {
                let cur = valueof(reg, &registers);
                if cur != 0 {
                    pc = (pc as i32 + offset.parse::<i32>().unwrap()) as usize;
                    continue;
                }
            }
            _ => panic!("Invalid instruction"),
        }
        pc += 1;
    }
    registers["a"]
}

fn problem1(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    solve(HashMap::new(), instructions)
}

fn problem2(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut registers = HashMap::new();
    registers.insert("c".to_string(), 1);
    solve(registers, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem1(input), 42);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem2(input), 42);
    }
}
