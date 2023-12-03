use std::{collections::HashSet, io::BufRead};

const INPUT: &[u8] = include_bytes!("input.txt");

struct Input2d<'a> {
    input: &'a [u8],
    width: i32,
    height: i32,
}

impl<'a> Input2d<'a> {
    fn new(input: &'a [u8]) -> Self {
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().unwrap().len() as i32;

        Self {
            input,
            width,
            height,
        }
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return b'.';
        }
        self.input[(x + y * (self.width + 1)) as usize]
    }
}

#[test]
fn check_get() {
    let input = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let input = Input2d::new(input);
    assert_eq!(input.get(0, 0), b'4');
    assert_eq!(input.get(0, 4), b'6');
    assert_eq!(input.get(9, 3), b'.');
    assert_eq!(input.get(10, 3), b'.');
    assert_eq!(input.get(11, 3), b'.');
}

const SURROUNDING: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn first_task() {
    let input = Input2d::new(INPUT);

    let mut sum = 0;

    dbg!(input.height, input.width);

    for y in 0..input.height {
        let mut is_part_number = false;
        let mut current_number = 0;
        for x in 0..(input.width + 1) {
            let value = input.get(x, y);
            if value.is_ascii_digit() {
                if !is_part_number {
                    is_part_number |=
                        SURROUNDING
                            .iter()
                            .map(|(xx, yy)| (xx + x, yy + y))
                            .any(|(x, y)| {
                                let value = input.get(x, y);
                                !value.is_ascii_digit() && value != b'.'
                            });
                }
                current_number *= 10;
                current_number += (value - b'0') as i32;
            } else {
                if is_part_number {
                    sum += current_number;
                }
                is_part_number = false;
                current_number = 0;
            }
        }
    }

    dbg!(sum);
}

fn grab_number_from_input(input: &Input2d, mut x: i32, y: i32) -> (i32, Vec<(i32, i32)>) {
    while input.get(x, y).is_ascii_digit() {
        x -= 1;
    }
    x += 1;
    let mut number = 0;
    let mut part_of_number = Vec::new();
    while input.get(x, y).is_ascii_digit() {
        number *= 10;
        number += (input.get(x, y) - b'0') as i32;
        part_of_number.push((x, y));
        x += 1;
    }

    (number, part_of_number)
}

fn second_task() {
    let input = Input2d::new(INPUT);

    let mut total = 0;

    for y in 0..input.height {
        for x in 0..(input.width) {
            let value = input.get(x, y);
            if value == b'*' {
                let places_to_check: Vec<_> = SURROUNDING
                    .iter()
                    .map(|(xx, yy)| (xx + x, yy + y))
                    .collect();
                let mut checked_locations: HashSet<(i32, i32)> = HashSet::new();
                let mut grabbed_numbers = Vec::new();
                for &(x, y) in places_to_check.iter() {
                    if checked_locations.contains(&(x, y)) {
                        continue;
                    }
                    if input.get(x, y).is_ascii_digit() {
                        let (number, mut checked_areas) = grab_number_from_input(&input, x, y);
                        checked_locations.extend(checked_areas.iter());
                        grabbed_numbers.push(number);
                    }
                }
                if grabbed_numbers.len() == 2 {
                    total += grabbed_numbers
                        .iter()
                        .copied()
                        .reduce(|acc, a| acc * a)
                        .unwrap();
                }
            }
        }
    }

    dbg!(total);
}

fn main() {
    first_task();
    second_task();
}
