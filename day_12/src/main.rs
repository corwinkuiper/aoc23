use rayon::prelude::*;
use std::{iter, str::FromStr};

static INPUT: &str = include_str!("input.txt");

static TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for SpringCondition {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => SpringCondition::Damaged,
            '.' => SpringCondition::Operational,
            '?' => SpringCondition::Unknown,
            _ => return Err("not a valid spring type"),
        })
    }
}

#[derive(Clone, Debug)]
struct Springs {
    condition: Vec<SpringCondition>,
    damaged_springs: Vec<u32>,
}

impl FromStr for Springs {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((condition, groups)) = s.split_once(' ') else {
            return Err("bad spring data");
        };

        let condition: Result<Vec<_>, _> = condition.chars().map(|x| x.try_into()).collect();
        let damaged_springs: Result<Vec<_>, _> = groups
            .split(',')
            .map(|x| x.parse().map_err(|_| "bad number"))
            .collect();

        Ok(Self {
            condition: condition?,
            damaged_springs: damaged_springs?,
        })
    }
}

struct ValidResult {
    encountered_unknown: bool,
    valid: bool,
}

impl Springs {
    fn expand(&mut self) {
        let x = self.condition.clone();
        self.condition.push(SpringCondition::Unknown);
        self.condition.append(&mut x.clone());
        self.condition.push(SpringCondition::Unknown);
        self.condition.append(&mut x.clone());
        self.condition.push(SpringCondition::Unknown);
        self.condition.append(&mut x.clone());
        self.condition.push(SpringCondition::Unknown);
        self.condition.append(&mut x.clone());

        let b = self.damaged_springs.clone();
        self.damaged_springs.append(&mut b.clone());
        self.damaged_springs.append(&mut b.clone());
        self.damaged_springs.append(&mut b.clone());
        self.damaged_springs.append(&mut b.clone());
    }

    fn valid_up_to_unknown(&self) -> ValidResult {
        let mut group_iter = self.damaged_springs.iter().peekable();
        let mut current_damaged_group_length = 0;

        for &cond in self
            .condition
            .iter()
            .chain(iter::once(&SpringCondition::Operational))
        {
            if cond == SpringCondition::Damaged {
                current_damaged_group_length += 1;
            }
            if cond == SpringCondition::Unknown {
                return ValidResult {
                    encountered_unknown: true,
                    valid: true,
                };
            }
            if cond == SpringCondition::Operational {
                if current_damaged_group_length != 0 {
                    let Some(&expected_length) = group_iter.next() else {
                        return ValidResult {
                            encountered_unknown: false,
                            valid: false,
                        };
                    };
                    if expected_length != current_damaged_group_length {
                        return ValidResult {
                            encountered_unknown: false,
                            valid: false,
                        };
                    };

                    current_damaged_group_length = 0;
                }
            }
        }

        if group_iter.next().is_some() {
            ValidResult {
                encountered_unknown: false,
                valid: false,
            }
        } else {
            ValidResult {
                encountered_unknown: false,
                valid: true,
            }
        }
    }

    fn attempt_direction(&mut self, idx: usize, condition: SpringCondition) -> u64 {
        self.condition[idx] = condition;
        let res = self.valid_up_to_unknown();
        if res.valid {
            if !res.encountered_unknown {
                1
            } else {
                self.count_possible_solutions(idx + 1)
            }
        } else {
            0
        }
    }

    fn count_possible_solutions(&self, mut start_point: usize) -> u64 {
        let mut decent = self.clone();

        while decent.condition[start_point] != SpringCondition::Unknown {
            start_point += 1;
        }

        decent.attempt_direction(start_point, SpringCondition::Damaged)
            + decent.attempt_direction(start_point, SpringCondition::Operational)
    }
}

fn first_task(input: &str) -> u64 {
    let puzzles: Result<Vec<Springs>, _> = input.lines().map(|x| x.parse()).collect();
    let puzzles = puzzles.unwrap();

    puzzles
        .par_iter()
        .map(|x| x.count_possible_solutions(0))
        .sum()
}

fn second_task(input: &str) -> u64 {
    let puzzles: Result<Vec<Springs>, _> = input.lines().map(|x| x.parse()).collect();
    let mut puzzles = puzzles.unwrap();

    puzzles.iter_mut().for_each(|x| x.expand());

    puzzles
        .par_iter()
        .map(|x| x.count_possible_solutions(0))
        .sum()
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 21);
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT), 525152);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
