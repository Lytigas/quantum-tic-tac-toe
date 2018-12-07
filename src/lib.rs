#![feature(test)]
extern crate test;

pub mod graph;

#[derive(Copy, Clone)]
pub struct ClassicalBoardState(u32);

impl ClassicalBoardState {
    pub fn new() -> Self {
        Self(0)
    }

    fn x_mask(sq: u8) -> u32 {
        1 << (2 * sq)
    }

    fn o_mask(sq: u8) -> u32 {
        1 << (2 * sq + 1)
    }

    pub fn set_x(&mut self, sq: u8) {
        self.0 |= Self::x_mask(sq);
        self.0 &= !Self::o_mask(sq);
    }

    pub fn set_o(&mut self, sq: u8) {
        self.0 |= Self::o_mask(sq);
        self.0 &= !Self::x_mask(sq);
    }

    pub fn is_x(&self, sq: u8) -> bool {
        self.0 & Self::x_mask(sq) > 0
    }

    pub fn is_o(&self, sq: u8) -> bool {
        self.0 & Self::o_mask(sq) > 0
    }

    pub fn is_empty(&self, sq: u8) -> bool {
        !(self.is_o(sq) || self.is_x(sq))
    }
}

use std::fmt;

impl fmt::Debug for ClassicalBoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let get = |sq| {
            if self.is_x(sq) {
                "x"
            } else if self.is_o(sq) {
                "o"
            } else {
                " "
            }
        };
        write!(
            f,
            "ClassicalBoardState {{
\t{a0}|{a1}|{a2}
\t-----
\t{a3}|{a4}|{a5}
\t-----
\t{a6}|{a7}|{a8}
}}",
            a0 = get(0),
            a1 = get(1),
            a2 = get(2),
            a3 = get(3),
            a4 = get(4),
            a5 = get(5),
            a6 = get(6),
            a7 = get(7),
            a8 = get(8)
        )
    }
}

#[derive(Clone)]
pub struct QuantumBoardState([u16; 9]);

impl QuantumBoardState {
    pub fn new() -> Self {
        Self([0; 9])
    }

    fn mask(mov: u8) -> u16 {
        1 << mov
    }

    pub fn is(&self, mov: u8, sq: u8) -> bool {
        self.0[sq as usize] & Self::mask(mov) > 0
    }

    pub fn add(&mut self, mov: u8, sq: u8, sq2: u8) {
        self.0[sq as usize] |= Self::mask(mov);
        self.0[sq2 as usize] |= Self::mask(mov);
    }

    pub fn clear(&mut self, sq: u8) {
        self.0[sq as usize] = 0
    }

    pub fn mask_in(&self, sq: u8) -> u16 {
        self.0[sq as usize]
    }

    pub fn is_sound(&self) -> bool {
        // for each move, there should be either two or zero instances
        for mov in 0..9 {
            let mut count = 0;
            for sq in 0..9 {
                count += if self.is(mov, sq) { 1 } else { 0 };
            }
            if !(count == 2 || count == 0) {
                return false;
            }
        }
        for sq in 0..9 {
            if self.0[sq as usize] > 0b111111111 {
                return false;
            }
        }
        true
    }
}

impl fmt::Debug for QuantumBoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "QuantumBoardState {{
\t0:{:016b} 1:{:016b} 2:{:016b},
\t3:{:016b} 4:{:016b} 5:{:016b},
\t6:{:016b} 7:{:016b} 8:{:016b},
}}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8]
        )
    }
}

#[cfg(test)]
mod hidden_board_test {
    use super::*;
    #[test]
    fn cboard_test() {
        let mut cb = ClassicalBoardState::new();
        let mut tester = |s| {
            cb.set_x(s);
            assert!(cb.is_x(s));
            cb.set_o(s);
            assert!(!cb.is_x(s));
            assert!(cb.is_o(s));
            cb.set_x(s);
            assert!(!cb.is_o(s));
            assert!(cb.is_x(s));
        };
        for i in 0..9 {
            tester(i)
        }
    }

