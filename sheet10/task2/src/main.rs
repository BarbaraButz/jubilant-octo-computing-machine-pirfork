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
            .arg(
                Arg::with_name("list")
                .required(true)
                .multiple(true)
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
    let side = rand::random::<bool>();
    if side {
        println!("heads");
    } else {
        println!("tails");
    }
}

fn throw_dice(args: &ArgMatches) {
    let sides = args.value_of("sides").unwrap().parse::<usize>().unwrap();
    let side = rand::thread_rng().gen_range(1, sides + 1);
    println!("{:?}", side);
}

fn choose(args: &ArgMatches) {
    let mut count = args.value_of("count").unwrap().parse::<usize>().unwrap();
    let elements = args.values_of("list").unwrap();
    let mut list = elements.collect::<Vec<&str>>();
    rand::thread_rng().shuffle(&mut list);
    count = std::cmp::min(count, list.len());
    for i in 0..count {
        println!("{:?}", list.get(i).unwrap());
    }
}
