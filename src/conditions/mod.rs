use crate::tile::Tile;
use crate::board::Board;

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq)]
pub enum Direction {
    Row(usize),
    Column(usize),
    MatchDiagonal,
    UnmatchDiagonal
}

#[derive(Debug, PartialEq)]
pub enum CheckType {
    Win,
    Block
}

#[derive(Debug, PartialEq)]
pub enum EmptyTile {
    Corner,
    Side
}

#[derive(Debug, PartialEq)]
pub enum Winner {
    AI,
    Player
}