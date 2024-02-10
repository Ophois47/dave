use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
    value_parser,
};
use colored::*;
use std::borrow::BorrowMut;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;
use std::time::Instant;
use davelib::config::*;
use davelib::dave_grep;
use davelib::dave_grep::Config;
use davelib::dave_guess::guess_number;
use davelib::dave_hash::*;
use davelib::dave_land::dave_game_loop;
use davelib::dave_perceptron::daves_perceptron;
use davelib::utils::*;
use davelib::release;

fn argument_parser() -> ArgMatches {
    Command::new(release::DISPLAY_NAME)
        .version(release::VERSION_STR)
        .about(release::DISPLAY_DESCRIPTION)
        .arg(Arg::new("defaults")
            .long("defaults")
            .action(ArgAction::SetTrue)
            .help("Applies the default configuration"))
        .arg(Arg::new("config-path")
            .long("config-path")
            .value_name("path")
            .action(ArgAction::Set)
            .help("Point to a new location of the configuration file"))
        .arg(Arg::new("save-config")
            .long("save-config")
            .action(ArgAction::SetTrue)
            .help("Write this configuration to it's default location or the path specified by --config-path"))
        .arg(Arg::new("export-config")
            .long("export-config")
            .action(ArgAction::SetTrue)
            .help("Export configuration file in the selected output format"))
        .arg(Arg::new("size")
            .long("size")
            .short('s')
            .value_name("path")
            .help("Check the size of a file or directory"))
        .arg(Arg::new("hash")
            .long("hash")
            .short('h')
            .value_name("path")
            .help("Hash a file"))
        .arg(Arg::new("hash-type")
            .long("hash-type")
            .default_value("sha3-256")
            .value_name("hashing algorithm")
            .value_parser(["md5", "sha3-256", "sha3-384", "sha3-512"])
            .help("Chooses which hashing algorithm the program will use"))
        .arg(Arg::new("guess")
            .long("guess")
            .short('g')
            .value_name("guess")
            .help("Guess a number from 0 - 10 for funsies"))
        .arg(Arg::new("dgrep")
            .long("dgrep")
            .short('d')
            .action(ArgAction::Set)
            .value_names(["[options]", "[pattern]", "[file]"])
            .num_args(3)
            .value_parser(value_parser!(String))
            .help("Behold Dave's glorious implementation of grep in Rust.\nPass this function 'i' or 'insensitive' for case insensitive\nsearches, then pass a pattern to query and a\nfilename to search"))
        .arg(Arg::new("perceptron")
            .long("perceptron")
            .short('p')
            .action(ArgAction::SetTrue)
            .help("Behold Dave's glorious Perceptron in Rust. A Perceptron\nis a computer model or computerized machine devised to represent or\nsimulate the ability of the brain to recognize and discriminate"))
        .arg(Arg::new("dave-land")
            .long("dave-land")
            .action(ArgAction::SetTrue)
            .help("This is a text based adventure game by Dave"))
        .get_matches()
}

fn update_config<'a>(matches: &ArgMatches) {
    // Setup Config
    let mut writer = CONFIG.write().unwrap();
    if matches.get_flag("defaults") {
        *writer = DaveConfig::default();
    }
    let config: &mut DaveConfig = writer.borrow_mut();

    // Deal With Arguments Related to Output Paths
    if let Some(config_path) = matches.get_one::<String>("config-path") {
        let config_path_buf = PathBuf::from(config_path);
        config.set_config_path(config_path_buf);
    }

    // Deal With Config Arguments That are Flags or Bools

    // Deal With Saving Config to Proper File and Location
    if matches.get_flag("save-config") {
        let config_path = config.config_path();

        if let Err(error) = config.save() {
            eprintln!("##==>>>> ERROR: Unable to save configuration: {}: {}", config_path.display(), error);
            std::process::exit(1);
        }
        println!("##==> Successfully Wrote Configuration to: {}", config_path.display());
        std::process::exit(0);
    }

    // Deal With Determining Hashing Algorithm to Use
    if let Some(hash_choice) = matches.get_one::<String>("hash-type") {
        let hash_choice_parsed = HashType::from_str(hash_choice);
        config.hash_type = hash_choice_parsed.unwrap();
    }
}

fn main() {
    let start = Instant::now();

    // Setup Files Necessary for Output
    let mut file_options = OpenOptions::new();
    file_options.write(true);
    file_options.append(true);
    file_options.create(true);

    // Create Files That Will Have Important Data Written to Them
    let _output_file = match file_options.open(find_output_file()) {
        Ok(output_file) => output_file,
        Err(error) => {
            eprintln!("##==>>>> ERROR: {}: {}", find_output_file().display(), error);
            return
        }
    };

    // Parse CLI Args
    let matches = argument_parser();

    // Handle Options That Only Print Messages and Exit

    // Handle Configuration Updates
    update_config(&matches);

    if matches.contains_id("dgrep") {
        let dgrep_args: Vec<String> = matches.get_many("dgrep")
            .expect("##==>>>> ERROR: Missing Values")
            .cloned()
            .collect();
        
        let gotten_filename = &dgrep_args[2];
        if Path::new(gotten_filename).exists() {
            let config = Config::new(dgrep_args).unwrap_or_else(|error| {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error.red());
                process::exit(1);
            });
            if let Err(error) = dave_grep::run(config) {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        } else {
            eprintln!("{}", "##==>>>> ERROR: File Not Found".red());
        }
    }

    if let Some(passed_directory) = matches.get_one::<String>("size") {
        let path = Path::new(passed_directory);
        if let Err(error) = get_file_size(path) {
            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            process::exit(1);
        }
    }

    if let Some(passed_path) = matches.get_one::<String>("hash") {
        let path = Path::new(passed_path);
        if path.exists() {
            println!("{}", "##==> Path Exists! Continuing ...".green());
            match hash_file(CONFIG.read().unwrap().hash_type(), passed_path.into()) {
                Ok(_hash_result) => {
                    // println!("#==>> Hex Output: {:x?}", hash_result);
                },
                Err(error) => eprintln!("{}{}", "##==>>>> ERROR: ".red(), error),
            };
        } else {
            eprintln!("{}", "##==>>>> ERROR: File Not Found".red());
        }
    }

    if let Some(passed_value) = matches.get_one::<String>("guess") {
        let guess = passed_value.parse::<u16>().unwrap();
        if let Err(error) = guess_number(guess) {
            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            process::exit(1);
        }
    }

    if matches.get_flag("perceptron") {
        if let Err(error) = daves_perceptron() {
            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            process::exit(1);
        }
    }

    if matches.get_flag("dave-land") {
        if let Err(error) = dave_game_loop() {
            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
        }
    }

    let time = start.elapsed();
    println!(
        "\n##==> Program Took {}.{}ms to Run",
        time.as_secs(),
        time.subsec_millis(),
    )
}
