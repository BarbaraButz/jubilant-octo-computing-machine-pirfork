pub mod board;
pub mod players;

use self::board::{Board, Symbol};
use self::players::{Player, PlayerKind};

pub fn play(noughts: PlayerKind, crosses: PlayerKind) {
    let noughts = Player {
        symbol: Symbol::O,
        kind: noughts,
    };
    let crosses = Player {
        symbol: Symbol::X,
        kind: crosses,
    };

    let mut board = Board::new();
    println!("{}", board);

    while !board.full() {
        noughts.mark(&mut board);
        println!("\n{}", board);

        if board.full() {
            break;
        }

        crosses.mark(&mut board);
        println!("\n{}", board);
    }
}