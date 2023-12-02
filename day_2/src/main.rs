use std::sync::OnceLock;

use regex::Regex;

const INPUT: &str = include_str!("input.txt");

#[derive(Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn max_of(a: Self, b: Self) -> Self {
        Draw {
            red: a.red.max(b.red),
            green: a.green.max(b.green),
            blue: a.blue.max(b.blue),
        }
    }
}

fn game_regex() -> &'static Regex {
    static GAME_REGEX: OnceLock<Regex> = OnceLock::new();
    GAME_REGEX.get_or_init(|| Regex::new(r"Game (\d*)").unwrap())
}

fn draw_regex() -> &'static Regex {
    static GAME_REGEX: OnceLock<Regex> = OnceLock::new();
    GAME_REGEX.get_or_init(|| Regex::new(r"(\d*) (red|blue|green)").unwrap())
}

fn input_parse(input: &str) -> (u32, impl Iterator<Item = Draw> + '_) {
    let mut s = input.split(':').into_iter();
    let game_count = s.next().unwrap();
    let draws = s.next().unwrap();
    let game_count: u32 = game_regex()
        .captures(game_count)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let draw_re = draw_regex();
    (
        game_count,
        draws.split(';').into_iter().map(|draw| {
            let mut cubes = Draw::default();
            for cube in draw.split(',') {
                let draw_capture = draw_re.captures(cube).unwrap();
                let count: u32 = draw_capture.get(1).unwrap().as_str().parse().unwrap();
                let colour_name = draw_capture.get(2).unwrap().as_str();
                let r = match colour_name {
                    "red" => &mut cubes.red,
                    "blue" => &mut cubes.blue,
                    "green" => &mut cubes.green,
                    _ => panic!("unknown colour {}", colour_name),
                };
                *r = count;
            }
            cubes
        }),
    )
}

fn first_task() {
    let answer: u32 = INPUT
        .lines()
        .map(|line| input_parse(line))
        .map(|(gc, draws)| {
            (
                gc,
                draws
                    .reduce(|acc, e| Draw::max_of(acc, e))
                    .unwrap_or(Draw::default()),
            )
        })
        .filter(|(_, game)| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|(gc, _)| gc)
        .sum();

    dbg!(answer);
}

fn second_task() {
    let answer: u32 = INPUT
        .lines()
        .map(|line| input_parse(line))
        .map(|(gc, draws)| {
            (
                gc,
                draws
                    .reduce(|acc, e| Draw::max_of(acc, e))
                    .unwrap_or(Draw::default()),
            )
        })
        .map(|(_, game)| game.red * game.green * game.blue)
        .sum();

    dbg!(answer);
}

fn main() {
    first_task();
    second_task();
}
