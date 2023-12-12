use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::str::FromStr;

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
    damaged_springs: Vec<DamagedSprings>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DamagedSprings {
    impossible_if_reached: usize,
    number_of_damaged_springs: u32,
}

impl DamagedSprings {
    fn make_list_of(consecutive_damaged_springs: &[u32], length: usize) -> Vec<DamagedSprings> {
        let mut c: Vec<_> = consecutive_damaged_springs
            .iter()
            .copied()
            .map(|x| DamagedSprings {
                number_of_damaged_springs: x,
                impossible_if_reached: usize::MAX,
            })
            .collect();
        let mut impossible_position = length;

        for ds in c.iter_mut().rev() {
            ds.impossible_if_reached = impossible_position;
            impossible_position -= ds.number_of_damaged_springs as usize;
            impossible_position -= 1;
        }

        c
    }
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
        let damaged_springs = DamagedSprings::make_list_of(&damaged_springs?, condition.len());

        Ok(Self {
            condition,
            damaged_springs,
        })
    }
}

#[derive(Debug, Clone)]
struct SpringChecker<'a> {
    damaged_spring_groups: core::iter::Peekable<core::slice::Iter<'a, DamagedSprings>>,
    current_damaged_group_length: u32,
    index: usize,
}

impl<'a> SpringChecker<'a> {
    fn new(spring_groups: &'a [DamagedSprings]) -> Self {
        Self {
            damaged_spring_groups: spring_groups.iter().peekable(),
            current_damaged_group_length: 0,
            index: 0,
        }
    }

    fn give(&mut self, c: SpringCondition) -> bool {
        assert_ne!(c, SpringCondition::Unknown);

        let &DamagedSprings {
            impossible_if_reached,
            number_of_damaged_springs,
        } = self
            .damaged_spring_groups
            .peek()
            .copied()
            .unwrap_or(&DamagedSprings {
                impossible_if_reached: usize::MAX,
                number_of_damaged_springs: 0,
            });

        if self.index > impossible_if_reached {
            return false;
        }

        self.index += 1;

        match c {
            SpringCondition::Operational => {
                if self.current_damaged_group_length != 0 {
                    if self.current_damaged_group_length != number_of_damaged_springs {
                        return false;
                    }
                    self.damaged_spring_groups.next();
                    self.current_damaged_group_length = 0;
                }
            }
            SpringCondition::Damaged => {
                self.current_damaged_group_length += 1;
                if self.current_damaged_group_length > number_of_damaged_springs {
                    return false;
                }
            }
            SpringCondition::Unknown => panic!("can't hand give an unknown spring"),
        }

        true
    }

    fn complete(&mut self) -> bool {
        self.give(SpringCondition::Operational) && self.damaged_spring_groups.next().is_none()
    }
}

#[derive(Clone, Debug)]
struct SpringEnumerator<'a> {
    checker: SpringChecker<'a>,
    springs: core::slice::Iter<'a, SpringCondition>,
}

impl<'a> SpringEnumerator<'a> {
    fn new(spring_groups: &'a [DamagedSprings], springs: &'a [SpringCondition]) -> Self {
        Self {
            checker: SpringChecker::new(spring_groups),
            springs: springs.iter(),
        }
    }

    fn count(&mut self) -> u64 {
        match self.springs.next() {
            Some(&spring @ (SpringCondition::Damaged | SpringCondition::Operational)) => {
                if self.checker.give(spring) {
                    self.count()
                } else {
                    0
                }
            }
            None => {
                if self.checker.complete() {
                    1
                } else {
                    0
                }
            }
            Some(SpringCondition::Unknown) => {
                let mut operational = self.clone();
                let damaged = self;

                let dc = if damaged.checker.give(SpringCondition::Damaged) {
                    damaged.count()
                } else {
                    0
                };
                let oc = if operational.checker.give(SpringCondition::Operational) {
                    operational.count()
                } else {
                    0
                };

                dc + oc
            }
        }
    }
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

        let b: Vec<_> = self
            .damaged_springs
            .iter()
            .map(|x| x.number_of_damaged_springs)
            .collect();
        let mut c = b.clone();
        c.append(&mut b.clone());
        c.append(&mut b.clone());
        c.append(&mut b.clone());
        c.append(&mut b.clone());
        self.damaged_springs = DamagedSprings::make_list_of(&c, self.condition.len());
    }

    fn count_possible_solutions(&self) -> u64 {
        let mut checker = SpringEnumerator::new(&self.damaged_springs, &self.condition);
        checker.count()
    }
}

fn first_task(input: &str) -> u64 {
    let puzzles: Result<Vec<Springs>, _> = input.lines().map(|x| x.parse()).collect();
    let puzzles = puzzles.unwrap();

    puzzles
        .par_iter()
        .map(|x| x.count_possible_solutions())
        .progress_count(puzzles.len() as u64)
        .sum()
}

fn second_task(input: &str) -> u64 {
    let puzzles: Result<Vec<Springs>, _> = input.lines().map(|x| x.parse()).collect();
    let mut puzzles = puzzles.unwrap();

    puzzles.iter_mut().for_each(|x| x.expand());

    puzzles
        .par_iter()
        .map(|x| x.count_possible_solutions())
        .progress_count(puzzles.len() as u64)
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
