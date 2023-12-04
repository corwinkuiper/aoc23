use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn matching_numbers() -> impl Iterator<Item = usize> {
    INPUT.lines().map(|line| {
        let round = line.split(":").nth(1).unwrap();
        let mut x = round.split("|").map(|x| x.trim());
        let winning: HashSet<u32> = x
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse())
            .flatten()
            .collect();
        let mine: HashSet<u32> = x
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse())
            .flatten()
            .collect();

        winning.intersection(&mine).count()
    })
}

fn first_task() {
    let score: u64 = matching_numbers()
        .map(|x| match x {
            0 => 0,
            x => 2_u64.pow(x as u32 - 1),
        })
        .sum();

    dbg!(score);
}

fn second_task() {
    let matches: Vec<_> = matching_numbers().collect();
    let mut total_cards = vec![0; matches.len()];
    for idx in (0..matches.len()).rev() {
        let cards_won = matches[idx];
        total_cards[idx] = 1;
        for widx in 1..=cards_won {
            total_cards[idx] += total_cards[widx + idx];
        }
    }

    let total: usize = total_cards.iter().sum();

    dbg!(total);
}

fn main() {
    first_task();
    second_task();
}
