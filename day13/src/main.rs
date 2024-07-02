use std::collections::{HashSet, VecDeque};

type Point = (i64, i64);

enum Target {
    Point(Point),
    Distance(usize),
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn is_wall(point: &Point, input: i64) -> bool {
    // x*x + 3*x + 2*x*y + y + y*y
    let sum = point.0 * point.0
        + 3 * point.0
        + 2 * point.0 * point.1
        + point.1
        + point.1 * point.1
        + input;
    let binary = format!("{:b}", sum);
    binary.chars().filter(|&c| c == '1').count() % 2 == 1
}

fn bfs(start: Point, end: Target, input: i64) -> usize {
    let mut queue = VecDeque::from(vec![(0, start)]);
    let mut visited = HashSet::new();
    while let Some((distance, point)) = queue.pop_front() {
        if let Target::Point(end) = end {
            if point == end {
                return distance;
            }
        }
        visited.insert(point);
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_point = (point.0 + dx, point.1 + dy);
            if new_point.0 < 0
                || new_point.1 < 0
                || is_wall(&new_point, input)
                || visited.contains(&new_point)
            {
                continue;
            }

            match end {
                Target::Distance(max_distance) if distance >= max_distance => (),
                _ => queue.push_back((distance + 1, new_point)),
            }
        }
    }

    match end {
        Target::Distance(_) => visited.len(),
        _ => panic!("No path found"),
    }
}

fn problem1(input: &str) -> usize {
    bfs(
        (1, 1),
        Target::Point((31, 39)),
        input.trim().parse().unwrap(),
    )
}

fn problem2(input: &str) -> usize {
    bfs((1, 1), Target::Distance(50), input.trim().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = 10;
        assert_eq!(11, bfs((1, 1), Target::Point((7, 4)), input));
    }

    #[test]
    fn test_problem2() {
        let input = 10;
        assert_eq!(3, bfs((1, 1), Target::Distance(1), input));
        assert_eq!(5, bfs((1, 1), Target::Distance(2), input));
    }
}
