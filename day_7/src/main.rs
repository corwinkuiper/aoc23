mod first;
mod second;

const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

const INPUT: &str = include_str!("input.txt");

fn main() {
    dbg!(first::first_task(INPUT));
    dbg!(second::second_task(INPUT));
}
