use game::board::{Board, Symbol, Field};
use rand;
use rand::{Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerKind {
    Human,
    StupidAI,
    SmartAI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player {
    symbol: Symbol,
    kind: PlayerKind,
}

impl Player {
    pub fn mark(&self, board: &mut Board) {
        let (i, j) = match self.kind {
            PlayerKind::Human => Self::human_move(board),
            PlayerKind::StupidAI => Self::stupid_move(board),
            PlayerKind::SmartAI => Self::smart_move(board),
        };

        board.0[i][j] = Field::Occupied(self.symbol);
    }

    fn human_move(board: &mut Board) -> (usize, usize) {
        unimplemented!();
    }

    fn stupid_move(board: &mut Board) -> (usize, usize) {
        *(rand::thread_rng().choose(&board.empty_indices()).unwrap())
    }

    fn smart_move(board: &mut Board) -> (usize, usize) {
        unimplemented!();
    }
}
