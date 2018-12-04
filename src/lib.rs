#![feature(test)]
extern crate test;
use std::thread_local;

thread_local! {
    pub static SRCH_Q: Vec<u8> = Vec::with_capacity(20);
}

/// Graph with implicit nodes 0-8 for a tic tac toe board
#[derive(Debug, Clone)]
pub struct BoardGraph {
    edges: [[u8; 9]; 9],
}

impl BoardGraph {
    pub fn new() -> Self {
        Self { edges: [[0; 9]; 9] }
    }

    pub fn add_edge(&mut self, u: u8, v: u8) {
        self.edges[u as usize][v as usize] += 1;
        self.edges[v as usize][u as usize] += 1;
    }

    pub fn has_cycle(&self, start: u8) -> bool {
        // case 1: cycle of length 2
        // find the max number of edges to one node
        if self.edges[start as usize]
            .iter()
            .max_by_key(|&x| x)
            .unwrap()
            >= &2
        {
            return true;
        }

        // case 2: cycle of length >2 with bfs
        fn bfs(
            start: u8,
            parent: Option<u8>,
            visited: &mut [bool; 9],
            graph: &[[u8; 9]; 9],
        ) -> bool {
            if visited[start as usize] {
                return true;
            };
            visited[start as usize] = true;
            let parent_idx = match parent {
                Some(p) => p as usize,
                None => std::usize::MAX,
            };
            for v in graph[start as usize]
                .iter()
                .enumerate()
                .filter_map(|(idx, &val)| {
                    if val > 0 && idx != parent_idx {
                        Some(idx)
                    } else {
                        None
                    }
                }) {
                if bfs(v as u8, Some(start), visited, graph) {
                    return true;
                }
            }
            false
        }

        let mut visited = [false; 9];
        bfs(start, None, &mut visited, &self.edges)
    }

    pub fn clear_vert(&mut self, v: u8) {
        self.edges[v as usize] = [0; 9];
        for arr in self.edges.iter_mut() {
            arr[v as usize] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use super::*;
    #[test]
    fn two_cycle() {
        let mut b = BoardGraph::new();
        assert!(!b.has_cycle(0));
        b.add_edge(0, 1);
        b.add_edge(1, 0);
        assert!(b.has_cycle(0));
        assert!(b.has_cycle(1));

        b = BoardGraph::new();
        b.add_edge(2, 4);
        assert!(!b.has_cycle(2));
        assert!(!b.has_cycle(7));
        b.add_edge(2, 4);
        assert!(b.has_cycle(2));
        assert!(b.has_cycle(4));

        b = BoardGraph::new();
        b.add_edge(1, 8);
        assert!(!b.has_cycle(0));
        assert!(!b.has_cycle(1));
        b.add_edge(1, 8);
        assert!(b.has_cycle(8));
        assert!(b.has_cycle(1));
    }

    #[test]
    fn complex_cycle() {
        let mut b = BoardGraph::new();
        assert!(!b.has_cycle(0));
        b.add_edge(0, 1);
        b.add_edge(1, 4);
        b.add_edge(4, 3);
        b.add_edge(3, 6);
        b.add_edge(6, 7);
        assert!(!b.has_cycle(0));
        assert!(!b.has_cycle(7));
        assert!(!b.has_cycle(4));
        b.add_edge(1, 2);
        b.add_edge(1, 5);
        assert!(!b.has_cycle(5));
        b.add_edge(4, 8);
        assert!(!b.has_cycle(0));
        b.add_edge(7, 1);
        assert!(b.has_cycle(0));
        b.clear_vert(4);
        assert!(!b.has_cycle(0));
    }

    #[bench]
    fn bench(bench: &mut Bencher) {
        let mut b = BoardGraph::new();
        b.add_edge(0, 1);
        b.add_edge(1, 4);
        b.add_edge(4, 3);
        b.add_edge(3, 6);
        b.add_edge(6, 7);
        b.add_edge(1, 2);
        b.add_edge(1, 5);
        b.add_edge(4, 8);
        b.add_edge(7, 1);
        bench.iter(|| b.has_cycle(0));
    }
}
