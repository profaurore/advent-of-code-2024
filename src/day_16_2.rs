use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
    ops::Add,
};

const DEBUG: bool = false;

const COST_TURN: usize = 1000;
const COST_FORWARD: usize = 1;

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Vertex {
    position: Position,
    orientation: Orientation,
}

impl Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.orientation, self.position)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct EdgeEnd {
    end: Vertex,
    cost: usize,
}

impl Debug for EdgeEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}+{:?}", self.end, self.cost)
    }
}

#[derive(Default)]
struct Graph {
    vertices: HashSet<Vertex>,
    edges: HashMap<Vertex, Vec<EdgeEnd>>,
    start: Option<Vertex>,
    end: Option<[Vertex; 2]>,
}

impl Graph {
    fn add_edge(&mut self, v1: Vertex, v2: Vertex, cost: usize) {
        self.vertices.insert(v1);
        self.vertices.insert(v2);
        self.edges
            .entry(v1)
            .or_default()
            .push(EdgeEnd { end: v2, cost });
        self.edges
            .entry(v2)
            .or_default()
            .push(EdgeEnd { end: v1, cost });
    }

    fn set_start(&mut self, start: Vertex) {
        self.start = Some(start);
    }

    fn set_end(&mut self, end: Position) {
        self.end = Some([
            Vertex {
                position: end,
                orientation: Orientation::NS,
            },
            Vertex {
                position: end,
                orientation: Orientation::EW,
            },
        ]);
    }

    fn trim(&mut self) {
        let required = [
            self.start.unwrap(),
            self.end.unwrap()[0],
            self.end.unwrap()[1],
        ];

        if DEBUG {
            println!(
                "pre-trim {} {}",
                self.vertices.len(),
                self.edges
                    .iter()
                    .flat_map(|(_, edges)| edges.iter())
                    .count()
            );
        }

        loop {
            let to_remove: Vec<_> = self
                .edges
                .iter()
                .filter(|(v, edges)| !required.contains(v) && edges.len() <= 1)
                .map(|(&v, edges)| (v, edges.clone()))
                .collect();

            if to_remove.is_empty() {
                break;
            }

            to_remove.iter().for_each(|(v, edges)| {
                edges.iter().for_each(|edge| {
                    self.edges
                        .get_mut(&edge.end)
                        .unwrap()
                        .retain(|edge| edge.end != *v);
                });

                self.edges.remove(v);
                self.vertices.remove(v);
            });
        }

        if DEBUG {
            println!(
                "post-trim {} {}",
                self.vertices.len(),
                self.edges
                    .iter()
                    .flat_map(|(_, edges)| edges.iter())
                    .count()
            );
        }
    }

    fn dijkstra(
        &self,
    ) -> (HashMap<&Vertex, usize>, HashMap<&Vertex, Vec<Vertex>>) {
        let mut dist: HashMap<&Vertex, usize> =
            HashMap::from_iter(self.vertices.iter().map(|v| (v, usize::MAX)));
        let mut prev: HashMap<&Vertex, Vec<Vertex>> =
            HashMap::from_iter(self.vertices.iter().map(|v| (v, Vec::new())));

        if let Some(start) = self.start {
            let mut queue = self.vertices.clone();

            *dist.get_mut(&start).unwrap() = 0;

            while !queue.is_empty() {
                let min_vertex = *queue
                    .iter()
                    .filter_map(|v| {
                        dist.get(v)
                            .copied()
                            .filter(|&d| d != usize::MAX)
                            .map(|d| (v, d))
                    })
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .map(|(v, _)| v)
                    .unwrap();
                queue.remove(&min_vertex);

                let d = *dist.get(&min_vertex).unwrap();

                self.edges
                    .get(&min_vertex)
                    .unwrap()
                    .iter()
                    .filter(|EdgeEnd { end, .. }| queue.contains(end))
                    .for_each(|EdgeEnd { end, cost }| {
                        let new_d = d + cost;
                        let end_d = *dist.get(end).unwrap();
                        if new_d <= end_d {
                            if new_d < end_d {
                                dist.insert(end, new_d);
                                prev.get_mut(end).unwrap().clear();
                            }

                            prev.get_mut(end).unwrap().push(min_vertex);
                        }
                    })
            }
        }

        (dist, prev)
    }

