use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/test.txt");

#[derive(Debug)]
struct Visit<T> {
    vertex: T,
    distance: usize,
}

impl<T> Ord for Visit<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<T> PartialOrd for Visit<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Visit<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<T> Eq for Visit<T> {}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Vertex {
    id: usize,
    x: usize,
    y: usize,
}

impl Vertex {
    fn new(id: usize, x: usize, y: usize) -> Self {
        Self { id, x, y }
    }
}

struct Graph {
    adjacency_list: Vec<HashMap<usize, usize>>,
}

impl Graph {
    fn from_grid(grid: &[Vec<usize>]) -> Self {
        let height = grid.len();
        let width = grid.get(0).expect("Should be at least one line").len();

        // Get a list of the vertices
        let vertices: Vec<Vertex> = (0..height)
            .flat_map(|y| (0..width).map(move |x| Vertex::new(width * y + x, x, y)))
            .collect();

        // Create an adjacency list form the vertices
        let mut adjacency_list = vec![HashMap::with_capacity(4); vertices.len()];
        for &Vertex { id, x, y } in vertices.iter() {
            if x > 0 {
                adjacency_list[id].insert(id - 1, grid[x - 1][y]);
            }
            if x < width - 1 {
                adjacency_list[id].insert(id + 1, grid[x + 1][y]);
            }
            if y > 0 {
                adjacency_list[id].insert(id - width, grid[x][y - 1]);
            }
            if y < height - 1 {
                adjacency_list[id].insert(id + width, grid[x][y + 1]);
            }
        }

        Self { adjacency_list }
    }

    fn dijkstra(self, start: usize) -> (Vec<usize>, Vec<usize>) {
        let mut distances = vec![usize::MAX; self.adjacency_list.len()];
        let mut predecessors = vec![usize::MAX; self.adjacency_list.len()];
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();

        predecessors[start] = start;
        distances[start] = 0;
        to_visit.push(Visit {
            vertex: start,
            distance: 0,
        });

        while let Some(Visit { vertex, distance }) = to_visit.pop() {
            if !visited.insert(vertex) {
                // Already visited this node
                continue;
            }

            if let Some(neighbors) = self.adjacency_list.get(vertex) {
                for (&neighbor, cost) in neighbors {
                    let new_distance = distance + cost;
                    let is_shorter = new_distance < distances[neighbor];

                    if is_shorter {
                        predecessors[neighbor] = vertex;
                        distances[neighbor] = new_distance;
                        to_visit.push(Visit {
                            vertex: neighbor,
                            distance: new_distance,
                        });
                    }
                }
            }
        }

        (distances, predecessors)
    }
}

pub struct Day17 {}

impl Day17 {
    fn print(grid: &[Vec<usize>], distances: &[usize], predecessors: &[usize]) {
        for row in distances.chunks(grid[0].len()) {
            for val in row {
                print!("{:2} ", val);
            }
            println!("");
        }
        println!("\n----------\n");
        for row in predecessors.chunks(grid[0].len()) {
            for val in row {
                print!("{:3} ", val);
            }
            println!("");
        }

        let height = grid.len();
        let width = grid.get(0).expect("Should be at least one line").len();
        let mut display = vec![vec!['.'; width]; height];
        let mut cur = height * width - 1;
        loop {
            display[cur / height][cur % height] = 'X';
            let prev = predecessors[cur];
            if prev == cur {
                break;
            } else {
                cur = prev;
            }
        }
        println!("\n----------\n");
        for row in display {
            for val in row {
                print!("{:2} ", val);
            }
            println!("");
        }
    }
}

impl Day for Day17 {
    fn problem1() {
        let grid: Vec<Vec<_>> = SRC
            .lines()
            .map(|line| line.bytes().map(|b| (b - b'0') as usize).collect())
            .collect();

        // Create a list of vertices from the grid
        let graph = Graph::from_grid(&grid);
        let (distances, predecessors) = graph.dijkstra(0);

        Self::print(&grid, &distances, &predecessors)
    }

    fn problem2() {
        todo!()
    }
}
