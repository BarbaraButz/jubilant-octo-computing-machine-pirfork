extern crate rand;
extern crate clap;

mod game;

use game::players::PlayerKind;
use clap::{App, Arg, ArgMatches};
use game::board::Symbol;

fn main() {
    let players = ["human", "smartAI", "stupidAI"];
    let matches = App::new("tictactoe")
        .arg(Arg::with_name("noughts")
            .required(true)
            .takes_value(true)
            .possible_values(&players)
            .long("noughts")
            .short("o"))
        .arg(Arg::with_name("crosses")
            .required(true)
            .takes_value(true)
            .possible_values(&players)
            .long("crosses")
            .short("x"))
        .get_matches();


    game::play(player_kind(Symbol::O, &matches), player_kind(Symbol::X, &matches));
}

fn player_kind(symbol: Symbol, matches: &ArgMatches) -> PlayerKind {
    match matches.value_of(
        match symbol {
            Symbol::O => "noughts",
            Symbol::X => "crosses",
        }
    ).unwrap() {
        "human" => PlayerKind::Human,
        "smartAI" => PlayerKind::SmartAI,
        "stupidAI" => PlayerKind::StupidAI,
        _ => panic!()
    }
}
