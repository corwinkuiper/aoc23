use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

static TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

struct Map {
    initial_width: usize,
    initial_height: usize,
    galaxies: HashSet<(usize, usize)>,
}

impl Map {
    fn construct(input: &str) -> Self {
        let Some((first, _)) = input.split_once('\n') else {
            panic!("bad input");
        };

        let width = first.len();
        let height = input.lines().count();

        let galaxies = input
            .lines()
            .enumerate()
            .map(|(y, x)| x.chars().enumerate().map(move |(x, c)| ((x, y), c)))
            .flatten()
            .filter_map(|((x, y), c)| if c == '#' { Some((x, y)) } else { None })
            .collect();

        Self {
            initial_width: width,
            initial_height: height,
            galaxies,
        }
    }

    fn expand_x(&mut self, by: usize) {
        let mut columns = Vec::new();
        columns.resize(self.initial_width as usize, 0);

        for (x, _) in self.galaxies.iter() {
            columns[*x] += 1;
        }

        let a = extended(&columns);

        self.galaxies = self
            .galaxies
            .iter()
            .map(|&(x, y)| (x + a[x] * by, y))
            .collect();
    }

    fn expand_y(&mut self, by: usize) {
        let mut rows = Vec::new();
        rows.resize(self.initial_height as usize, 0);

        for (_, y) in self.galaxies.iter() {
            rows[*y] += 1;
        }

        let a = extended(&rows);

        self.galaxies = self
            .galaxies
            .iter()
            .map(|&(x, y)| (x, y + a[y] * by))
            .collect();
    }
}

fn extended(v: &[usize]) -> Vec<usize> {
    let mut c = 0;
    v.iter()
        .map(|&x| {
            if x == 0 {
                c += 1
            }
            c
        })
        .collect()
}

fn first_task(input: &str, by: usize) -> i64 {
    let mut map = Map::construct(input);
    map.expand_x(by);
    map.expand_y(by);

    map.galaxies
        .iter()
        .map(|&c| map.galaxies.iter().map(move |&e| (e, c)))
        .flatten()
        .map(|(a, b)| (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs())
        .sum::<i64>()
        / 2
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT, 1), 374);
}

#[test]
fn check_second_task() {
    assert_eq!(first_task(TEST_INPUT, 9), 1030);
    assert_eq!(first_task(TEST_INPUT, 99), 8410);
}

fn main() {
    dbg!(first_task(INPUT, 1));
    dbg!(first_task(INPUT, 1000000 - 1));
}
