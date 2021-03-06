extern crate cgmath;
extern crate getopts;
extern crate gfx;
#[macro_use]
extern crate log;
extern crate rand;
extern crate vangers;

mod game;
mod physics;
#[path = "../boilerplate.rs"]
mod boilerplate;

fn main() {
    use std::env;

    let (mut harness, settings, main_targets) = boilerplate::Harness::init();

    info!("Parsing command line");
    let args: Vec<_> = env::args().collect();
    let mut options = getopts::Options::new();
    options
        .parsing_style(getopts::ParsingStyle::StopAtFirstFree)
        .optflag("h", "help", "print this help menu");

    let matches = options.parse(&args[1 ..]).unwrap();
    if matches.opt_present("h") || !matches.free.is_empty() {
        println!("Vangers game prototype");
        let brief = format!("Usage: {} [options]", args[0]);
        println!("{}", options.usage(&brief));
        return;
    }

    let game = game::Game::new(&settings, main_targets, &mut harness.factory);

    harness.main_loop(game);
}
