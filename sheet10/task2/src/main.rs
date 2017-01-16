extern crate clap;
extern crate rand;

use clap::{App, Arg, ArgMatches, SubCommand};
use rand::{Rng};

fn main() {

    let flip = App::new("flip")
        .arg(
            Arg::with_name("times")
            .long("times")
            .global(true)
            .default_value("1")
        )
        .subcommand(
            SubCommand::with_name("coin")
        )
        .subcommand(
            SubCommand::with_name("dice")
            .arg(
                Arg::with_name("sides")
                .long("sides")
                .default_value("6")
            )
        )
        .subcommand(
            SubCommand::with_name("choose")
            .arg(
                Arg::with_name("count")
                .long("count")
                .default_value("1")
            )
        )
        .get_matches();

        let times = match flip.value_of("times").unwrap().parse() {
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
    let side = rand::thread_rng().gen_range(0.0, 1.0);
    if side < 0.5 {
        println!("heads");
    } else {
        println!("tails");
    }
}

fn throw_dice(args: &ArgMatches) {
    if let Ok(sides) = args.value_of("sides").unwrap().parse::<u64>() {
        let side = rand::thread_rng().gen_range(1, sides + 1);
        println!("{:?}", side);
    }
}

fn choose(args: &ArgMatches) {
    if let Some(count) = args.subcommand_matches("count") {

    }
}
