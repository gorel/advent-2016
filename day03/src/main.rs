fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn parse(s: &str) -> (isize, isize, isize) {
    let [a, b, c]: [isize; 3] = s
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<isize>>()
        .try_into()
        .unwrap();
    (a, b, c)
}

fn possible(a: isize, b: isize, c: isize) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn problem1(input: &str) -> isize {
    input.lines().fold(0, |acc, line| {
        let (a, b, c) = parse(line);
        acc + possible(a, b, c) as isize
    })
}

fn problem2(input: &str) -> isize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |acc, chunk| {
            let [line1, line2, line3] = chunk else { panic!("Invalid input") };
            let (a1, b1, c1) = parse(line1);
            let (a2, b2, c2) = parse(line2);
            let (a3, b3, c3) = parse(line3);
            let p1 = possible(a1, a2, a3) as isize;
            let p2 = possible(b1, b2, b3) as isize;
            let p3 = possible(c1, c2, c3) as isize;
            acc + p1 + p2 + p3
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = "5 10 25\n5 10 10\n";
        assert_eq!(problem1(input), 1);
    }

    #[test]
    fn test_problem2() {
        let input = "5 10 25\n10 10 20\n25 5 15\n";
        assert_eq!(problem2(input), 2);
    }
}
