use std::{collections::HashMap, fmt::Display};

static TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

static INPUT: &str = include_str!("input.txt");

#[derive(Hash, PartialEq, Eq, Clone)]
struct Mirror {
    width: i32,
    height: i32,
    data: Vec<Ground>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Ground {
    Surface,
    Rock,
    RoundedRock,
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .data
            .chunks_exact(self.width as usize)
            .map(|x| {
                x.iter()
                    .map(|x| match x {
                        Ground::Surface => '.',
                        Ground::Rock => '#',
                        Ground::RoundedRock => 'O',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        s.fmt(f)
    }
}

impl Mirror {
    fn make_from_str(input: &str) -> Self {
        let width = input.split_once('\n').unwrap().0.len();
        let height = input.lines().count();

        let data = input
            .as_bytes()
            .iter()
            .filter(|x| **x != b'\n')
            .map(|x| match x {
                b'#' => Ground::Rock,
                b'.' => Ground::Surface,
                b'O' => Ground::RoundedRock,
                _ => panic!("bad data in map"),
            })
            .collect();

        Self {
            data,
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        }
    }

    fn get(&self, x: i32, y: i32) -> Ground {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            panic!(
                "out of bounds access, width {} height {}, x {} y {}",
                self.width, self.height, x, y
            )
        }
        self.data[(x + y * self.width) as usize]
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut Ground {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            panic!(
                "out of bounds access, width {} height {}, x {} y {}",
                self.width, self.height, x, y
            )
        }
        &mut self.data[(x + y * self.width) as usize]
    }

    fn row(&self, y: i32) -> impl Iterator<Item = Ground> + '_ {
        (0..self.width).map(move |x| self.get(x, y))
    }
    fn column(&self, x: i32) -> impl Iterator<Item = Ground> + '_ {
        (0..self.height).map(move |y| self.get(x, y))
    }
}

fn tilt_vector(mirror: &mut Mirror, vector: (i32, i32)) {
    fn do_point(mirror: &mut Mirror, x: i32, y: i32, vector: (i32, i32)) {
        let (mut xx, mut yy) = (x, y);
        if mirror.get(xx, yy) == Ground::RoundedRock {
            *mirror.get_mut(xx, yy) = Ground::Surface;
            while xx + vector.0 >= 0
                && yy + vector.1 >= 0
                && xx + vector.0 < mirror.width
                && yy + vector.1 < mirror.height
                && mirror.get(xx + vector.0, yy + vector.1) == Ground::Surface
            {
                (xx, yy) = (xx + vector.0, yy + vector.1);
            }
            *mirror.get_mut(xx, yy) = Ground::RoundedRock;
        }
    }

    if vector.0 < 0 || vector.1 < 0 {
        for y in 0..mirror.height {
            for x in 0..mirror.width {
                do_point(mirror, x, y, vector);
            }
        }
    } else {
        for y in (0..mirror.height).rev() {
            for x in (0..mirror.width).rev() {
                do_point(mirror, x, y, vector);
            }
        }
    }
}

fn calculate_load(mirror: &Mirror) -> u64 {
    (0..mirror.height)
        .map(|x| mirror.row(x).filter(|&x| x == Ground::RoundedRock).count())
        .enumerate()
        .map(|(idx, count)| (mirror.height as u64 - idx as u64) * count as u64)
        .sum()
}

fn first_task(input: &str) -> u64 {
    let mut mirror = Mirror::make_from_str(input);

    tilt_vector(&mut mirror, (0, -1));

    calculate_load(&mirror)
}

fn second_task(input: &str) -> u64 {
    let mut mirror = Mirror::make_from_str(input);

    let mut cache: HashMap<Mirror, usize> = HashMap::new();

    let repeat_times = 1000000000;

    for idx in 0..repeat_times {
        if let Some(&last_encounter) = cache.get(&mirror) {
            let distance_since = idx - last_encounter;
            if (repeat_times - idx) % distance_since == 0 {
                break;
            }
        } else {
            cache.insert(mirror.clone(), idx);
        }
        tilt_vector(&mut mirror, (0, -1));
        tilt_vector(&mut mirror, (-1, 0));
        tilt_vector(&mut mirror, (0, 1));
        tilt_vector(&mut mirror, (1, 0));
    }

    calculate_load(&mirror)
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 136);
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT), 64);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