    #[test]
    fn qboard_test() {
        let mut qb = QuantumBoardState::new();
        let mut tester = |mov, sq, sq2| {
            qb.add(mov, sq, sq2);
            assert!(qb.is(mov, sq));
            assert!(qb.is_sound());
        };
        for i in 0..9 {
            let sqs = [
                (0, 4),
                (1, 3),
                (4, 5),
                (8, 0),
                (2, 4),
                (1, 3),
                (5, 7),
                (8, 2),
                (1, 4),
            ];
            tester(i, sqs[i as usize].0, sqs[i as usize].1);
        }
        qb.clear(8);
        assert!(!qb.is_sound());
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Quantum(u8, u8),
    Collapse { sq: u8, mov: u8 },
}

use self::graph::BoardGraph;

#[derive(Clone, Debug)]
pub struct BoardState {
    c: ClassicalBoardState,
    q: QuantumBoardState,
    g: BoardGraph,
    next_mov: u8,
    cycle: smallvec::SmallVec<[u8; 9]>,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            c: ClassicalBoardState::new(),
            q: QuantumBoardState::new(),
            g: BoardGraph::new(),
            next_mov: 0,
            cycle: smallvec::SmallVec::new(),
        }
    }

    pub fn do_move(&mut self, m: Move) {
        debug_assert!(self.is_valid(m));
        match m {
            Move::Quantum(sq1, sq2) => {
                self.q.add(self.next_mov, sq1, sq2);
                self.g.add_edge(sq1, sq2);
                self.next_mov += 1;
                self.g.has_cycle(sq1, &mut self.cycle);
            }
            Move::Collapse { sq, mov } => {
                // find the index of sq in the cycle
                let idx = self
                    .cycle
                    .iter()
                    .enumerate()
                    .filter(|(_idx, &s)| s == sq)
                    .nth(0)
                    .unwrap()
                    .0;
                // last mask contains the moves that were in the last square but did not become classical
                let last_mask = QuantumBoardState::mask(mov);

                // destory the cycle, making the graph a tree so that we don't backtrack
                // however, we need to ensure the the direction we collapse contains contains the other
                // half the state we resolve to kick off the resolution in the dfs
                let candidate_square = self.cycle[wrap(idx as isize + 1, self.cycle.len())];
                if self.q.mask_in(candidate_square) & last_mask > 0 {
                    self.g.clear_edge(sq, candidate_square);
                } else {
                    self.g
                        .clear_edge(sq, self.cycle[wrap(idx as isize - 1, self.cycle.len())]);
                }
                resolve_depth_first(sq, last_mask, self);
                self.cycle.clear();
                fn resolve_depth_first(start: u8, last_mask: u16, board: &mut BoardState) {
                    // resolve this one
                    let decision_mask = board.q.mask_in(start) & last_mask;
                    match decision_mask {
                        // 0th - 8th bit
                        0b000000001 => board.c.set_x(start),
                        0b000000010 => board.c.set_o(start),
                        0b000000100 => board.c.set_x(start),
                        0b000001000 => board.c.set_o(start),
                        0b000010000 => board.c.set_x(start),
                        0b000100000 => board.c.set_o(start),
                        0b001000000 => board.c.set_x(start),
                        0b010000000 => board.c.set_o(start),
                        0b100000000 => board.c.set_x(start),
                        _ => unreachable!(),
                    };
                    let next_last_mask = board.q.mask_in(start) & (!decision_mask);
                    let edges = board.g.edges()[start as usize].clone();
                    board.g.clear_vert(start);
                    board.q.clear(start);
                    for sq in edges
                        .into_iter()
                        .enumerate()
                        .filter(|&(_idx, &amt)| amt > 0)
                        .map(|(idx, _)| idx as u8)
                    {
                        resolve_depth_first(sq, next_last_mask, board);
                    }
                }
            }
        }
    }

    pub fn valid_moves(&self, store: &mut Vec<Move>) {
        store.clear();
        if !self.cycle.is_empty() {
            // collapse the cycle at its start
            let square = self.q.mask_in(self.cycle[0]);
            // now find the possible collapses
            let next_square = self.q.mask_in(self.cycle[1]);
            let prev_square = self.q.mask_in(self.cycle[self.cycle.len() - 1]);
            let move_mask = (square & next_square) | (square & prev_square);
            // move_mask now contains two high bits -- the two moves that formed the cycle edges in the 0th vertex
            // return them
            for mov in 0..9 {
                if move_mask & (1 << mov) > 0 {
                    store.push(Move::Collapse {
                        sq: self.cycle[0],
                        mov: mov,
                    })
                }
            }
        } else {
            // generate all possible quantum moves, but without duplicates
            // assume all non-classical squares can fit our quantum move
            // this is one of the invariants other code should uphold
            // could optimize the loop
            for sq1 in 0..9 {
                if !self.c.is_empty(sq1) {
                    continue;
                }
                for sq2 in (sq1 + 1)..9 {
                    if !self.c.is_empty(sq2) {
                        continue;
                    }
                    store.push(Move::Quantum(sq1, sq2));
                }
            }
        }

        store.iter().for_each(|m| debug_assert!(self.is_valid(*m)));
    }

    pub fn is_valid(&self, m: Move) -> bool {
        match m {
            Move::Quantum(sq1, sq2) => {
                if !self.cycle.is_empty() {
                    return false;
                }
                // no classical moves
                if sq1 == sq2 {
                    return false;
                };
                // first assert both squares are classically empty
                if !self.c.is_empty(sq1) {
                    return false;
                };
                if !self.c.is_empty(sq2) {
                    return false;
                };
                // then assert the move hasn't been made anywhere before
                #[cfg(debug)]
                {
                    for i in 0..9 {
                        assert!(!self.q.is(self.next_mov), i);
                    }
                }

                true
            }
            Move::Collapse { sq, mov } => {
                // ensure there is a cycle there
                let mut store = smallvec::SmallVec::new();
                if !self.g.has_cycle(sq, &mut store) {
                    return false;
                }
                // there should also be a cycle from the last move
                if self.cycle.is_empty() {
                    return false;
                }
                // ensure this mov exists in the square we'd like to put it in
                if !self.q.is(mov, sq) {
                    return false;
                }

                // if this mov is part of the cycle, it should exist in either the next or previous square in the cycle
                let c_idx = self
                    .cycle
                    .iter()
                    .enumerate()
                    .filter(|(_idx, &s)| s == sq)
                    .nth(0)
                    .unwrap()
                    .0;
                let sq_check_1 = wrap(c_idx as isize - 1, self.cycle.len());
                let sq_check_2 = wrap(c_idx as isize + 1, self.cycle.len());

                if !(self.q.is(mov, self.cycle[sq_check_1])
                    || self.q.is(mov, self.cycle[sq_check_2]))
                {
                    return false;
                }
                true
            }
        }
    }

    pub fn is_state_valid(&self) -> bool {
        // if a square is classical, its graph and quanta must be empty
        for i in 0..9 {
            if !self.c.is_empty(i) {
                if self.q.mask_in(i) > 0 {
                    return false;
                }
                if self.g.edges()[i as usize] != [0; 9] {
                    return false;
                }
            }
        }

        // ensure corresponse between graph and quantum states
        for sq1 in 0..9 {
            for sq2 in self.g.edges()[sq1 as usize]
                .iter()
                .enumerate()
                .filter(|(_idx, &ct)| ct > 0)
                .map(|(idx, _)| idx)
            {
                if self.q.mask_in(sq1) & self.q.mask_in(sq2 as u8) == 0 {
                    return false;
                }
            }
        }

        true
    }
}

