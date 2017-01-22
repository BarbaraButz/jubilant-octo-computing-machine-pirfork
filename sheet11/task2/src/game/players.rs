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

        /// Reads a valid `usize` integer from the terminal/user.
        fn read_usize() -> usize {
            loop {
                match read_string().parse() {
                    Ok(res) => return res,
                    Err(_) => println!("That was not an integer! Please try again!"),
                }
            }
        }

        fn read_123() -> usize {
            loop {
                let n = read_usize();
                if n < 1 || n > 3 {
                    println!("1, 2 or 3...");
                } else {
                    return n;
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
