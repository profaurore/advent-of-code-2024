use std::{
    collections::{HashMap, HashSet},
    fs,
};

const IS_TEST: bool = false;
const CHEAT_THRESHOLD: usize = if IS_TEST {
    // Expected output is 4.
    30
} else {
    100
};

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
}

/// https://adventofcode.com/2024/day/20#part1
pub fn day_20_1() {
    let data = fs::read_to_string(if IS_TEST {
        "data/day_20_test.txt"
    } else {
        "data/day_20.txt"
    })
    .expect("missing file");

    let map = data
        .split('\n')
        .take_while(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let xmax = map[0].len();
    let ymax = map.len();

    let mut graph = Graph::default();
    let mut start = Vertex { x: 0, y: 0 };
    let mut end = Vertex { x: 0, y: 0 };

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &cell)| {
            if cell != b'#' {
                let vertex = Vertex { x, y };
                if x > 0 && map[y][x - 1] != b'#' {
                    graph.add_edge(vertex, Vertex { x: x - 1, y });
                }
                if y > 0 && map[y - 1][x] != b'#' {
                    graph.add_edge(vertex, Vertex { x, y: y - 1 });
                }

                match cell {
                    b'S' => start = vertex,
                    b'E' => end = vertex,
                    _ => (),
                }
            }
        });
    });

    let (dist, _) = graph.dijkstra(start);

    let x_in_map = 1..(xmax - 1);
    let y_in_map = 1..(ymax - 1);

    let valid_cheats = map
        .iter()
        .enumerate()
        .filter(|(y, _)| y_in_map.contains(y))
        .fold(0, |valid_cheats, (y, line)| {
            line.iter()
                .enumerate()
                .filter(|(x, _)| x_in_map.contains(x))
                .fold(valid_cheats, |mut valid_cheats, (x, &cell)| {
                    if cell == b'#' {
                        if map[y][x - 1] != b'#' && map[y][x + 1] != b'#' {
                            let dist_left =
                                *dist.get(&Vertex { x: x - 1, y }).unwrap();
                            let dist_right =
                                *dist.get(&Vertex { x: x + 1, y }).unwrap();

                            if dist_left.abs_diff(dist_right) - 2
                                >= CHEAT_THRESHOLD
                            {
                                valid_cheats += 1;
                            }
                        }

                        if map[y - 1][x] != b'#' && map[y + 1][x] != b'#' {
                            let dist_up =
                                *dist.get(&Vertex { x, y: y - 1 }).unwrap();
                            let dist_down =
                                *dist.get(&Vertex { x, y: y + 1 }).unwrap();

                            if dist_up.abs_diff(dist_down) - 2
                                >= CHEAT_THRESHOLD
                            {
                                valid_cheats += 1;
                            }
                        }
                    }

                    valid_cheats
                })
        });

    println!("{}", valid_cheats);
}
