use mini_grep::Config;
use std::{env, process};

// I am learning from the Rust Programming Book.
// Still I do my own research side by side to find idiomatic way and reason behind it,
// if you wanna know just dm me I will share you the reason behind every idiomatic code.
// Don't expect me to go strictly according to the book, as I am not XD.

fn main() {
    // let input_args: Vec<String> = env::args().collect();

    // let config: Config = Config::build_v2(&input_args).unwrap_or_else(|err| {
    //     eprintln!("{err}");
    //     process::exit(1);
    // });

    let config: Config = Config::build_v1(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    config.run();
}
