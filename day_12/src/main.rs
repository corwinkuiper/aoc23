use rayon::prelude::*;
use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("input.txt");

#[cfg(test)]
static TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        let damaged_springs: Result<Vec<u32>, _> = groups
            .split(',')
            .map(|x| x.parse().map_err(|_| "bad number"))
            .collect();

        let condition = condition?;

        Ok(Self {
            condition,
            damaged_springs: damaged_springs?,
        })
    }
}

fn apply<'a>(
    mut params: RecurseParams<'a>,
    spring_condition: SpringCondition,
    cache: &mut HashMap<RecurseParams<'a>, u64>,
) -> u64 {
    match spring_condition {
        SpringCondition::Operational => {
            if params.current_damaged_group_length != 0 {
                let Some(&expected_group_length) = params.remaining_groups.first() else {
                    return 0;
                };
                if expected_group_length != params.current_damaged_group_length {
                    return 0;
                }
                params.current_damaged_group_length = 0;
                params.remaining_groups = &params.remaining_groups[1..];
            }
        }
        SpringCondition::Damaged => params.current_damaged_group_length += 1,
        SpringCondition::Unknown => panic!("can't apply unknown"),
    }

    recurse_check(params, cache)
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct RecurseParams<'a> {
    remaining: &'a [SpringCondition],
    remaining_groups: &'a [u32],
    current_damaged_group_length: u32,
}

fn recurse_check_inner<'a>(
    mut params: RecurseParams<'a>,
    cache: &mut HashMap<RecurseParams<'a>, u64>,
) -> u64 {
    if let Some(next) = params.remaining.get(0) {
        params.remaining = &params.remaining[1..];

        match *next {
            normal @ (SpringCondition::Operational | SpringCondition::Damaged) => {
                apply(params, normal, cache)
            }
            SpringCondition::Unknown => {
                apply(params, SpringCondition::Damaged, cache)
                    + apply(params, SpringCondition::Operational, cache)
            }
        }
    } else if params.current_damaged_group_length != 0 && params.remaining_groups.len() == 1 {
        let &expected_length = params.remaining_groups.last().unwrap();
        if expected_length == params.current_damaged_group_length {
            1
        } else {
            0
        }
    } else if params.current_damaged_group_length == 0 && params.remaining_groups.is_empty() {
        1
    } else {
        0
    }
}

fn recurse_check<'a>(
    params: RecurseParams<'a>,
    cache: &mut HashMap<RecurseParams<'a>, u64>,
) -> u64 {
    if let Some(cached_value) = cache.get(&params).copied() {
        return cached_value;
    }

    let calculated_value = recurse_check_inner(params, cache);

    cache.insert(params, calculated_value);
    calculated_value
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

        let b: Vec<_> = self.damaged_springs.clone();
        self.damaged_springs.append(&mut b.clone());
        self.damaged_springs.append(&mut b.clone());
        self.damaged_springs.append(&mut b.clone());
        self.damaged_springs.append(&mut b.clone());
    }

    fn count(&self) -> u64 {
        recurse_check(
            RecurseParams {
                remaining: &self.condition,
                remaining_groups: &self.damaged_springs,
                current_damaged_group_length: 0,
            },
            &mut HashMap::new(),
        )
    }
}

fn first_task(input: &str) -> u64 {
    let puzzles: Result<Vec<Springs>, _> = input.lines().map(|x| x.parse()).collect();
    let puzzles = puzzles.unwrap();

    puzzles.par_iter().map(Springs::count).sum()
}

fn second_task(input: &str) -> u64 {
    let puzzles: Result<Vec<Springs>, _> = input.lines().map(|x| x.parse()).collect();
    let mut puzzles = puzzles.unwrap();

    puzzles.iter_mut().for_each(|x| x.expand());

    puzzles.par_iter().map(Springs::count).sum()
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
