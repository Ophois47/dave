use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
    value_parser,
};
use colored::*;
use std::path::Path;
use std::process;
use std::time::Instant;
use davelib::dave_grep;
use davelib::dave_grep::Config;
use davelib::dave_perceptron::daves_perceptron;
use davelib::utils::*;
use davelib::release;

fn argument_parser() -> ArgMatches {
    Command::new(release::DISPLAY_NAME)
        .version(release::VERSION_STR)
        .about(release::DISPLAY_DESCRIPTION)
        .arg(Arg::new("size")
            .long("size")
            .short('s')
            .value_name("Path")
            .action(ArgAction::Set)
            .help("Check the size of a file or directory"))
        .arg(Arg::new("guess")
            .long("guess")
            .short('g')
            .value_name("Guess")
            .action(ArgAction::Set)
            .help("Guess a number from 0 - 10 for funsies"))
        .arg(Arg::new("dgrep")
            .long("dgrep")
            .short('d')
            .value_names(["[options]", "[pattern]", "[file]"])
            .action(ArgAction::Append)
            .num_args(3)
            .value_parser(value_parser!(String))
            .help("Behold my glorious implementation of grep in Rust.\nPass this function 'i' or 'insensitive' for case insensitive\nsearches, then pass a pattern to query and a\nfilename to search"))
        .arg(Arg::new("dperceptron")
            .long("dperceptron")
            .short('p')
            .action(ArgAction::SetTrue)
            .help("Behold my glorious Perceptron in Rust. A Perceptron\nis a computer model or computerized machine devised to represent or\nsimulate the ability of the brain to recognize and discriminate"))
        .get_matches()
}

fn main() {
    let start = Instant::now();
    let matches = argument_parser();

    let dgrep_args: Vec<String> = matches.get_many("dgrep")
        .expect("##==>>>> ERROR: Missing Values")
        .cloned()
        .collect();

    let gotten_filename = &dgrep_args[2];
    if Path::new(gotten_filename).exists() {
        let config = Config::new(dgrep_args).unwrap_or_else(|error| {
            eprintln!("{}{}", "##==>>>> ERROR: Problem Parsing Arguments -> ".red(), error);
            process::exit(1);
        });
        if let Err(error) = dave_grep::run(config) {
            eprintln!("{}{}", "##==>>>> ERROR: Application Error -> ".red(), error);
            process::exit(1);
        }
    } else {
        eprintln!("{}", "##==>>>> ERROR: File Not Found".red());
    }

    if let Some(passed_directory) = matches.get_one::<String>("size") {
        let path = Path::new(passed_directory);
        if let Err(error) = get_file_size(path) {
            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            process::exit(1);
        }
    }

    if let Some(passed_value) = matches.get_one::<String>("guess") {
        let guess = passed_value.parse::<u16>().unwrap();
        if let Err(error) = guess_number(guess) {
            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            process::exit(1);
        }
    }

    if matches.get_flag("dperceptron") {
        if let Err(error) = daves_perceptron() {
            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            process::exit(1);
        }
    }

    let time = start.elapsed();
    println!(
        "\n##==> Program Took {}.{}ms to Run",
        time.as_secs(),
        time.subsec_millis(),
    )
}
