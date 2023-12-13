#[cfg(test)]
static TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

static INPUT: &str = include_str!("input.txt");

struct Terrain<'a> {
    width: i32,
    height: i32,
    data: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ground {
    Ash,
    Rock,
}

impl<'a> Terrain<'a> {
    fn make_from_str(input: &'a str) -> Self {
        let width = input.split_once('\n').unwrap().0.len();
        let height = input.lines().count();

        Self {
            data: input.as_bytes(),
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
        match self.data[(x + y * (self.width + 1)) as usize] {
            b'#' => Ground::Rock,
            b'.' => Ground::Ash,
            _ => panic!("bad data in map"),
        }
    }

    fn row(&self, y: i32) -> impl Iterator<Item = Ground> + '_ {
        (0..self.width).map(move |x| self.get(x, y))
    }
    fn column(&self, x: i32) -> impl Iterator<Item = Ground> + '_ {
        (0..self.height).map(move |y| self.get(x, y))
    }
}

fn count_differences<I: Iterator<Item = Ground>>(
    a: I,
    b: I,
    difference_limit: usize,
) -> Option<usize> {
    let mut difference_count = 0;
    for (aa, bb) in a.zip(b) {
        if aa != bb {
            difference_count += 1;
            if difference_count > difference_limit {
                return None;
            }
        }
    }

    return Some(difference_count);
}

fn find_matching<F, I>(limit: i32, matching_thing: F, difference_count: usize) -> Option<i32>
where
    F: Fn(i32) -> I,
    I: Iterator<Item = Ground>,
{
    'outer: for (a, b) in (0..limit - 1).map(|a| (a, a + 1)) {
        let aa = matching_thing(a);
        let bb = matching_thing(b);

        if let Some(mut accumulated_difference) = count_differences(aa, bb, difference_count) {
            for (aaa, bbb) in (0..a).rev().zip(b + 1..limit) {
                let aa = matching_thing(aaa);
                let bb = matching_thing(bbb);

                if let Some(additional_difference) =
                    count_differences(aa, bb, difference_count - accumulated_difference)
                {
                    accumulated_difference += additional_difference;
                } else {
                    continue 'outer;
                }
            }

            if accumulated_difference != difference_count {
                continue 'outer;
            }

            return Some(a + 1);
        }
    }

    None
}

fn calculate_mirrors(input: &str, difference_count: usize) -> u64 {
    input
        .split("\n\n")
        .map(|x| Terrain::make_from_str(x))
        .map(|x| {
            find_matching(x.height, |y| x.row(y), difference_count)
                .map(|x| x * 100)
                .unwrap_or_else(|| {
                    find_matching(x.width, |xx| x.column(xx), difference_count).unwrap()
                })
        })
        .map(|x| TryInto::<u64>::try_into(x).unwrap())
        .sum()
}

#[test]
fn check_first_task() {
    assert_eq!(calculate_mirrors(TEST_INPUT, 0), 405);
}

#[test]
fn check_second_task() {
    assert_eq!(calculate_mirrors(TEST_INPUT, 1), 400);
}

fn main() {
    dbg!(calculate_mirrors(INPUT, 0));
    dbg!(calculate_mirrors(INPUT, 1));
}
