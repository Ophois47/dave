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
        .subcommand(Command::new("config")
            .about("Save or set default settings for the program's config file, along with the path")
            .arg(Arg::new("defaults")
                .long("defaults")
                .action(ArgAction::SetTrue)
                .help("Applies the default configuration"))
            .arg(Arg::new("path")
                .long("path")
                .value_name("path")
                .num_args(1)
                .help("Point to a new location for the configuration file"))
            .arg(Arg::new("save")
                .long("save")
                .action(ArgAction::SetTrue)
                .help("Write this configuration to its default location or the path specified by config --path")))
        .subcommand(Command::new("size")
            .about("Check the size of a file or directory")
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .num_args(1)))
        .subcommand(Command::new("hash")
            .about("Hash a file using a preferred hashing algorithm")
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .num_args(1))
            .arg(Arg::new("hash-type")
                .long("hash-type")
                .default_value("sha3-256")
                .value_name("algorithm")
                .value_parser(["md5", "sha3-256", "sha3-384", "sha3-512"])
                .num_args(1)
                .help("Chooses which hashing algorithm the program will use")))
        .subcommand(Command::new("guess")
            .about("Guess a number from 0 - 10 for funsies")
            .arg(Arg::new("number")
                .value_parser(value_parser!(u16))
                .num_args(1)))
        .subcommand(Command::new("dgrep")
            .about("Behold Dave's glorious implementation of grep in Rust.\nPass this function 'i' or 'insensitive' for case insensitive\nsearches, then pass a pattern to query and a\nfilename to search")
            .arg(Arg::new("option")
                .long("option")
                .short('o')
                .value_name("option")
                .num_args(1)
                .value_parser(value_parser!(String))
                .help("Type 'i' for case insensitivity"))
            .arg(Arg::new("pattern")
                .value_parser(value_parser!(String))
                .value_name("pattern")
                .num_args(1)
                .help("The pattern for DGREP to match against"))
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("filename")
                .num_args(1)
                .help("The file or directory passed to DGREP for it to search through")))
        .subcommand(Command::new("perceptron")
            .about("Behold Dave's glorious Perceptron in Rust. A Perceptron\nis a computer model or computerized machine devised to represent or\nsimulate the ability of the brain to recognize and discriminate"))
        .subcommand(Command::new("dave-land")
            .about("This is a text based adventure game by Dave"))
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
    if let Some(config_path) = matches.get_one::<String>("path") {
        let config_path_buf = PathBuf::from(config_path);
        config.set_config_path(config_path_buf);
    }

    // Deal With Config Arguments That are Flags or Bools

    // Deal With Saving Config to Proper File and Location
    if matches.get_flag("save") {
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

    // Deal With Passed Subcommands and Their Arguments
    match matches.subcommand() {
        Some(("config", matches)) => {
            update_config(&matches);
        },
        Some(("perceptron", _matches)) => {
            if let Err(error) = daves_perceptron() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                process::exit(1);
            }
        },
        Some(("dave-land", _matches)) => {
            if let Err(error) = dave_game_loop() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("guess", matches)) => {
            if let Some(passed_value) = matches.get_one::<u16>("number") {
                if let Err(error) = guess_number(*passed_value) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    process::exit(1);
                }
            }
        },
        Some(("size", matches)) => {
            if let Some(passed_directory) = matches.get_one::<String>("size") {
                let path = Path::new(passed_directory);
                if let Err(error) = get_file_size(path) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    process::exit(1);
                }
            }
        },
        Some(("hash", matches)) => {
            if let Some(passed_path) = matches.get_one::<String>("filename") {
                let path = Path::new(passed_path);
                if path.exists() {
                    println!("{}", "##==> Path Exists! Continuing ...".green());
                    // Deal With Determining Hashing Algorithm to Use
                    let mut hash_type = HashType::Sha3_256;
                    if let Some(hash_choice) = matches.get_one::<String>("hash-type") {
                        let hash_choice_parsed: Result<HashType, DaveError> = HashType::from_str(hash_choice);
                        match hash_choice_parsed {
                            Ok(ht) => {
                                hash_type = ht;
                            },
                            Err(error) => {
                                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                            },
                        }
                    }
                    match hash_file(hash_type, passed_path.into()) {
                    Ok(_hash_result) => {
                        // println!("#==>> Hex Output: {:x?}", hash_result);
                    },
                    Err(error) => eprintln!("{}{}", "##==>>>> ERROR: ".red(), error),
                    };
                } else {
                    eprintln!("{}", "##==>>>> ERROR: File Not Found".red());
                }
            }
        },
        Some(("dgrep", matches)) => {
            let mut option = String::new();
            if let Some(gotten_option) = matches.get_one::<String>("option") {
                println!("GOTTEN OPTION: {}", gotten_option);
                option = gotten_option.to_string();
            }
            
            if let Some(gotten_pattern) = matches.get_one::<String>("pattern") {
                let pattern = gotten_pattern.to_string();
                if let Some(gotten_filename) = matches.get_one::<String>("filename") {
                    let filename = gotten_filename.to_string();
                    if Path::new(&filename).exists() {
                        let config = Config::new(
                            option.to_string(),
                            pattern.to_string(),
                            filename.to_string()
                        ).unwrap_or_else(|error| {
                            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error.red());
                            process::exit(1);
                        });
                        if let Err(error) = dave_grep::run(config) {
                            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        }
                    } else {
                        eprintln!("{}{}", "##==>>>> ERROR: File Not Found: ".red(), filename);
                    }
                }
            }
        },
        _ => {},
    }

    let time = start.elapsed();
    println!(
        "\n##==> Program Took {}.{}ms to Run",
        time.as_secs(),
        time.subsec_millis(),
    )
}