fn wrap(idx: isize, len: usize) -> usize {
    let len = len as isize;
    // rust uses the division operator instead of the modulus
    (((idx % len) + len) % len) as usize
}

#[cfg(test)]
mod boardstate_test {
    use super::*;

    #[test]
    fn three_cycle_tests() {
        let mut b = BoardState::new();
        let mut v = Vec::with_capacity(1000);
        // various invariants are asserted in debug mode
        b.do_move(Move::Quantum(1, 0));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(2, 1));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(2, 0));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Collapse { sq: 2, mov: 1 });
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        assert!(b.c.is_x(0));
        assert!(b.c.is_x(1));
        assert!(b.c.is_o(2));

        b.do_move(Move::Quantum(3, 4));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(4, 5));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(5, 3));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Collapse { sq: 3, mov: 3 });
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        assert!(b.c.is_o(3));
        assert!(b.c.is_x(4));
        assert!(b.c.is_o(5));

        b.do_move(Move::Quantum(6, 7));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(7, 8));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(8, 6));
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Collapse { sq: 7, mov: 6 });
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        assert!(b.c.is_x(6));
        assert!(b.c.is_x(7));
        assert!(b.c.is_o(8));
        println!("{:#?}", b);
    }

    #[test]
    fn large_collapse() {
        let mut b = BoardState::new();
        let mut v = Vec::with_capacity(1000);
        // various invariants are asserted in debug mode
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(0, 1)); // 1
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(7, 8)); // 2
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(2, 4)); // 3
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(4, 5)); // 4
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(5, 6)); // 5
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(1, 2)); // 6
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(4, 8)); // 7
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(3, 7)); // 8
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        b.do_move(Move::Quantum(0, 6)); // 9
        b.valid_moves(&mut v);
        assert!(b.is_state_valid());
        println!("{:#?}", b);
        // b.do_move(Move::Collapse { sq: 4, mov: 6 }); // should panic, 6 isn't part of the cycle
        b.do_move(Move::Collapse { sq: 4, mov: 2 });
        println!("{:#?}", v);

        assert!(b.is_state_valid());

        println!("{:#?}", b);

        assert!(b.c.is_x(1));
        assert!(b.c.is_x(4));
        assert!(b.c.is_x(6));
        assert!(b.c.is_x(8));
        assert!(b.c.is_x(0));
        assert!(b.c.is_o(7));
        assert!(b.c.is_o(5));
        assert!(b.c.is_o(2));
        assert!(b.c.is_o(3));
    }
}
