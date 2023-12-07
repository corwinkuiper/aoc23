use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl Rank {
    fn generate_rank(ipt: &[CardValue; 5]) -> Rank {
        let mut value_totals = HashMap::new();

        for &c in ipt.iter() {
            *value_totals.entry(c).or_insert(0) += 1;
        }

        let mut vals: Vec<_> = value_totals.iter().map(|(c, v)| (*c, *v)).collect();

        vals.sort_by_key(|x| -x.1);

        if vals[0].1 == 5 {
            return Rank::FiveOfKind;
        }

        if vals[0].1 == 4 {
            return Rank::FourOfKind;
        }

        if vals[0].1 == 3 && vals[1].1 == 2 {
            return Rank::FullHouse;
        }

        if vals[0].1 == 3 {
            return Rank::ThreeOfKind;
        }

        if vals[0].1 == 2 && vals[1].1 == 2 {
            return Rank::TwoPair;
        }

        if vals[0].1 == 2 {
            return Rank::OnePair;
        }

        return Rank::HighCard;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct CardValue {
    value: u32,
}

impl CardValue {
    fn from_char(c: char) -> Result<Self, &'static str> {
        let value = |v: u32| CardValue { value: v };

        if let Some(d) = c.to_digit(10) {
            return Ok(value(d));
        }

        Ok(match c {
            'T' => value(10),
            'J' => value(11),
            'Q' => value(12),
            'K' => value(13),
            'A' => value(14),
            _ => return Err("not good"),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    rank: Rank,
    hand: [CardValue; 5],
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_values: Result<Vec<_>, _> = s.chars().map(|c| CardValue::from_char(c)).collect();
        let char_values = char_values?;

        let hand: [CardValue; 5] = [
            char_values[0],
            char_values[1],
            char_values[2],
            char_values[3],
            char_values[4],
        ];

        Ok(Hand {
            rank: Rank::generate_rank(&hand),
            hand,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct HandBid {
    hand: Hand,
    bid: u64,
}

impl FromStr for HandBid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').ok_or("can't split a line")?;

        Ok(HandBid {
            hand: hand.parse()?,
            bid: bid.parse().map_err(|_| "can't make number")?,
        })
    }
}

pub fn first_task(input: &str) -> Result<u64, &'static str> {
    let hands: Result<Vec<HandBid>, _> = input.lines().map(|line| line.parse()).collect();
    let mut hands = hands?;

    hands.sort_by_key(|x| x.hand);

    Ok(hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u64 + 1))
        .sum())
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(crate::TEST_INPUT), Ok(6440));
}
