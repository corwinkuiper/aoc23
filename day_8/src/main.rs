use std::{collections::HashMap, fmt::Debug, str::FromStr};

const ALT_TEST_INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

const TEST_INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

const TEST_2_INPUT: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Location {
    code: [u8; 3],
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Location")
            .field("code", &core::str::from_utf8(&self.code).unwrap_or("---"))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    left: Location,
    right: Location,
}

impl FromStr for Location {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err("bad location");
        }

        let code = s.as_bytes().try_into().map_err(|_| "shouldn't happen")?;

        Ok(Location { code })
    }
}

fn construct_map(input: &str) -> HashMap<Location, Node> {
    let mut map = HashMap::new();

    for l in input.lines() {
        let (location, lr) = l.split_once(" = ").unwrap();

        let lr = lr.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let (left, right) = lr.split_once(", ").unwrap();

        map.insert(
            location.parse().unwrap(),
            Node {
                left: left.parse().unwrap(),
                right: right.parse().unwrap(),
            },
        );
    }

    map
}

fn first_location(nodes: &str) -> Location {
    let (first, _) = nodes.split_once(" = ").unwrap();

    first.parse().unwrap()
}

fn first_task(input: &str) -> u64 {
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let map = construct_map(nodes);

    let mut current_node: Location = "AAA".parse().unwrap();
    let target_node: Location = "ZZZ".parse().unwrap();

    for (direction, count) in core::iter::repeat(directions.as_bytes())
        .flatten()
        .copied()
        .zip(1_u64..)
    {
        let node = map.get(&current_node).unwrap();
        let next_node = match direction {
            b'L' => node.left,
            b'R' => node.right,
            _ => panic!("bad direction"),
        };

        if next_node == target_node {
            return count;
        }

        current_node = next_node;
    }

    panic!("infinite loop terminated unexpectedly")
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 6);
    assert_eq!(first_task(ALT_TEST_INPUT), 2);
}

fn how_long_till_end(
    mut location: Location,
    nodes: &HashMap<Location, Node>,
    directions: &str,
) -> u64 {
    for (direction, count) in core::iter::repeat(directions.as_bytes())
        .flatten()
        .copied()
        .zip(1_u64..)
    {
        let node = nodes.get(&location).unwrap();
        let next_node = match direction {
            b'L' => node.left,
            b'R' => node.right,
            _ => panic!("bad direction"),
        };
        location = next_node;

        if location.code[2] == b'Z' {
            return count;
        }
    }

    panic!("unexpected end of infinite loop")
}

fn second_task(input: &str) -> u64 {
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let map = construct_map(nodes);

    let locations: Vec<Location> = map.keys().filter(|x| x.code[2] == b'A').copied().collect();

    locations
        .iter()
        .map(|l| how_long_till_end(*l, &map, directions))
        .reduce(|acc, a| num::integer::lcm(acc, a))
        .unwrap()
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_2_INPUT), 6);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
