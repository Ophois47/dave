use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
};
use std::path::Path;
use std::time::Instant;
use davelib::utils::*;
use davelib::release;

fn argument_parser<'a>() -> ArgMatches {
    Command::new(release::DISPLAY_NAME)
        .version(release::VERSION_STR)
        .about(release::DISPLAY_DESCRIPTION)
        .arg(Arg::new("fsize")
            .long("fsize")
            .short('s')
            .value_name("path to scan")
            .action(ArgAction::Set)
            .help("Check the size of a file."))
        .get_matches()
}

fn main() {
    let start = Instant::now();
    let matches = argument_parser();

    if let Some(passed_directory) = matches.get_one::<String>("fsize") {
        let path = Path::new(passed_directory);
        if let Err(error) = get_file_size(path) {
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
