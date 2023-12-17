use std::ops::{Add, AddAssign};

use petgraph::{
    stable_graph::{DefaultIx, NodeIndex},
    Graph,
};

static INPUT: &str = include_str!("input.txt");
#[cfg(test)]
static TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

#[cfg(test)]
static TEST_2_INPUT: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vector2d(i32, i32);

impl From<(i32, i32)> for Vector2d {
    fn from(value: (i32, i32)) -> Self {
        Vector2d(value.0, value.1)
    }
}

impl Add for Vector2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2d(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vector2d {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

struct Node;

struct Edge {
    cost: u8,
}

struct HeatLossMap {
    graph: Graph<Node, Edge>,
    start: NodeIndex<DefaultIx>,
    end: NodeIndex<DefaultIx>,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as usize
    }
}

impl HeatLossMap {
    fn construct_first_task_from_input(input: &str) -> Self {
        let mut graph = Graph::new();

        let width = input.split_once('\n').unwrap().0.len() as i32;
        let height = input.lines().count() as i32;

        let nodes: Vec<_> = input
            .bytes()
            .filter(|&x| x != b'\n')
            .map(|v| {
                let digit = v - b'0';

                (
                    digit,
                    [
                        graph.add_node(Node),
                        graph.add_node(Node),
                        graph.add_node(Node),
                        graph.add_node(Node),
                        graph.add_node(Node),
                        graph.add_node(Node),
                        graph.add_node(Node),
                        graph.add_node(Node),
                    ],
                )
            })
            .collect();

        let get = |position: Vector2d| {
            let (x, y) = (position.0, position.1);
            if x < 0 || y < 0 || x >= width || y >= height {
                None
            } else {
                Some(nodes[(x + y * width) as usize])
            }
        };

        for y in 0..height {
            for x in 0..width - 1 {
                let (a, b) = (
                    get(Vector2d(x, y)).unwrap(),
                    get(Vector2d(x + 1, y)).unwrap(),
                );

                graph.add_edge(a.1[0], b.1[1], Edge { cost: b.0 });
                graph.add_edge(a.1[1], b.1[2], Edge { cost: b.0 });
                // graph.add_edge(a.1[2], b.1[3], Edge { cost: b.0 });

                graph.add_edge(b.1[0], a.1[1], Edge { cost: a.0 });
                graph.add_edge(b.1[1], a.1[2], Edge { cost: a.0 });
                // graph.add_edge(b.1[2], a.1[3], Edge { cost: a.0 });

                graph.add_edge(a.1[4], b.1[0], Edge { cost: b.0 });
                graph.add_edge(a.1[5], b.1[0], Edge { cost: b.0 });
                graph.add_edge(a.1[6], b.1[0], Edge { cost: b.0 });
                graph.add_edge(a.1[7], b.1[0], Edge { cost: b.0 });

                graph.add_edge(b.1[4], a.1[0], Edge { cost: a.0 });
                graph.add_edge(b.1[5], a.1[0], Edge { cost: a.0 });
                graph.add_edge(b.1[6], a.1[0], Edge { cost: a.0 });
                graph.add_edge(b.1[7], a.1[0], Edge { cost: a.0 });
            }
        }

        for y in 0..height - 1 {
            for x in 0..width {
                let (a, b) = (
                    get(Vector2d(x, y)).unwrap(),
                    get(Vector2d(x, y + 1)).unwrap(),
                );

                graph.add_edge(a.1[4], b.1[5], Edge { cost: b.0 });
                graph.add_edge(a.1[5], b.1[6], Edge { cost: b.0 });
                // graph.add_edge(a.1[6], b.1[7], Edge { cost: b.0 });

                graph.add_edge(b.1[4], a.1[5], Edge { cost: a.0 });
                graph.add_edge(b.1[5], a.1[6], Edge { cost: a.0 });
                // graph.add_edge(b.1[6], a.1[7], Edge { cost: a.0 });

                graph.add_edge(a.1[0], b.1[4], Edge { cost: b.0 });
                graph.add_edge(a.1[1], b.1[4], Edge { cost: b.0 });
                graph.add_edge(a.1[2], b.1[4], Edge { cost: b.0 });
                graph.add_edge(a.1[3], b.1[4], Edge { cost: b.0 });

                graph.add_edge(b.1[0], a.1[4], Edge { cost: a.0 });
                graph.add_edge(b.1[1], a.1[4], Edge { cost: a.0 });
                graph.add_edge(b.1[2], a.1[4], Edge { cost: a.0 });
                graph.add_edge(b.1[3], a.1[4], Edge { cost: a.0 });
            }
        }

        let start = graph.add_node(Node);
        let end = graph.add_node(Node);

        let a = get((1, 0).into()).unwrap();
        graph.add_edge(start, a.1[0], Edge { cost: a.0 });
        let a = get((0, 1).into()).unwrap();
        graph.add_edge(start, a.1[4], Edge { cost: a.0 });

        let end_nodes = get((width - 1, height - 1).into()).unwrap().1;

        for e in end_nodes.iter() {
            graph.add_edge(*e, end, Edge { cost: 0 });
        }

        Self { graph, start, end }
    }

    fn construct_second_task_from_input(input: &str) -> Self {
        let mut graph = Graph::new();

        let width = input.split_once('\n').unwrap().0.len() as i32;
        let height = input.lines().count() as i32;

        let nodes: Vec<_> = input
            .bytes()
            .filter(|&x| x != b'\n')
            .map(|v| {
                let digit = v - b'0';

                let nodes: [[_; 10]; 4] = (0..4)
                    .map(|_| {
                        (0..10)
                            .map(|_| graph.add_node(Node))
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                (digit, nodes)
            })
            .collect();

        let get = |position: Vector2d| {
            let (x, y) = (position.0, position.1);
            if x < 0 || y < 0 || x >= width || y >= height {
                None
            } else {
                Some(nodes[(x + y * width) as usize])
            }
        };

        for y in 0..height {
            for x in 0..width - 1 {
                let (a, b) = (
                    get(Vector2d(x, y)).unwrap(),
                    get(Vector2d(x + 1, y)).unwrap(),
                );

                for idx in 0..9 {
                    graph.add_edge(
                        a.1[Direction::East as usize][idx],
                        b.1[Direction::East as usize][idx + 1],
                        Edge { cost: b.0 },
                    );
                    graph.add_edge(
                        b.1[Direction::West as usize][idx],
                        a.1[Direction::West as usize][idx + 1],
                        Edge { cost: a.0 },
                    );
                }

                for idx in 3..10 {
                    for da in [Direction::North, Direction::South].into_iter() {
                        graph.add_edge(
                            a.1[da as usize][idx],
                            b.1[Direction::East as usize][0],
                            Edge { cost: b.0 },
                        );
                        graph.add_edge(
                            b.1[da as usize][idx],
                            a.1[Direction::West as usize][0],
                            Edge { cost: a.0 },
                        );
                    }
                }
            }
        }

        for y in 0..height - 1 {
            for x in 0..width {
                let (a, b) = (
                    get(Vector2d(x, y)).unwrap(),
                    get(Vector2d(x, y + 1)).unwrap(),
                );

                for idx in 0..9 {
                    graph.add_edge(
                        a.1[Direction::South as usize][idx],
                        b.1[Direction::South as usize][idx + 1],
                        Edge { cost: b.0 },
                    );
                    graph.add_edge(
                        b.1[Direction::North as usize][idx],
                        a.1[Direction::North as usize][idx + 1],
                        Edge { cost: a.0 },
                    );
                }

                for idx in 3..10 {
                    for da in [Direction::East, Direction::West].into_iter() {
                        graph.add_edge(
                            a.1[da as usize][idx],
                            b.1[Direction::South as usize][0],
                            Edge { cost: b.0 },
                        );
                        graph.add_edge(
                            b.1[da as usize][idx],
                            a.1[Direction::North as usize][0],
                            Edge { cost: a.0 },
                        );
                    }
                }
            }
        }

        let start = graph.add_node(Node);
        let end = graph.add_node(Node);

        let a = get((1, 0).into()).unwrap();
        graph.add_edge(start, a.1[Direction::East as usize][0], Edge { cost: a.0 });
        let a = get((0, 1).into()).unwrap();
        graph.add_edge(start, a.1[Direction::South as usize][0], Edge { cost: a.0 });

        let end_nodes = get((width - 1, height - 1).into()).unwrap().1;

        for node in end_nodes.iter().flat_map(|x| x.iter().skip(3)) {
            graph.add_edge(*node, end, Edge { cost: 0 });
        }

        Self { graph, start, end }
    }

    fn solve(&self) -> u64 {
        let nodes = petgraph::algo::dijkstra(&self.graph, self.start, Some(self.end), |e| {
            e.weight().cost as u64
        });

        *nodes.get(&self.end).unwrap()
    }
}

fn first_task(input: &str) -> u64 {
    HeatLossMap::construct_first_task_from_input(input).solve()
}

fn second_task(input: &str) -> u64 {
    HeatLossMap::construct_second_task_from_input(input).solve()
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 102);
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT), 94);
    assert_eq!(second_task(TEST_2_INPUT), 71);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
