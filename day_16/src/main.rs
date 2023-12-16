use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
};

#[cfg(test)]
static TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

static INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
struct Mirror {
    elem: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vector2d(i32, i32);

impl From<(i32, i32)> for Vector2d {
    fn from(value: (i32, i32)) -> Self {
        Vector2d(value.0, value.1)
    }
}

impl Add for Vector2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2d(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vector2d {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_vector(self) -> Vector2d {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
        .into()
    }
}

enum MirrorEncounter {
    Stop,
    GoInDirection(Direction),
    SplitInDirections([Direction; 2]),
}

impl From<Direction> for MirrorEncounter {
    fn from(value: Direction) -> Self {
        MirrorEncounter::GoInDirection(value)
    }
}

impl From<(Direction, Direction)> for MirrorEncounter {
    fn from(value: (Direction, Direction)) -> Self {
        MirrorEncounter::SplitInDirections([value.0, value.1])
    }
}

impl Mirror {
    fn behaviour(&self, direction: Direction) -> MirrorEncounter {
        match (self.elem, direction) {
            (b'/', Direction::North) => Direction::East.into(),
            (b'/', Direction::East) => Direction::North.into(),
            (b'/', Direction::South) => Direction::West.into(),
            (b'/', Direction::West) => Direction::South.into(),
            (b'\\', Direction::North) => Direction::West.into(),
            (b'\\', Direction::East) => Direction::South.into(),
            (b'\\', Direction::South) => Direction::East.into(),
            (b'\\', Direction::West) => Direction::North.into(),
            (b'|', Direction::East | Direction::West) => {
                (Direction::North, Direction::South).into()
            }
            (b'|', Direction::North | Direction::South) => direction.into(),
            (b'-', Direction::East | Direction::West) => direction.into(),
            (b'-', Direction::North | Direction::South) => {
                (Direction::East, Direction::West).into()
            }
            (b'.', _) => direction.into(),
            _ => MirrorEncounter::Stop,
        }
    }
}

struct MirrorMap<'a> {
    map: &'a str,
    width: i32,
    height: i32,
}

impl<'a> MirrorMap<'a> {
    fn make_from_input(s: &'a str) -> Self {
        let width = s.split_once('\n').unwrap().0.len();
        let height = s.lines().count();

        Self {
            map: s,
            width: width as i32,
            height: height as i32,
        }
    }

    fn get(&self, pos: Vector2d) -> Mirror {
        if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
            Mirror { elem: b'E' }
        } else {
            Mirror {
                elem: self.map.as_bytes()[(pos.0 + pos.1 * (self.width + 1)) as usize],
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Encountered {
    position: Vector2d,
    direction: Direction,
}

fn add_encounters_to_set(
    set: &mut HashSet<Encountered>,
    map: &MirrorMap<'_>,
    position: Vector2d,
    direction: Direction,
) {
    let behaviour = map.get(position).behaviour(direction);

    if !matches!(behaviour, MirrorEncounter::Stop)
        && !set.insert(Encountered {
            position,
            direction,
        })
    {
        return;
    }

    match behaviour {
        MirrorEncounter::Stop => {}
        MirrorEncounter::GoInDirection(next_direction) => {
            add_encounters_to_set(
                set,
                map,
                position + next_direction.to_vector(),
                next_direction,
            );
        }
        MirrorEncounter::SplitInDirections([a, b]) => {
            add_encounters_to_set(set, map, position + a.to_vector(), a);
            add_encounters_to_set(set, map, position + b.to_vector(), b);
        }
    }
}

fn first_task(input: &str) -> u64 {
    let map = MirrorMap::make_from_input(input);

    let mut set = HashSet::new();

    add_encounters_to_set(&mut set, &map, (0, 0).into(), Direction::East);

    let locations = set.iter().map(|x| x.position).collect::<HashSet<_>>();

    locations.len().try_into().unwrap()
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 46);
}

fn second_task(input: &str) -> u64 {
    let map = MirrorMap::make_from_input(input);

    (0..map.width)
        .flat_map(|x| {
            [
                ((x, 0), Direction::South),
                ((x, map.height), Direction::North),
            ]
        })
        .chain(
            (0..map.height)
                .flat_map(|y| [((0, y), Direction::East), ((map.width, y), Direction::West)]),
        )
        .map(|(position, direction)| {
            let mut set = HashSet::new();

            add_encounters_to_set(&mut set, &map, position.into(), direction);

            let locations = set.iter().map(|x| x.position).collect::<HashSet<_>>();
            locations.len().try_into().unwrap()
        })
        .max()
        .unwrap()
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT), 51);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
