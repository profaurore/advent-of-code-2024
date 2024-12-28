use std::{
    collections::{HashMap, HashSet},
    fs,
};

const IS_TEST: bool = false;
const CHEAT_THRESHOLD: usize = if IS_TEST { 50 } else { 100 };
const CHEAT_LENGTH: usize = if IS_TEST { 50 } else { 20 };

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

/// https://adventofcode.com/2024/day/20#part2
pub fn day_20_2() {
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

    let valid_cheats = (0..ymax)
        .map(|y| {
            (0..xmax)
                .filter_map(|x| dist.get(&Vertex { x, y }).map(|&d| (x, d)))
                .map(|(x, dist_v1)| {
                    ((y.saturating_sub(CHEAT_LENGTH))
                        ..=(y.saturating_add(CHEAT_LENGTH)).min(ymax - 1))
                        .map(|y2| {
                            let x_range = CHEAT_LENGTH - y2.abs_diff(y);

                            (x.saturating_sub(x_range)
                                ..=x.saturating_add(x_range).min(xmax - 1))
                                .filter_map(|x2| {
                                    dist.get(&Vertex { x: x2, y: y2 })
                                        .map(|&d| (x2, d))
                                })
                                .filter(|&(x2, dist_v2)| {
                                    let cheat_dist =
                                        x2.abs_diff(x) + y2.abs_diff(y);

                                    dist_v2
                                        >= dist_v1
                                            + cheat_dist
                                            + CHEAT_THRESHOLD
                                })
                                .count()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", valid_cheats);
}
