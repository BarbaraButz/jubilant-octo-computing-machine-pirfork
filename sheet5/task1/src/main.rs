extern crate term_painter;

pub mod db;
pub mod engine;
pub mod game;

use game::fight;

fn main() {
    fight();
}
