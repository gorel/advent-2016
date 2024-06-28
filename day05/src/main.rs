fn main() {
    let input = include_str!("../data/input.txt").trim();
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> String {
    (0..)
        .map(|i| format!("{}{}", input, i))
        .filter_map(|s| {
            let md5 = format!("{:x}", md5::compute(&s));
            if md5.starts_with("00000") {
                Some(md5.chars().nth(5).unwrap())
            } else {
                None
            }
        })
        .take(8)
        .collect()
}

fn problem2(input: &str) -> String {
    let mut password = vec![' '; 8];
    (0..).find(|i| {
        let s = format!("{}{}", input, i);
        let md5 = format!("{:x}", md5::compute(&s));
        if md5.starts_with("00000") {
            let pos = md5.chars().nth(5).unwrap();
            let val = md5.chars().nth(6).unwrap();
            if pos.is_digit(10) {
                let pos = pos.to_digit(10).unwrap() as usize;
                if pos < 8 && password[pos] == ' ' {
                    password[pos] = val;
                    if password.iter().all(|&c| c != ' ') {
                        return true;
                    }
                }
            }
        }
        false
    });
    password.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1("abc"), "18f47a30");
    }

    #[test]
    fn test_problem2() {
        assert_eq!(problem2("abc"), "05ace8e3");
    }
}
