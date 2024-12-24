use std::{
    collections::{HashMap, HashSet},
    fs,
};

const IS_TEST: bool = false;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vertex {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct Graph {
    vertices: HashSet<Vertex>,
    edges: HashMap<Vertex, HashSet<Vertex>>,
}

impl Graph {
    fn add_edge(&mut self, v1: Vertex, v2: Vertex) {
        self.vertices.insert(v1);
        self.vertices.insert(v2);
        self.edges.entry(v1).or_default().insert(v2);
        self.edges.entry(v2).or_default().insert(v1);
    }

    fn dijkstra(
        &self,
        start: Vertex,
    ) -> (HashMap<Vertex, usize>, HashMap<Vertex, Vec<Vertex>>) {
        let mut dist = self
            .vertices
            .iter()
            .map(|&v| (v, usize::MAX))
            .collect::<HashMap<_, _>>();
        let mut prev = self
            .vertices
            .iter()
            .map(|&v| (v, Vec::new()))
            .collect::<HashMap<_, _>>();
        let mut remaining = self.vertices.clone();

        *dist.get_mut(&start).unwrap() = 0;

        while !remaining.is_empty() {
            let vertex = *remaining
                .iter()
                .min_by(|&a, &b| dist.get(a).unwrap().cmp(dist.get(b).unwrap()))
                .unwrap();
            remaining.remove(&vertex);

            let vertex_dist = dist.get(&vertex).unwrap();

            if *vertex_dist == usize::MAX {
                return (dist, prev);
            }

            let new_dist = vertex_dist + 1;

            self.edges
                .get(&vertex)
                .unwrap()
                .iter()
                .filter(|v| remaining.contains(v))
                .for_each(|v| {
                    let current_dist = dist.get_mut(v).unwrap();

                    if new_dist <= *current_dist {
                        let vertex_prev = prev.get_mut(v).unwrap();

                        if new_dist < *current_dist {
                            vertex_prev.clear();
                            *current_dist = new_dist;
                        }

                        vertex_prev.push(vertex);
                    }
                });
        }

        (dist, prev)
    }

    fn min_dist(&self, start: Vertex, end: Vertex) -> Option<usize> {
        let (dist, _) = self.dijkstra(start);

        let min_dist = *dist.get(&end).unwrap();

        if min_dist == usize::MAX {
            None
        } else {
            Some(min_dist)
        }
    }
}

/// https://adventofcode.com/2024/day/18#part2
pub fn day_18_2() {
    let data = fs::read_to_string(if IS_TEST {
        "data/day_18_test.txt"
    } else {
        "data/day_18.txt"
    })
    .expect("missing file");

    let corrupted_memory_list = data
        .split('\n')
        .filter_map(|line| {
            line.split_once(',').and_then(|(x, y)| {
                match (x.parse::<usize>(), y.parse::<usize>()) {
                    (Ok(x), Ok(y)) => Some(Vertex { x, y }),
                    _ => None,
                }
            })
        })
        .collect::<Vec<_>>();

    let size = if IS_TEST { 7 } else { 71 };

    let mut min = 0;
    let mut max = corrupted_memory_list.len() - 1;

    while min <= max {
        let mid = min.midpoint(max);

        let corrupted_memory = corrupted_memory_list
            .iter()
            .take(mid)
            .copied()
            .collect::<Vec<_>>();
        let mut graph = Graph::default();

        (0..size).for_each(|x| {
            (0..size).for_each(|y| {
                let vertex = Vertex { x, y };

                if !corrupted_memory.contains(&vertex) {
                    if x > 0 {
                        let left = Vertex { x: x - 1, y };

                        if !corrupted_memory.contains(&left) {
                            graph.add_edge(vertex, left);
                        }
                    }

                    if y > 0 {
                        let up = Vertex { x, y: y - 1 };

                        if !corrupted_memory.contains(&up) {
                            graph.add_edge(vertex, up);
                        }
                    }
                }
            });
        });

        let min_dist = graph.min_dist(
            Vertex { x: 0, y: 0 },
            Vertex {
                x: size - 1,
                y: size - 1,
            },
        );

        if min_dist.is_some() {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }

    let bad_byte = corrupted_memory_list[max];

    println!("{},{}", bad_byte.x, bad_byte.y);
}
