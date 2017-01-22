extern crate rand;
extern crate clap;

mod game;

use game::players::PlayerKind;
use clap::{App, Arg, ArgMatches};
use game::board::Symbol;

fn main() {
    let players = ["human", "smartAI", "stupidAI"];
    let matches = App::new("tictactoe")
        .arg(Arg::with_name("player1")
            .required(true)
            .takes_value(true)
            .possible_values(&players)
            .long("noughts")
            .short("o")
            .global(true))
        .arg(Arg::with_name("player2")
            .required(true)
            .takes_value(true)
            .possible_values(&players)
            .long("crosses")
            .short("x")
            .global(true))
        .get_matches();


    game::play(player_kind(Symbol::O, &matches), player_kind(Symbol::X, &matches));
}

fn player_kind(symbol: Symbol, matches: &ArgMatches) -> PlayerKind {
    match matches.value_of(
        match symbol {
            Symbol::O => "player1",
            Symbol::X => "player2",
        }
    ).unwrap() {
        "human" => PlayerKind::Human,
        "smartAI" => PlayerKind::SmartAI,
        "stupidAI" => PlayerKind::StupidAI,
        _ => panic!()
    }
}
