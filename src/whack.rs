use anyhow::{anyhow, Result};
use rand::Rng;

pub struct Game {
    pub board: Board,
    pub len: usize,
}

impl Game {
    pub fn new(len: usize) -> Self {
        Self {
            board: Board::new(len),
            len: len * len,
        }
    }

    pub fn whack(&mut self, idx: usize) -> Result<()> {
        if let Some(hole) = self.board.get_hole(idx) {
            if hole {
                return Ok(());
            } else {
                return Err(anyhow!("`idx`: {idx} is an invalid value"));
            }
        } else {
            Ok(())
        }
    }
}

struct Board {
    holes: Vec<bool>,
    prev_hole_idx: Option<usize>,
    len: usize,
}

impl Board {
    fn new(len: usize) -> Self {
        Self {
            holes: vec![false; len * len],
            prev_hole_idx: None,
            len: len * len,
        }
    }

    pub fn get_hole(&self, idx: usize) -> Option<bool> {
        if idx >= self.len {
            None
        } else {
            Some(self.holes[idx])
        }
    }

    pub fn get_hole_mut(&mut self, idx: usize) -> Option<&mut bool> {
        if idx >= self.len {
            None
        } else {
            Some(&mut self.holes[idx])
        }
    }

    fn toggle_hole(&mut self, idx: usize) {
        if let Some(hole) = self.get_hole_mut(idx) {
            *hole = !(*hole); // negates bool value
            self.prev_hole_idx = Some(idx);
        }
    }

    pub fn toggle_random_hole(&mut self) {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.len);
        self.toggle_hole(idx);
    }

    pub fn clean_prev_hole(&mut self) {
        if let Some(idx) = self.prev_hole_idx {
            self.toggle_hole(idx);
            self.prev_hole_idx = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_board_toggle_hole() {
        let mut game = Game::new(2);
        game.board.toggle_hole(2);
        assert!(game.board.holes[2] == true)
    }

    #[test]
    fn test_game_board_clean_prev_hole() {
        let mut game = Game::new(3);
        game.board.toggle_hole(8);
        assert!(game.board.holes[8] == true);

        game.board.clean_prev_hole();
        assert!(game.board.holes[8] == false)
    }

    #[test]
    fn test_game_board_toggle_random_hole() {
        let mut game = Game::new(4);
        game.board.toggle_random_hole();
        let mole_hole_idx = game.board.prev_hole_idx;
        assert!(game.board.holes[mole_hole_idx.unwrap()] == true)
    }

    #[test]
    fn test_game_board_toggle_random_hole_and_clean_prev_hole() {
        let mut game = Game::new(4);
        game.board.toggle_random_hole();
        let mole_hole_idx = game.board.prev_hole_idx;

        game.board.clean_prev_hole();
        assert!(game.board.holes[mole_hole_idx.unwrap()] == false)
    }
}
