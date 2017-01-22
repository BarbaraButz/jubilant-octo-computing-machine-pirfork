pub mod board;
pub mod players;

use self::board::{Board, Symbol};
use self::players::{Player, PlayerKind};

pub fn play(player1: PlayerKind, player2: PlayerKind) {
    let noughts = Player {
        symbol: Symbol::O,
        kind: player1,
    };
    let crosses = Player {
        symbol: Symbol::X,
        kind: player2,
    };
    let mut board = Board::new();
    println!("{}", board);

    while !board.full() {
        noughts.mark(&mut board);
        println!("{}", board);
        if board.full() {
            break;
        }
        crosses.mark(&mut board);
        println!("{}", board);
    }
}