    fn shortest_path_vertices(&self) -> HashSet<Position> {
        let mut vertices = HashSet::new();

        if let (Some(start), Some(end)) = (self.start, self.end) {
            let (dist, prev) = self.dijkstra();

            let min_end_dist =
                end.iter().filter_map(|end| dist.get(&end)).min();
            let mut remaining = Vec::from_iter(
                end.iter()
                    .filter(|end| dist.get(end) == min_end_dist)
                    .copied(),
            );

            if DEBUG {
                println!(
                    "{:?} {:?} {:?} {:?}",
                    min_end_dist,
                    end,
                    end.iter().map(|end| dist.get(&end)).collect::<Vec<_>>(),
                    remaining
                );
            }

            while let Some(vertex) = remaining.pop() {
                if vertices.insert(vertex) && vertex != start {
                    if let Some(prevs) = prev.get(&vertex) {
                        if DEBUG {
                            println!(
                                "{:?} {:?} {:?}",
                                dist.get(&vertex).unwrap(),
                                vertex,
                                prevs
                            );
                        }
                        remaining.extend(prevs);
                    }
                }
            }
        }

        vertices
            .iter()
            .map(|&Vertex { position, .. }| position)
            .collect()
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vertices_vec: Vec<_> = self.vertices.iter().collect();
        vertices_vec.sort();

        let mut edges_vec: Vec<_> = self.edges.iter().collect();
        edges_vec.sort_by(|a, b| a.0.cmp(b.0));

        writeln!(f, "Graph")?;
        writeln!(f, "  Vertices")?;
        for vertex in vertices_vec.iter() {
            writeln!(f, "    {:?}", vertex)?;
        }
        writeln!(f, "  Edges")?;
        for edge in edges_vec.iter() {
            writeln!(f, "    {:?}", edge)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Direction> for &Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        Position {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Direction {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Orientation {
    NS,
    EW,
}

/// https://adventofcode.com/2024/day/16#part2
pub fn day_16_2() {
    let data = fs::read_to_string("data/day_16.txt").expect("missing file");

    let map: Vec<Vec<_>> = data
        .split('\n')
        .map(|line| line.as_bytes().to_vec())
        .filter(|line| !line.is_empty())
        .collect();
    let max_x = map[0].len();
    let max_y = map.len();

    let mut graph = Graph::default();

    (0..max_y).for_each(|y| {
        (0..max_x).for_each(|x| {
            if map[y][x] != b'#' {
                let position = Position { x, y };
                let vertex_ns = Vertex {
                    position,
                    orientation: Orientation::NS,
                };
                let vertex_ew = Vertex {
                    position,
                    orientation: Orientation::EW,
                };

                // Moving between the vertical and horizontal paths.
                graph.add_edge(vertex_ns, vertex_ew, COST_TURN);

                // Moving between horizontally adjacent nodes.
                if map[y][x - 1] != b'#' {
                    let vertex_left_ew = Vertex {
                        position: Position { x: x - 1, y },
                        orientation: Orientation::EW,
                    };
                    graph.add_edge(vertex_ew, vertex_left_ew, COST_FORWARD);
                }

                // Moving between vertically adjacent nodes.
                if map[y - 1][x] != b'#' {
                    let vertex_up_ns = Vertex {
                        position: Position { x, y: y - 1 },
                        orientation: Orientation::NS,
                    };
                    graph.add_edge(vertex_ns, vertex_up_ns, COST_FORWARD);
                }

                match map[y][x] {
                    b'S' => graph.set_start(vertex_ew),
                    b'E' => graph.set_end(position),
                    _ => (),
                }
            }
        });
    });

    graph.trim();

    if DEBUG {
        println!("{:#?}", graph);
    }

    let best_lookouts = graph.shortest_path_vertices();

    println!("{}", best_lookouts.len());
}
