/// Graph with implicit nodes 0-8 for a tic tac toe board
#[derive(Clone)]
pub struct BoardGraph {
    edges: [[u8; 9]; 9],
}

impl BoardGraph {
    pub fn new() -> Self {
        Self { edges: [[0; 9]; 9] }
    }

    pub fn add_edge(&mut self, u: u8, v: u8) {
        debug_assert!(u != v);
        self.edges[u as usize][v as usize] += 1;
        self.edges[v as usize][u as usize] += 1;
    }

    pub fn clear_edge(&mut self, u: u8, v: u8) {
        debug_assert!(u != v);
        self.edges[u as usize][v as usize] = 0;
        self.edges[v as usize][u as usize] = 0;
    }

    pub fn has_cycle(&self, start: u8, store: &mut smallvec::SmallVec<[u8; 9]>) -> bool {
        store.clear();
        // case 1: cycle of length 2
        // find the max number of edges to one node
        let max_edge = self.edges[start as usize]
            .iter()
            .enumerate()
            .max_by_key(|(_idx, &x)| x)
            .unwrap();
        if *max_edge.1 >= 2 {
            store.push(start);
            store.push(max_edge.0 as u8);
            return true;
        }

        // case 2: cycle of length >2 with bfs
        fn bfs(
            start: u8,
            parent: Option<u8>,
            visited: &mut [bool; 9],
            graph: &[[u8; 9]; 9],
            store: &mut smallvec::SmallVec<[u8; 9]>,
        ) -> bool {
            if visited[start as usize] {
                store.push(start);
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
                if bfs(v as u8, Some(start), visited, graph, store) {
                    if start != store[0] {
                        store.push(start);
                        return true;
                    }
                    return false;
                }
            }
            false
        }

        let mut visited = [false; 9];
        bfs(start, None, &mut visited, &self.edges, store);
        !store.is_empty()
    }

    pub fn clear_vert(&mut self, v: u8) {
        self.edges[v as usize] = [0; 9];
        for arr in self.edges.iter_mut() {
            arr[v as usize] = 0;
        }
    }

    pub fn edges(&self) -> &[[u8; 9]; 9] {
        &self.edges
    }
}

use std::fmt;
impl fmt::Debug for BoardGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoardGraph {{
\t   0  1  2  3  4  5  6  7  8
\t0 {:?}
\t1 {:?}
\t2 {:?}
\t3 {:?}
\t4 {:?}
\t5 {:?}
\t6 {:?}
\t7 {:?}
\t8 {:?}
}}
", self.edges[0], self.edges[1], self.edges[2],self.edges[3],self.edges[4],self.edges[5],self.edges[6],self.edges[7],self.edges[8])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;
    use smallvec::SmallVec;
    #[test]
    fn two_cycle() {
        let mut b = BoardGraph::new();
        let mut store = smallvec::SmallVec::new();

        assert!(!b.has_cycle(0, &mut store));
        b.add_edge(0, 1);
        b.add_edge(1, 0);
        assert!(b.has_cycle(0, &mut store));
        assert!(b.has_cycle(1, &mut store));
        assert_eq!(store, SmallVec::from_buf([1u8, 0u8]));

        b = BoardGraph::new();
        b.add_edge(2, 4);
        assert!(!b.has_cycle(2, &mut store));
        assert!(!b.has_cycle(7, &mut store));
        b.add_edge(2, 4);
        assert!(b.has_cycle(2, &mut store));
        assert!(b.has_cycle(4, &mut store));

        b = BoardGraph::new();
        b.add_edge(1, 8);
        assert!(!b.has_cycle(0, &mut store));
        assert!(!b.has_cycle(1, &mut store));
        b.add_edge(1, 8);
        assert!(b.has_cycle(8, &mut store));
        assert!(b.has_cycle(1, &mut store));
        assert_eq!(store, SmallVec::from_buf([1u8, 8u8]));
    }

    #[test]
    fn complex_cycle() {
        let mut b = BoardGraph::new();
        let mut store = smallvec::SmallVec::new();
        assert!(!b.has_cycle(0, &mut store));
        b.add_edge(0, 1);
        b.add_edge(1, 4);
        b.add_edge(4, 3);
        b.add_edge(3, 6);
        b.add_edge(6, 7);
        assert!(!b.has_cycle(0, &mut store));
        assert!(!b.has_cycle(7, &mut store));
        assert!(!b.has_cycle(4, &mut store));
        b.add_edge(1, 2);
        b.add_edge(1, 5);
        assert!(!b.has_cycle(5, &mut store));
        b.add_edge(4, 8);
        assert!(!b.has_cycle(0, &mut store));
        b.add_edge(7, 1);
        assert!(b.has_cycle(0, &mut store));
        assert_eq!(store, SmallVec::from_buf([1, 7, 6, 3, 4]));
        b.clear_vert(4);
        assert!(!b.has_cycle(0, &mut store));
    }

    #[bench]
    fn bench_cyclic(bench: &mut Bencher) {
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
        let mut store = smallvec::SmallVec::new();
        assert!(b.has_cycle(0, &mut store));
        bench.iter(|| b.has_cycle(0, &mut store));
    }

    #[bench]
    fn bench_acyclic(bench: &mut Bencher) {
        let mut b = BoardGraph::new();
        b.add_edge(0, 1);
        b.add_edge(1, 4);
        b.add_edge(4, 3);
        b.add_edge(3, 6);
        b.add_edge(6, 7);
        b.add_edge(1, 2);
        b.add_edge(1, 5);
        b.add_edge(4, 8);
        let mut store = smallvec::SmallVec::new();
        assert!(!b.has_cycle(0, &mut store));
        bench.iter(|| b.has_cycle(0, &mut store));
    }
}
