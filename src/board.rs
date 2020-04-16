use crate::tile::Tile;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    inner: HashMap<(usize, usize), Option<Tile>>,
}

impl Deref for Board {
    type Target = HashMap<(usize, usize), Option<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Board {
    pub fn new() -> Board {
        let mut board = HashMap::new();

        for x in 0..3 {
            for y in 0..3 {
                board.insert((x, y), None);
            }
        }

        Board {
            inner : board
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize, tile: Tile) -> Result<(), String> {
        if let Some(tile_entry) = self.get_mut(&(x,y)) {
            if let Some(owner) = tile_entry {
                Err(format!("Tile ({}, {}) is already claimed by {:?}", x, y, owner))
            } else {
                *tile_entry = Some(tile);
                Ok(())
            }
        } else {
            Err(format!("Coordinates not found for Tile : ({}, {})", x, y))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_board() {
        use super::*;

        let board = Board::new();

        if board.keys().len() != 9 {
            panic!("Board does not have 9 elements")
        }

        for (k, v) in board.iter() {
            let (x, y) = k;

            if *x > 2 || *y > 2 {
                panic!("Board is larger than 3x3")
            }

            if *v != None {
                panic!("Tile is not starting empty")
            }

        }
    }
}
