use itertools::Itertools;
use std::collections::{hash_map::DefaultHasher, BTreeMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Element {
    Curium,
    Hydrogen,
    Lithium,
    Plutonium,
    Ruthenium,
    Strontium,
    Thulium,
}

impl Element {
    fn from_str(s: &str) -> Self {
        match s {
            "curium" => Element::Curium,
            "hydrogen" => Element::Hydrogen,
            "lithium" => Element::Lithium,
            "plutonium" => Element::Plutonium,
            "ruthenium" => Element::Ruthenium,
            "strontium" => Element::Strontium,
            "thulium" => Element::Thulium,
            _ => panic!("{}", format!("Unknown element {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Item {
    Generator(Element),
    Microchip(Element),
}

impl Item {
    fn is_safe_on_floor(&self, floor_contents: &Vec<Item>) -> bool {
        match self {
            Item::Generator(_) => true,
            Item::Microchip(mtype) => {
                // A microchip is safe if there is no generator on the floor, or if there is a generator for the same element
                let mut gens = floor_contents
                    .iter()
                    .filter(|item| match item {
                        Item::Generator(_) => true,
                        _ => false,
                    })
                    .peekable();

                gens.peek().is_none()
                    || gens.any(|item| match item {
                        Item::Generator(gtype) => gtype == mtype,
                        _ => false,
                    })
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    elevator_location: isize,
    floor_contents: BTreeMap<isize, Vec<Item>>,
    top_floor: isize,
}

fn parse_contents(parts: &[&str]) -> Vec<Item> {
    let mut result = vec![];
    if parts.len() == 0 || parts[0] == "nothing" {
        return result;
    }

    // The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.
    let mut i = 0;
    while i + 2 < parts.len() {
        let item = match parts[i + 2].replace(".", "").replace(",", "").as_str() {
            "generator" => Item::Generator(Element::from_str(parts[i + 1])),
            "microchip" => {
                Item::Microchip(Element::from_str(parts[i + 1].split('-').nth(0).unwrap()))
            }
            _ => panic!("Unknown item type"),
        };
        result.push(item);
        i += 3;
        if i < parts.len() && parts[i] == "and" {
            i += 1;
        }
    }
    result
}

impl State {
    fn new(input: &str) -> Self {
        let floor_contents = input
            .lines()
            .map(|line| {
                //The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
                let parts = line.split(" ").collect::<Vec<&str>>();
                let floor = match parts[1] {
                    "first" => 0,
                    "second" => 1,
                    "third" => 2,
                    "fourth" => 3,
                    _ => panic!("Unknown floor"),
                };
                let contents = parse_contents(&parts[4..]);
                (floor, contents)
            })
            .collect::<BTreeMap<isize, Vec<Item>>>();
        let top_floor = *floor_contents.keys().max().unwrap();
        State {
            elevator_location: 0,
            floor_contents,
            top_floor,
        }
    }

    fn is_done(&self) -> bool {
        // We're done if all contents are on the top floor
        let top_floor = self
            .floor_contents
            .iter()
            .max_by_key(|(floor, _)| *floor)
            .unwrap()
            .0;

        self.floor_contents
            .iter()
            .all(|(floor, contents)| floor == top_floor || contents.is_empty())
    }

    fn hashcode(&self) -> u64 {
        // TODO
        // Hash: [elevator, floor1_paircount, floor1_unpaired, floor2_paircount, floor2_unpaired, ...]
        let mut hasher = DefaultHasher::new();
        self.elevator_location.hash(&mut hasher);
        for (_, contents) in self.floor_contents.iter() {
            let mut pairs = contents
                .iter()
                .combinations(2)
                .filter(|combo| match combo[0] {
                    Item::Generator(gtype) => match combo[1] {
                        Item::Microchip(mtype) => gtype == mtype,
                        _ => false,
                    },
                    _ => false,
                });
            contents
                .iter()
                .filter(|item| !pairs.any(|pair| pair.contains(item)))
                .collect::<Vec<&Item>>()
                .sort()
                .hash(&mut hasher);
            pairs.count().hash(&mut hasher);
        }
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn can_hold(&self, items: &[&Item], floor: isize) -> bool {
        let mut all_contents = vec![];
        for item in self.floor_contents.get(&floor).unwrap() {
            all_contents.push(*item);
        }
        for item in items.iter() {
            all_contents.push(**item);
        }
        all_contents
            .iter()
            .all(|item| item.is_safe_on_floor(&all_contents))
    }

    fn advance(&self, items_to_move: &[&Item], dest: isize) -> State {
        let mut new_contents = self.floor_contents.clone();
        new_contents
            .get_mut(&self.elevator_location)
            .unwrap()
            .retain(|i| items_to_move.iter().all(|item| i != *item));
        new_contents
            .get_mut(&dest)
            .unwrap()
            .extend(items_to_move.iter().copied());
        State {
            elevator_location: dest,
            floor_contents: new_contents,
            top_floor: self.top_floor,
        }
    }

    fn possible_moves(&self, floor: isize) -> Vec<State> {
        if floor < 0 || floor > self.top_floor {
            return vec![];
        }

        // Rules:
        // Can't move microchip(A) and generator(B) at the same time
        // So when moving two items:
        //  1. Two microchips
        //  2. Two generators
        //  3. A matched pair (*any* pair is equivalent here)
        // When moving one item:
        //  1.

        let current_floor = self.elevator_location;
        let current_contents = self.floor_contents.get(&current_floor).unwrap();
        let mut res = vec![];

        let two_micros = current_contents
            .iter()
            .filter(|item| match item {
                Item::Microchip(_) => true,
                _ => false,
            })
            .combinations(2)
            .collect::<Vec<Vec<&Item>>>();
        let two_gens = current_contents
            .iter()
            .filter(|item| match item {
                Item::Generator(_) => true,
                _ => false,
            })
            .combinations(2)
            .collect::<Vec<Vec<&Item>>>();
        let matched = current_contents
            .iter()
            .combinations(2)
            .find(|combo| match combo[0] {
                Item::Generator(gtype) => match combo[1] {
                    Item::Microchip(mtype) => gtype == mtype,
                    _ => false,
                },
                _ => false,
            });

        two_micros
            .iter()
            .filter(|combo| self.can_hold(combo, floor))
            .for_each(|combo| res.push(self.advance(combo, floor)));
        two_gens
            .iter()
            .filter(|combo| self.can_hold(combo, floor))
            .for_each(|combo| {
                res.push(self.advance(combo, floor));
            });
        if let Some(pair) = matched {
            if self.can_hold(pair.as_slice(), floor) {
                res.push(self.advance(pair.as_slice(), floor));
            }
        }
        // Lastly, try moving one item
        for item in current_contents.iter() {
            if self.can_hold(&[item], floor) {
                res.push(self.advance(&[item], floor));
            }
        }

        res
    }

    fn next_states(&self) -> Vec<State> {
        let mut res = vec![];
        res.append(&mut self.possible_moves(self.elevator_location + 1));
        res.append(&mut self.possible_moves(self.elevator_location - 1));
        res
    }

    fn bfs(&self) -> i32 {
        let start_timestamp = std::time::Instant::now();
        let mut max_floor = 0;
        // I can't call `.clone()` on self because it returns a &State
        let start = State {
            elevator_location: 0,
            floor_contents: self.floor_contents.clone(),
            top_floor: self.top_floor,
        };
        let hashcode = start.hashcode();
        let mut queue = VecDeque::from(vec![(0, start)]);
        let mut seen = HashSet::new();
        seen.insert(hashcode);
        while let Some((steps, state)) = queue.pop_front() {
            if steps > max_floor {
                max_floor = steps;
                let elapsed = start_timestamp.elapsed().as_millis();
                println!("Reached floor {}, Time: {}ms", max_floor, elapsed);
            }
            if state.is_done() {
                return steps;
            }
            for next_state in state.next_states() {
                let hashcode = next_state.hashcode();
                if seen.contains(&hashcode) {
                    continue;
                }
                seen.insert(hashcode);
                queue.push_back((steps + 1, next_state));
            }
        }
        -1
    }
}

fn problem1(input: &str) -> i32 {
    State::new(input).bfs()
}

fn problem2(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        assert_eq!(problem1(input), 11);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
    }
}
