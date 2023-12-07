static INPUT: &str = include_str!("input.txt");
static TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let Some((time, distance)) = input.split_once('\n') else {
        panic!("no newline")
    };
    let Some((_, time)) = time.split_once(':') else {
        panic!("no colon")
    };
    let Some((_, distance)) = distance.split_once(':') else {
        panic!("no colon")
    };

    let (time, distance) = (time.trim(), distance.trim());

    time.split_whitespace()
        .map(|x| x.parse())
        .flatten()
        .zip(distance.split_whitespace().map(|x| x.parse()).flatten())
        .collect()
}

fn number_of_integers_between(a: f64, b: f64) -> u64 {
    let mi = a.min(b);
    let ma = a.max(b);
    0.max(ma.ceil() as i32 - mi.floor() as i32 - 1) as u64
}

fn number_of_ways_to_win(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;
    let t2_4d = t * t - 4. * d;

    let s = t2_4d.sqrt() / 2.;

    let ht = t / 2.;

    number_of_integers_between(ht - s, ht + s)
}

fn first_task(input: &str) -> u64 {
    let input = parse_input(input);
    input
        .iter()
        .map(|&(t, d)| number_of_ways_to_win(t, d))
        .product()
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 288);
}

fn parse_input_2(input: &str) -> (u64, u64) {
    let Some((time, distance)) = input.split_once('\n') else {
        panic!("no newline")
    };
    let Some((_, time)) = time.split_once(':') else {
        panic!("no colon")
    };
    let Some((_, distance)) = distance.split_once(':') else {
        panic!("no colon")
    };

    let (time, distance) = (time.trim(), distance.trim());

    let time: String = time.split_whitespace().collect();
    let distance: String = distance.split_whitespace().collect();

    (time.parse().unwrap(), distance.parse().unwrap())
}

fn second_task(input: &str) -> u64 {
    let (t, d) = parse_input_2(input);
    number_of_ways_to_win(t, d)
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT), 71503);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
