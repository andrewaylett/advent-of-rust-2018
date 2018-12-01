extern crate advent_of_rust_2018;
extern crate clap;

use clap::{Arg, App, SubCommand};

macro_rules! subcommand {
    ($day:ident) => {
    {
        SubCommand::with_name(stringify!($day))
        .subcommand(SubCommand::with_name("one"))
        .subcommand(SubCommand::with_name("two"))
    }
    }
}

macro_rules! run {
    ($day:ident, $app:ident) => {
    {
        use advent_of_rust_2018::$day;
        if let Some(_matches) = $app.subcommand_matches(stringify!($day)) {
            $day();
        }
    }
    }
}

fn main() {
    let app = App::new("Advent of Rust 2018")
        .author("Andrew Aylett <andrew@aylett.co.uk>")
        .subcommand(subcommand!(day01))
        .get_matches();

    run!(day01, app);
}
