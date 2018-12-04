#![feature(self_struct_ctor)]
#![feature(test)]
extern crate test;
use std::thread_local;

pub mod graph;

#[derive(Copy, Debug, Clone)]
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
}

pub struct QuantumBoardState([u16; 9]);

impl QuantumBoardState {
    pub fn new() -> Self {
        Self([0; 9])
    }

    fn mask(idx: u8) -> u16 {
        1 << idx
    }

    pub fn is(&self, idx: u8, sq: u8) -> bool {
        self.0[sq as usize] & Self::mask(idx) > 0
    }

    pub fn add(&mut self, idx: u8, sq: u8, sq2: u8) {
        self.0[sq as usize] |= Self::mask(idx);
        self.0[sq2 as usize] |= Self::mask(idx);
    }

    pub fn clear(&mut self, sq: u8) {
        self.0[sq as usize] = 0
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
        true
    }
}

#[cfg(test)]
mod cboard_test {
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
