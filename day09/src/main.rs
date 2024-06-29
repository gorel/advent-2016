fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input.trim()));
    println!("Problem 2: {}", problem2(input.trim()));
}

fn get_len(s: &str, start: usize, end: usize, part2: bool) -> usize {
    let mut res = 0;
    let mut i = start;
    let mut inside = false;

    let mut expansion = String::new();
    while i < end {
        match &s[i..i + 1] {
            "(" => inside = true,
            ")" => {
                let axb: Vec<&str> = expansion.split('x').collect();
                let a = axb[0].parse::<usize>().unwrap();
                let b = axb[1].parse::<usize>().unwrap();
                if part2 {
                    res += get_len(s, i + 1, i + 1 + a, true) * b;
                } else {
                    res += a * b;
                }
                i += a;
                expansion.clear();
                inside = false;
            }
            c if inside => expansion.push_str(c),
            _ => res += 1,
        }
        i += 1;
    }
    res
}

fn problem1(input: &str) -> usize {
    get_len(input, 0, input.len(), false)
}

fn problem2(input: &str) -> usize {
    get_len(input, 0, input.len(), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(6, problem1("ADVENT"));
        assert_eq!(7, problem1("A(1x5)BC"));
        assert_eq!(9, problem1("(3x3)XYZ"));
        assert_eq!(11, problem1("A(2x2)BCD(2x2)EFG"));
        assert_eq!(6, problem1("(6x1)(1x3)A"));
    }

    #[test]
    fn test_problem2() {
        assert_eq!(9, problem2("(3x3)XYZ"));
        assert_eq!(20, problem2("X(8x2)(3x3)ABCY"));
        assert_eq!(241920, problem2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(
            445,
            problem2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }
}
