extern crate clap;
extern crate rand;

use clap::{App, Arg, ArgMatches, SubCommand};
use rand::{Closed01, random};

fn main() {

    let flip = App::new("flip")
        .arg(
            Arg::with_name("times")
            .long("times")
            .global(true)
            .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("coin")
        )
        .subcommand(
            SubCommand::with_name("dice")
            .arg(
                Arg::with_name("sides")
                .long("sides")
                .takes_value(true)
            )
        )
        .subcommand(
            SubCommand::with_name("choose")
            .arg(
                Arg::with_name("count")
                .long("count")
                .takes_value(true)
            )
        )
        .get_matches();

        let times = match flip.value_of("times").unwrap_or("1").parse() {
            Ok(x) => x,
            Err(_) => {
                println!("{:?}", flip.usage());
                return;
            }
        };

        let (function, sub_matches): (fn(&ArgMatches), &ArgMatches) = match flip.subcommand() {
            ("coin", Some(args)) => (flip_coin, args),
            ("dice", Some(args)) => (throw_dice, args),
            ("choose", Some(args)) => (choose, args),
            _ => {
                println!("{:?}", flip.usage());
                return;
            }
        };

        for _ in 0..times {
            function(sub_matches);
        }

}

fn flip_coin(_: &ArgMatches) {
    let Closed01(side) = random::<Closed01<f32>>();
    if side < 0.5 {
        println!("heads");
    } else {
        println!("tails");
    }
}

fn throw_dice(_: &ArgMatches) {
    println!("dice");
}

fn choose(_: &ArgMatches) {
    println!("choose");
}
