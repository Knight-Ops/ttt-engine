use crate::board::Board;
use crate::strategy::Strategy;
use crate::tile::Tile;
use crate::conditions::{Direction, CheckType, EmptyTile, Winner};

use std::collections::HashMap;

static BOARD_SIZE : usize = 3;

struct GameState {
    board : Board,
    ai_token : Tile,
    player_token: Tile,
    last_move: Option<(usize, usize)>,
    filled_tiles : usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::new(),
            ai_token: Tile::X,
            player_token: Tile::O,
            last_move: None,
            filled_tiles: 0,
        }
    }

    pub fn set_player_token(&mut self, tile: Tile) {
        self.player_token = tile;

        match tile {
            Tile::X => self.ai_token = Tile::O,
            Tile::O => self.ai_token = Tile::X,
        }
    }

    pub fn find_ai_move(&self) -> Result<(usize, usize), String> {
        // Game over
        if self.filled_tiles == BOARD_SIZE * BOARD_SIZE {
            return Err(String::from("Game is over"))
        }

        let mut state : Strategy = Strategy::Win;
        loop {
            match state {
                Strategy::Win => {
                    println!("Checking strategy win...");
                    if let Some(valid_move) = self.check_win_block_condition(CheckType::Win) {
                        return Ok(valid_move);
                    } else {
                        state = Strategy::Block;
                    }
                },
                Strategy::Block => {
                    println!("Checking strategy block...");
                    if let Some(valid_move) = self.check_win_block_condition(CheckType::Block) {
                        return Ok(valid_move);
                    } else {
                        state = Strategy::Fork;
                    }
                },
                Strategy::Fork => {
                    println!("Checking strategy fork...");
                },
                Strategy::BlockFork => {
                    println!("Checking strategy block fork...");
                },
                Strategy::Center => {
                    println!("Checking strategy center...");
                    if let Some(valid_move) = self.check_center() {
                        return Ok(valid_move);
                    } else {
                        state = Strategy::OppositeCorner;
                    }
                },
                Strategy::OppositeCorner => {
                    println!("Checking strategy opposite corner...");
                    if let Some(valid_move) = self.check_corner() {
                        return Ok(valid_move);
                    } else {
                        state = Strategy::EmptyCorner;
                    }
                },
                Strategy::EmptyCorner => {
                    println!("Checking strategy empty corner...");
                    if let Some(valid_move) = self.find_empty(EmptyTile::Corner) {
                        return Ok(valid_move);
                    } else {
                        state = Strategy::EmptySide;
                    }
                },
                Strategy::EmptySide => {
                    println!("Checking strategy empty side...");
                    if let Some(valid_move) = self.find_empty(EmptyTile::Side) {
                        return Ok(valid_move);
                    } else {
                        return Err(String::from("No valid move found"));
                    }
                }
            }
        }
    }

    fn check_win_block_condition(&self, check: CheckType) -> Option<(usize, usize)> {
        let mut board_state : HashMap<Direction, (usize, usize, usize)> = HashMap::new();

        let check_token = {
            match check {
                CheckType::Win => self.ai_token,
                CheckType::Block => self.player_token,
            }
        };

        self.board.iter().for_each(|tile_entry| {
            if let Some(fill) = tile_entry.1 {
                let addin = {
                    if *fill == check_token {
                        1
                    } else {
                        // This is just so that opponents tokens are recognized
                        3
                    }
                };

                let (x, y) = tile_entry.0;
                let row = Direction::Row(*y);
                let col = Direction::Column(*x);

                board_state.entry(row).and_modify(|entry| {
                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin)
                }).or_insert((*x, *y, addin));

                board_state.entry(col).and_modify(|entry| {
                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                }).or_insert((*x, *y, addin));

                // Diagonal Check
                if (x + y) % 2 == 0 {
                    // In the center case, we make changes to both the matching diagonal
                    // and the unmatching diagonal
                    // let match_diag = ;
                    // let unmatch_diag = ;
                    if *x == *y && *x == 1 {
                        board_state.entry(Direction::MatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));

                        board_state.entry(Direction::UnmatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));
                    } 
                    // In the matching diagonal case, we just add to the matching diagonal
                    else if *x == *y && *x != addin {
                        board_state.entry(Direction::MatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));
                    } 
                    // In the unmatching diagonal case, we just add to the unmatching diagonal
                    else {
                        board_state.entry(Direction::UnmatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));
                    }
                } 
            }            
        });

        let wins : Vec<_> = board_state.iter().filter(|&direction| {
            (*direction.1).2 == BOARD_SIZE - 1
        }).collect();

        Some(((BOARD_SIZE - (*wins[0].1).0) % BOARD_SIZE, (BOARD_SIZE - (*wins[0].1).1) % BOARD_SIZE))
    }

    fn check_fork_block_condition(&self, check: CheckType) -> Option<(usize, usize)> {

        let turn = self.filled_tiles / 2;
        // We can't possibly have a fork before turn 2
        if turn < 2 {
            return None
        }

        let mut fork_move = None;
        
        self.board.iter().for_each(|tile_outer| {
            let mut fork_board = self.board.clone();
            if *tile_outer.1 == None && fork_move == None {
                if check == CheckType::Win {
                    println!("Move {} : {}", (*tile_outer.0).0, (*tile_outer.0).0);
                    fork_board.make_move((*tile_outer.0).0, (*tile_outer.0).1, self.ai_token);
                } else {
                    fork_board.make_move((*tile_outer.0).0, (*tile_outer.0).1, self.player_token);
                }

                let mut board_state : HashMap<Direction, (usize, usize, usize)> = HashMap::new();

                let check_token = {
                    match check {
                        CheckType::Win => self.ai_token,
                        CheckType::Block => self.player_token,
                    }
                };

                fork_board.iter().for_each(|tile_entry| {
                    if let Some(fill) = tile_entry.1 {
                        let addin = {
                            if *fill == check_token {
                                1
                            } else {
                                // This is just so that opponents tokens are recognized
                                3
                            }
                        };

                        let (x, y) = tile_entry.0;
                        let row = Direction::Row(*y);
                        let col = Direction::Column(*x);

                        board_state.entry(row).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin)
                        }).or_insert((*x, *y, addin));

                        board_state.entry(col).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));

                        // Diagonal Check
                        if (x + y) % 2 == 0 {
                            // In the center case, we make changes to both the matching diagonal
                            // and the unmatching diagonal
                            // let match_diag = ;
                            // let unmatch_diag = ;
                            if *x == *y && *x == 1 {
                                board_state.entry(Direction::MatchDiagonal).and_modify(|entry| {
                                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                                }).or_insert((*x, *y, addin));

                                board_state.entry(Direction::UnmatchDiagonal).and_modify(|entry| {
                                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                                }).or_insert((*x, *y, addin));
                            } 
                            // In the matching diagonal case, we just add to the matching diagonal
                            else if *x == *y && *x != 1 {
                                board_state.entry(Direction::MatchDiagonal).and_modify(|entry| {
                                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                                }).or_insert((*x, *y, addin));
                            } 
                            // In the unmatching diagonal case, we just add to the unmatching diagonal
                            else {
                                board_state.entry(Direction::UnmatchDiagonal).and_modify(|entry| {
                                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                                }).or_insert((*x, *y, addin));
                            }
                        }
                    }            
                });

                let wins : Vec<_> = board_state.iter().filter(|&direction| {
                    (*direction.1).2 == BOARD_SIZE - 1
                }).collect();

                if wins.len() >= 2 {
                    fork_move = Some(((*tile_outer.0).0, (*tile_outer.0).1))
                }
            }
        });

        fork_move
    }

    fn check_center(&self) -> Option<(usize, usize)> {
        let center = (BOARD_SIZE - 1) / 2;
        if let Some(_) = self.board.get(&(center, center))? {
            println!("{:?}", self.board.get(&(center, center)));
            None
        } else {
            Some((center, center))
        }
    }

    fn check_corner(&self) -> Option<(usize, usize)> {
        let corner = BOARD_SIZE - 1;

        if let Some(last_move) = self.last_move {
            if last_move.0 == last_move.1 && last_move.0 == 0 {
                if let Some(_) = self.board.get(&(corner, corner))? {
                    None
                } else {
                    Some((corner, corner))
                }
            } 
            else if last_move.0 == last_move.1 && last_move.0 != 0 {
                if let Some(_) = self.board.get(&(0, 0))? {
                    None
                } else {
                    Some((corner, corner))
                }
            }
            else {
                if let Some(_) = self.board.get(&(last_move.1, last_move.0))? {
                    None
                } else {
                    Some((last_move.1, last_move.0))
                }
            }
        } else {
            None
        }
    }

    fn find_empty(&self, empty: EmptyTile) -> Option<(usize, usize)> {

        let tile_filter = {
            match empty {
                EmptyTile::Corner => 0,
                EmptyTile::Side => 1,
            }
        };

        let mut found_move = None;

        self.board.iter().for_each(|tile_entry| {
            if *tile_entry.1 == None {
                if ((*tile_entry.0).0 + (*tile_entry.0).1) % 2 == tile_filter {
                    found_move = Some(((*tile_entry.0).0, (*tile_entry.0).1));
                }
            }
        });

        found_move
    }

    fn is_game_won(&self) -> Option<Winner> {
        let turn = self.filled_tiles / 2;
        // We can't possibly have a win before turn 3
        if turn < 3 {
            return None
        }

        let mut board_state : HashMap<Direction, (usize, usize, usize)> = HashMap::new();

        self.board.iter().for_each(|tile_entry| {
            if let Some(fill) = tile_entry.1 {
                let addin = {
                    if *fill == self.ai_token {
                        1
                    } else {
                        // This is just so that opponents tokens are recognized
                        4
                    }
                };

                let (x, y) = tile_entry.0;
                let row = Direction::Row(*y);
                let col = Direction::Column(*x);

                board_state.entry(row).and_modify(|entry| {
                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin)
                }).or_insert((*x, *y, addin));

                board_state.entry(col).and_modify(|entry| {
                    *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                }).or_insert((*x, *y, addin));

                // Diagonal Check
                if (x + y) % 2 == 0 {
                    // In the center case, we make changes to both the matching diagonal
                    // and the unmatching diagonal
                    // let match_diag = ;
                    // let unmatch_diag = ;
                    if *x == *y && *x == 1 {
                        board_state.entry(Direction::MatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));

                        board_state.entry(Direction::UnmatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));
                    } 
                    // In the matching diagonal case, we just add to the matching diagonal
                    else if *x == *y && *x != addin {
                        board_state.entry(Direction::MatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));
                    } 
                    // In the unmatching diagonal case, we just add to the unmatching diagonal
                    else {
                        board_state.entry(Direction::UnmatchDiagonal).and_modify(|entry| {
                            *entry = (entry.0 + x, entry.1 + y, entry.2 + addin);
                        }).or_insert((*x, *y, addin));
                    }
                } 
            }            
        });

        let ai_win : Vec<_> = board_state.iter().filter(|&direction| {
            (*direction.1).2 == BOARD_SIZE * 1
        }).collect();

        let player_win : Vec<_> = board_state.iter().filter(|&direction| {
            (*direction.1).2 == BOARD_SIZE * 4
        }).collect();

        if ai_win.len() > 0 && player_win.len() > 0 {
            panic!("How did that happen?!");
        } else if ai_win.len() > 0 {
            Some(Winner::AI)
        } else if player_win.len() > 0 {
            Some(Winner::Player)
        } else {
            None
        }
    }

    fn make_ai_move(&mut self, x: usize, y: usize) -> Result<(), String> {
        self.last_move = Some((x, y));
        self.filled_tiles += 1;
        self.board.make_move(x, y, self.ai_token)
    }

    fn make_player_move(&mut self, x: usize, y: usize) -> Result<(), String> {
        self.last_move = Some((x, y));
        self.filled_tiles += 1;
        self.board.make_move(x, y, self.player_token)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_check_win_conditions_col() {
        use super::*;

        let mut game = GameState::new();

        game.make_ai_move(0, 0);
        game.make_player_move(0, 2);
        game.make_ai_move(0, 1);
        game.make_player_move(1, 1);
        game.make_ai_move(2,0);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Win) {
            let (x, y) = tuple;
            assert_eq!(x, 1);
            assert_eq!(y, 0);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_win_conditions_row() {
        use super::*;

        let mut game = GameState::new();

        game.make_ai_move(0, 0);
        game.make_ai_move(1, 0);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Win) {
            let (x, y) = tuple;
            assert_eq!(x, 2);
            assert_eq!(y, 0);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_win_conditions_diag() {
        use super::*;

        let mut game = GameState::new();

        game.make_ai_move(0, 0);
        game.make_ai_move(1, 1);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Win) {
            let (x, y) = tuple;
            assert_eq!(x, 2);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_win_conditions_rev_diag() {
        use super::*;

        let mut game = GameState::new();

        game.make_ai_move(2, 0);
        game.make_ai_move(1, 1);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Win) {
            let (x, y) = tuple;
            assert_eq!(x, 0);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_block_conditions_col() {
        use super::*;

        let mut game = GameState::new();

        game.make_player_move(0, 0);
        game.make_player_move(0, 1);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Block) {
            let (x, y) = tuple;
            assert_eq!(x, 0);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_block_conditions_row() {
        use super::*;

        let mut game = GameState::new();

        game.make_player_move(0, 0);
        game.make_player_move(1, 0);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Block) {
            let (x, y) = tuple;
            assert_eq!(x, 2);
            assert_eq!(y, 0);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_block_conditions_diag() {
        use super::*;

        let mut game = GameState::new();

        game.make_player_move(0, 0);
        game.make_player_move(1, 1);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Block) {
            let (x, y) = tuple;
            assert_eq!(x, 2);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_block_conditions_rev_diag() {
        use super::*;

        let mut game = GameState::new();

        game.make_player_move(2, 0);
        game.make_player_move(1, 1);

        if let Some(tuple) = game.check_win_block_condition(CheckType::Block) {
            let (x, y) = tuple;
            assert_eq!(x, 0);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_center() {
        use super::*;

        let mut game = GameState::new();

        if let Some(tuple) = game.check_center() {
            let (x, y) = tuple;
            assert_eq!(x, 1);
            assert_eq!(y, 1);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_opposite_corner() {
        use super::*;

        let mut game = GameState::new();

        game.make_player_move(0, 0);

        if let Some(tuple) = game.check_corner() {
            let (x, y) = tuple;
            assert_eq!(x, 2);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_check_opposite_corner_rev() {
        use super::*;

        let mut game = GameState::new();

        game.make_player_move(2, 0);

        if let Some(tuple) = game.check_corner() {
            let (x, y) = tuple;
            assert_eq!(x, 0);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_fork() {
        use super::*;

        let mut game = GameState::new();

        game.make_ai_move(0, 0);
        game.make_player_move(1, 0);
        game.make_ai_move(2, 0);
        game.make_player_move(0, 2);

        if let Some(tuple) = game.check_fork_block_condition(CheckType::Win) {
            let (x, y) = tuple;
            println!("{} {}", x, y);
            assert_eq!(x, 2);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_fork_block() {
        use super::*;

        let mut game = GameState::new();

        game.make_player_move(0, 0);
        game.make_ai_move(1, 0);
        game.make_player_move(2, 0);
        game.make_ai_move(0, 2);

        if let Some(tuple) = game.check_fork_block_condition(CheckType::Block) {
            let (x, y) = tuple;
            println!("{} {}", x, y);
            assert_eq!(x, 2);
            assert_eq!(y, 2);
        } else {
            panic!()
        }
    }
}