use game::board::{Board, Symbol, Field};
use rand;
use rand::{Rng};
use std;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerKind {
    Human,
    StupidAI,
    SmartAI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player {
    pub symbol: Symbol,
    pub kind: PlayerKind,
}

impl Player {
    pub fn mark(&self, board: &mut Board) {
        let (i, j) = match self.kind {
            PlayerKind::Human => {
                println!("{}, it's your turn! Please specify your move.", self.symbol.name());
                Self::human_move(board)
            }
            PlayerKind::StupidAI => Self::stupid_move(board),
            PlayerKind::SmartAI => Self::smart_move(board),
        };

        board.0[i][j] = Field::Occupied(self.symbol);
    }

    fn human_move(board: &mut Board) -> (usize, usize) {
        fn read_string() -> String {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went horribly wrong...");

            // Discard trailing newline
            let new_len = buffer.trim_right().len();
            buffer.truncate(new_len);

            buffer
        }

        fn read_123() -> usize {
            loop {
                match read_string().parse() {
                    Ok(x @ 1 ... 3) => return x,
                    _ => println!("1, 2 or 3..."),
                }
            }
        }

        loop {
            println!("Row: ");
            let i = read_123() - 1;
            println!("Column: ");
            let j = read_123() - 1;

            if board.0[i][j] != Field::Empty {
                println!("This field is already occupied.");
            } else {
                return (i, j)
            }
        }
    }

    fn stupid_move(board: &mut Board) -> (usize, usize) {
        *(rand::thread_rng().choose(&board.empty_indices()).unwrap())
    }

    fn smart_move(board: &mut Board) -> (usize, usize) {
        unimplemented!();
    }
}
