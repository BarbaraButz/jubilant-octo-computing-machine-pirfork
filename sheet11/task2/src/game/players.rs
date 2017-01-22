use game::board::{Board, Symbol, Field};
use rand;

trait Player {
    fn mark(&self, board: &mut Board);
}

struct Human(Symbol);
struct StupidAI(Symbol);
struct SmartAI(Symbol);
// doppelter Code? Wie kann man das besser machen? Eventuell Symbol mit in move übergeben?

impl Player for Human {
    fn mark(&self, board: &mut Board) {
        unimplemented!();
    }
}

impl Player for StupidAI {
    fn mark(&self, board: &mut Board) {
        let (i, j) = rand::thread_rng().choose(&empty_indices()).unwrap();
        board.0[i][j] = Field::Occupied(self.0);
    }
}

impl Player for SmartAI {
    fn mark(&self, board: &mut Board) {
        unimplemented!();
    }
}
