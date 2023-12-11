use std::{cell::Cell, collections::HashSet, io::BufRead};

static INPUT: &[u8] = include_bytes!("input.txt");
static TEST_INPUT_1: &[u8] = b"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

static TEST_INPUT_2: &[u8] = b".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

struct Map {
    input: &'static [u8],
    width: i32,
    height: i32,
    start_symbol: Cell<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_component(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

trait Pipe {
    fn has_south_connection(self) -> bool;
    fn has_north_connection(self) -> bool;
    fn has_east_connection(self) -> bool;
    fn has_west_connection(self) -> bool;
}

const PIPE_SYMBOLS: &[u8] = b"|7FLJ-";

impl Pipe for u8 {
    fn has_south_connection(self) -> bool {
        matches!(self, b'|' | b'7' | b'F')
    }

    fn has_north_connection(self) -> bool {
        matches!(self, b'|' | b'L' | b'J')
    }

    fn has_east_connection(self) -> bool {
        matches!(self, b'-' | b'L' | b'F')
    }

    fn has_west_connection(self) -> bool {
        matches!(self, b'-' | b'J' | b'7')
    }
}

impl Map {
    fn new(input: &'static [u8]) -> Self {
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().unwrap().len() as i32;

        Self {
            input,
            width,
            height,
            start_symbol: Cell::new(b'.'),
        }
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return b'.';
        }
        let s = self.input[(x + y * (self.width + 1)) as usize];
        if s == b'S' && self.start_symbol.get() != b'.' {
            self.start_symbol.get()
        } else {
            s
        }
    }

    fn find_start(&self) -> (i32, i32) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == b'S' {
                    return (x, y);
                }
            }
        }

        panic!("no start found");
    }

    fn starting_connections(&self, x: i32, y: i32) -> [Direction; 2] {
        let connections: [Direction; 2] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .map(|d| {
            let (xx, yy) = d.to_component();
            (self.get(x + xx, y + yy), d)
        })
        .filter(|(c, d)| match d {
            Direction::North => c.has_south_connection(),
            Direction::East => c.has_west_connection(),
            Direction::South => c.has_north_connection(),
            Direction::West => c.has_east_connection(),
        })
        .map(|(_, x)| *x)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

        let symbol = PIPE_SYMBOLS
            .iter()
            .filter(|x| {
                connections.iter().all(|d| match d {
                    Direction::North => x.has_north_connection(),
                    Direction::East => x.has_east_connection(),
                    Direction::South => x.has_south_connection(),
                    Direction::West => x.has_west_connection(),
                })
            })
            .next()
            .unwrap();

        self.start_symbol.set(*symbol);

        connections
    }

    fn connecting_to(&self, x: i32, y: i32, from: Direction) -> Direction {
        let p = self.get(x, y);
        let connections = [
            (p.has_north_connection(), Direction::North),
            (p.has_east_connection(), Direction::East),
            (p.has_south_connection(), Direction::South),
            (p.has_west_connection(), Direction::West),
        ];

        connections
            .iter()
            .filter(|(h, d)| *h && *d != from.opposite())
            .next()
            .unwrap()
            .1
    }

    fn make_pipe_set(&self) -> HashSet<(i32, i32)> {
        let start = self.find_start();

        let mut visited = HashSet::new();

        visited.insert(start);

        let start_directions = self.starting_connections(start.0, start.1);

        let mut current_position = start;
        let mut current_direction = start_directions[0];

        loop {
            let direction_component = current_direction.to_component();

            let next_position = (
                current_position.0 + direction_component.0,
                current_position.1 + direction_component.1,
            );

            if !visited.insert(next_position) {
                break;
            }

            let next_direction =
                self.connecting_to(next_position.0, next_position.1, current_direction);

            current_direction = next_direction;
            current_position = next_position;
        }

        visited
    }
}

fn first_task(input: &'static [u8]) -> u64 {
    let map = Map::new(input);
    map.make_pipe_set().len() as u64 / 2
}

#[test]
fn check_fist_task() {
    assert_eq!(first_task(TEST_INPUT_1), 8);
}

fn second_task(input: &'static [u8]) -> u64 {
    let map = Map::new(input);
    let s = map.make_pipe_set();

    let mut inside_count = 0;

    for y in 0..map.height {
        let mut inside = false;
        let mut in_horizontal_pipe_section = None;

        for x in 0..map.width {
            let is_pipe = s.contains(&(x, y));
            if !is_pipe && inside {
                inside_count += 1;
            }

            if is_pipe {
                let pipe = map.get(x, y);
                if let Some(hor) = in_horizontal_pipe_section {
                    if hor == Direction::North && pipe.has_south_connection()
                        || hor == Direction::South && pipe.has_north_connection()
                    {
                        inside = !inside;
                    }
                    if pipe.has_north_connection() || pipe.has_south_connection() {
                        in_horizontal_pipe_section = None;
                    }
                } else if pipe.has_north_connection() && pipe.has_south_connection() {
                    inside = !inside;
                } else {
                    if pipe.has_north_connection() {
                        in_horizontal_pipe_section = Some(Direction::North);
                    }
                    if pipe.has_south_connection() {
                        in_horizontal_pipe_section = Some(Direction::South);
                    }
                }
            }
        }

        assert_eq!(inside, false);
        assert_eq!(in_horizontal_pipe_section, None);
    }

    inside_count
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT_2), 8);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
