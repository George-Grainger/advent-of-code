use self::Direction::*;
use advent_of_code::Day;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

const SRC: &str = include_str!("../../input/day17.txt");

pub type Coord = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Visit {
    cost: u32,
    position: Coord,
    direction: Direction,
    steps: u8,
}

impl Visit {
    fn new(cost: u32, position: Coord, direction: Direction, steps: u8) -> Self {
        Visit {
            cost,
            position,
            direction,
            steps,
        }
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct VisitKey {
    position: Coord,
    direction: Direction,
    steps: u8,
}

impl From<Visit> for VisitKey {
    fn from(value: Visit) -> Self {
        Self {
            position: value.position,
            direction: value.direction,
            steps: value.steps,
        }
    }
}

struct Graph<'a> {
    grid: &'a Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl<'a> Graph<'a> {
    fn new(grid: &'a Vec<Vec<u32>>) -> Self {
        let height = grid.len();
        let width = grid.get(0).map_or(0, |v| v.len());
        Self {
            grid,
            height,
            width,
        }
    }
}

impl Graph<'_> {
    fn dijkstra(&self, start: Coord, end: Coord, min_step: u8, max_step: u8) -> Option<u32> {
        // Create structures for tracking
        let mut dist = HashMap::<VisitKey, u32>::new();
        let mut heap = BinaryHeap::new();

        // Create the starting positions
        let v1 = Visit::new(0, start, East, 0);
        let v2 = Visit::new(0, start, South, 0);
        dist.insert(v1.into(), 0);
        dist.insert(v2.into(), 0);
        heap.push(v1);
        heap.push(v2);

        while let Some(state) = heap.pop() {
            // Extract the fields
            let Visit {
                cost,
                position,
                direction,
                steps,
            } = state;

            // Exit early since only need distance to target node
            if position == end && steps >= min_step {
                return Some(cost);
            }

            if dist.get(&state.into()).map_or(false, |&c| c < cost) {
                continue;
            }

            for (neighbour, neighbour_direction) in self.neighbors(position, direction) {
                let next = Visit {
                    cost: cost + self.grid[neighbour.0][neighbour.1],
                    position: neighbour,
                    direction: neighbour_direction,
                    steps: if neighbour_direction == direction {
                        steps + 1
                    } else {
                        1
                    },
                };
                if next.steps <= max_step && dist.get(&next.into()).map_or(true, |&c| next.cost < c)
                {
                    if next.direction == direction || steps >= min_step {
                        heap.push(next);
                        dist.insert(next.into(), next.cost);
                    }
                }
            }
        }

        None
    }

    fn neighbors(&self, (x, y): Coord, direction: Direction) -> Vec<(Coord, Direction)> {
        let mut neighbours: Vec<_> = Vec::with_capacity(4);
        if y > 0 && direction != South {
            neighbours.push(((x, y - 1), North))
        }
        if x > 0 && direction != East {
            neighbours.push(((x - 1, y), West))
        }
        if y < self.height - 1 && direction != North {
            neighbours.push(((x, y + 1), South))
        }
        if x < self.width - 1 && direction != West {
            neighbours.push(((x + 1, y), East))
        }
        neighbours
    }
}

pub struct Day17 {}

impl Day17 {
    fn parse() -> Vec<Vec<u32>> {
        SRC.lines()
            .map(|line| line.bytes().map(|b| (b - b'0') as u32).collect())
            .collect()
    }
}

impl Day for Day17 {
    fn problem1() {
        let grid = Self::parse();
        let graph = Graph::new(&grid);
        let heat_loss = graph.dijkstra((0, 0), (graph.width - 1, graph.height - 1), 1, 3);

        if let Some(heat_loss) = heat_loss {
            println!("Heat loss p1: {}", heat_loss);
        } else {
            println!("No path found");
        }
    }

    fn problem2() {
        let grid = Self::parse();
        let graph = Graph::new(&grid);
        let heat_loss = graph.dijkstra((0, 0), (graph.width - 1, graph.height - 1), 4, 10);

        if let Some(heat_loss) = heat_loss {
            println!("Heat loss p2: {}", heat_loss);
        } else {
            println!("No path found");
        }
    }
}
