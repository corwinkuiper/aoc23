const TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

const INPUT: &str = include_str!("input.txt");

fn generate_reduced_numbers(numbers: Vec<i64>) -> Vec<Vec<i64>> {
    let mut reduced_numbers: Vec<Vec<i64>> = Vec::new();
    reduced_numbers.push(numbers);

    loop {
        let last = reduced_numbers.last().unwrap();
        {
            let f = last[0];
            if last.iter().all(|x| *x == f) {
                break;
            }
        }

        reduced_numbers.push(
            last.windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        );
    }

    reduced_numbers
}

fn first_task(input: &str) -> i64 {
    input
        .lines()
        .map(|x| {
            let numbers: Vec<_> = x
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            generate_reduced_numbers(numbers)
                .iter()
                .map(|x| x.last())
                .flatten()
                .sum::<i64>()
        })
        .sum()
}

fn second_task(input: &str) -> i64 {
    input
        .lines()
        .map(|x| {
            let numbers: Vec<_> = x
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let reduced = generate_reduced_numbers(numbers)
                .iter()
                .rev()
                .map(|x| x.first())
                .flatten()
                .copied()
                .reduce(|acc, a| a - acc)
                .unwrap();
            reduced
        })
        .sum()
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 114);
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT), 2);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
