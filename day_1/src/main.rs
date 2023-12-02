const INPUT: &str = include_str!("input.txt");

fn digits(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.chars().filter_map(|x| x.to_digit(10))
}

fn first_task() {
    let x: u32 = INPUT
        .lines()
        .map(|line| {
            let first = digits(line).next().unwrap();
            let last = digits(line).last().unwrap();

            first * 10 + last
        })
        .sum();

    dbg!(x);
}

const NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_number(mut s: &str) -> u32 {
    while s.len() != 0 {
        if s.as_bytes().first().unwrap().is_ascii_digit() {
            let digit = (*s.as_bytes().first().unwrap() - b"0"[0]) as u32;
            return digit;
        }
        for (num, number_str) in NUMBERS.iter().enumerate() {
            if s.starts_with(number_str) {
                return num as u32 + 1;
            }
        }
        s = &s[1..];
    }

    panic!("can't find first!!!")
}

fn find_last_number(mut s: &str) -> u32 {
    while s.len() != 0 {
        if s.as_bytes().last().unwrap().is_ascii_digit() {
            let digit = (*s.as_bytes().last().unwrap() - b"0"[0]) as u32;
            return digit;
        }
        for (num, number_str) in NUMBERS.iter().enumerate() {
            if s.ends_with(number_str) {
                return num as u32 + 1;
            }
        }
        s = &s[..s.len() - 1];
    }

    panic!("can't find last!!!")
}

fn second_task() {
    let x: u32 = INPUT
        .lines()
        .map(|line| {
            let first = find_first_number(line);
            let last = find_last_number(line);

            first * 10 + last
        })
        .sum();

    dbg!(x);
}

fn main() {
    first_task();
    second_task();
}
