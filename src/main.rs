use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
};
use std::path::Path;
use std::time::Instant;
use davelib::perceptron::daves_perceptron;
use davelib::utils::*;
use davelib::release;

fn argument_parser() -> ArgMatches {
    Command::new(release::DISPLAY_NAME)
        .version(release::VERSION_STR)
        .about(release::DISPLAY_DESCRIPTION)
        .arg(Arg::new("size")
            .long("size")
            .short('s')
            .value_name("path")
            .action(ArgAction::Set)
            .help("Check the size of a file or directory"))
        .arg(Arg::new("perceptron")
            .long("perceptron")
            .short('p')
            .action(ArgAction::SetTrue)
            .help("Behold my glorious Perceptron in Rust. A Perceptron\nis a computer model or computerized machine devised to represent or\nsimulate the ability of the brain to recognize and discriminate"))
        .get_matches()
}

fn main() {
    let start = Instant::now();
    let matches = argument_parser();

    if let Some(passed_directory) = matches.get_one::<String>("size") {
        let path = Path::new(passed_directory);
        if let Err(error) = get_file_size(path) {
            eprintln!("##==>>>> ERROR: {}", error);
        }
    }

    if matches.get_flag("perceptron") {
        if let Err(error) = daves_perceptron() {
            eprintln!("##==>>>> ERROR: {}", error);
        }
    }

    let time = start.elapsed();
    println!(
        "\n##==> Program Took {}.{}ms to Run",
        time.as_secs(),
        time.subsec_millis(),
    )
}
