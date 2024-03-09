use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
    value_parser,
};
use colored::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::borrow::BorrowMut;
use std::env;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;
use std::time::Instant;
use davelib::config::*;
use davelib::dave_currency::dave_currency_conv;
use davelib::dave_encrypt::*;
use davelib::dave_grep;
use davelib::dave_grep::Config;
use davelib::dave_guess::guess_number;
use davelib::dave_hash::*;
use davelib::dave_land::dave_game_loop;
use davelib::dave_perceptron::daves_perceptron;
use davelib::dave_rep_max::dave_rep_max_calc;
use davelib::utils::*;
use davelib::release;
use davelib::release::*;

fn argument_parser() -> ArgMatches {
    Command::new(release::DISPLAY_NAME)
        .version(release::VERSION_STR)
        .about(release::DISPLAY_DESCRIPTION)
        .subcommand(Command::new("config")
            .about("Save or set default settings for the program's config file, along with the path to it")
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
                .default_value("sha-256")
                .value_name("algorithm")
                .value_parser(["md5", "sha-256", "sha-384", "sha-512"])
                .num_args(1)
                .help("Chooses which hashing algorithm the program will use")))
        .subcommand(Command::new("guess")
            .about("Guess a number from 0 - 10 for funsies")
            .arg(Arg::new("number")
                .value_parser(value_parser!(u16))
                .num_args(1)))
        .subcommand(Command::new("dgrep")
            .about("Behold Dave's glorious implementation of GREP in Rust.")
            .arg(Arg::new("option")
                .long("option")
                .short('o')
                .num_args(1)
                .value_parser(value_parser!(String))
                .help("Pass '-o i' or '-o I' for case insensitivity. Pass '-o r' or '-o R' for REGEX pattern matching"))
            .arg(Arg::new("pattern")
                .value_parser(value_parser!(String))
                .value_name("pattern")
                .num_args(1)
                .help("The pattern for DGREP to match against"))
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("filename")
                .num_args(1)
                .help("The file or directory passed to DGREP for it to search through for the given pattern")))
        .subcommand(Command::new("crypt")
            .about("File Encryption and Decryption using a Passphrase")
            .arg(Arg::new("option")
                .long("option")
                .short('o')
                .value_parser(value_parser!(String))
                .num_args(1)
                .help("Pass '-o e' to encrypt a file or pass '-o d' to decrypt a file"))
            .arg(Arg::new("password")
                .long("password")
                .short('p')
                .value_parser(value_parser!(String))
                .num_args(1)
                .help("Provide a secure Passphrase for the hashing algoritm to use"))
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("filename")
                .num_args(1)
                .help("The file or directory passed to DCRYPT for it to do all its crypty business with")))
        .subcommand(Command::new("perceptron")
            .about("Behold Dave's glorious Perceptron in Rust. A Perceptron\nis a computer model or computerized machine devised to represent or\nsimulate the ability of the brain to recognize and discriminate"))
        .subcommand(Command::new("dave-land")
            .about("This is a text based adventure game by Dave"))
        .subcommand(Command::new("dcurrency")
            .about("Dave's implementation of a currency converter. Current as of 3-9-2024")
            .arg(Arg::new("option")
                .long("option")
                .short('o')
                .value_parser(value_parser!(String))
                .num_args(1)
                .help("Pass '-o a' or '-o all' to see each nation that uses the currency specified"))
            .arg(Arg::new("amount")
                .num_args(1)
                .value_parser(value_parser!(f32))
                .help("Pass an amount to convert into another world currency"))
            .arg(Arg::new("currency")
                .num_args(1)
                .value_parser(value_parser!(String))
                .value_parser(["USD", "EUR", "GBP", "JPY", "CAD", "CNY", "AUD", "CHF", "SEK", "INR", "KRW", "NOK", "NZD", "RUB", "BRL", "SAR", "ILS", "DKK", "PLN", "MXN"])
                .help("Pass a three letter ISO 4217 currency code to indicate the starting currency"))
            .arg(Arg::new("convert")
                .num_args(1)
                .value_parser(value_parser!(String))
                .value_parser(["USD", "EUR", "GBP", "JPY", "CAD", "CNY", "AUD", "CHF", "SEK", "INR", "KRW", "NOK", "NZD", "RUB", "BRL", "SAR", "ILS", "DKK", "PLN", "MXN"])
                .help("Enter the three letter ISO 4217 currency code you wish to convert your intial amount to")))
        .subcommand(Command::new("drm")
            .about("Calculate your max possible repetitions by giving your weight lifted and for how many reps")
            .arg(Arg::new("option")
                .long("option")
                .short('o')
                .default_value("lbs")
                .num_args(1)
                .value_parser(value_parser!(String))
                .value_parser(["lb", "kg", "lbs", "kgs", "pounds", "kilograms", "kilos"])
                .help("Pass '-o lb' for pounds or '-o kg' for kilograms."))
            .arg(Arg::new("weight")
                .value_name("weight lifted")
                .num_args(1)
                .value_parser(value_parser!(u16))
                .help("Enter the weight lifted during the movement."))
            .arg(Arg::new("reps")
                .value_name("repetitions completed")
                .num_args(1)
                .value_parser(value_parser!(u16))
                .help("Enter the amount of reps completed during the movement.")))
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

fn print_startup_message() {
    println!("##==> Dave Version: {}, Release: {}, Patchlevel: {} ({})", VERSION[0], VERSION[1], VERSION[2], BUILD_DATE);
    println!();
}

#[cfg(not(windows))]
fn setup_terminal() -> std::io::Result<()> {
    Ok(())
}

#[cfg(windows)]
fn setup_terminal() -> std::io::Result<()> {
    control::set_virtual_terminal(true).unwrap();
    Ok(())
}

fn main() {
    let start = Instant::now();
    // Print Program Startup Message
    print_startup_message();

    // Check Current OS to Determine Colored Terminal Output
    println!("##==> INFO! Found Operating System '{}'. Configuring Terminal Environment ...", env::consts::OS);
    if let Err(error) = setup_terminal() {
        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
    }
    println!();

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
            }
        },
        Some(("dave-land", _matches)) => {
            if let Err(error) = dave_game_loop() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("dcurrency", matches)) => {
            if let Some(passed_amount) = matches.get_one::<f32>("amount") {
                if let Some(passed_currency) = matches.get_one::<String>("currency") {
                    if let Some(passed_conversion) = matches.get_one::<String>("convert") {
                        if let Err(error) = dave_currency_conv(
                            *passed_amount,
                            passed_currency,
                            passed_conversion,
                        ) {
                            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        }
                    } else {
                        println!("##==> INFO! A currency to convert your amount to must be passed to the program. Try running 'dave dcurrency --help' for more information");
                    }
                } else {
                    println!("##==> INFO! A type of currency for your amount must be passed to the program. Try running 'dave dcurrency --help' for more information");
                }
            } else {
                println!("##==> INFO! An amount must be passed to the program. Try running 'dave dcurrency --help' for more information");
            }
        },
        Some(("drm", matches)) => {
            if let Some(passed_weight) = matches.get_one::<u16>("weight") {
                if let Some(passed_reps) = matches.get_one::<u16>("reps") {
                    let mut option: String = "".to_string();
                    if let Some(passed_option) = matches.get_one::<String>("option") {
                        if passed_option == "lb" || passed_option == "lbs" || passed_option == "pounds" {
                            option = "lbs".to_string();
                        } else if passed_option == "kg" || passed_option == "kgs" || passed_option == "kilograms" || passed_option == "kilos" {
                            option = "kgs".to_string();
                        }
                    }
                    if let Err(error) = dave_rep_max_calc(*passed_weight, *passed_reps, &option) {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    }
                } else {
                    println!("##==> INFO! An amount of reps completed must be passed to the program. Try running 'dave drm --help' for more information");
                }
            } else {
                println!("##==> INFO! A amount of weight lifted must be passed to the program. Try running 'dave drm --help' for more information");
            }
        },
        Some(("guess", matches)) => {
            if let Some(passed_value) = matches.get_one::<u16>("number") {
                if let Err(error) = guess_number(*passed_value) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else {
                println!("##==> INFO! A guess must be passed to the program. Try running 'dave guess --help' for more information");
            }
        },
        Some(("size", matches)) => {
            if let Some(passed_directory) = matches.get_one::<String>("filename") {
                let path = Path::new(passed_directory);
                if let Err(error) = get_file_size(path) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else {
                println!("##==> INFO! A file or path must be passed to the program. Try running 'dave size --help' for more information");
            }
        },
        Some(("crypt", matches)) => {
            if let Some(passed_file) = matches.get_one::<String>("filename") {
                let path = Path::new(passed_file);
                let mut passphrase: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();

                if path.exists() {
                    let mut option: String = "".to_string();
                    if let Some(passed_option) = matches.get_one::<String>("option") {
                        if passed_option == "e" || passed_option == "encrypt" {
                            option = "e".to_string();
                        } else if passed_option == "d" || passed_option == "decrypt" {
                            option = "d".to_string();
                        }
                    }
                    if let Some(passed_phrase) = matches.get_one::<String>("password") {
                        passphrase = passed_phrase.to_string();
                    }
                    if option == "e" {
                        println!("##==> Encrypting {} ...", path.display());
                        match dave_encrypt(&passphrase, path) {
                            Ok(encrypted_result) => {
                                println!("##==>> Encrypted Result: {:?}", encrypted_result);
                            },
                            Err(error) => eprintln!("{}{}", "##==>>>> ERROR: ".red(), error),
                        }
                    } else if option == "d" {
                        println!("##==> Decrypting {} ...", path.display());
                        match dave_decrypt(&passphrase, path) {
                            Ok(decrypted_result) => {
                                println!("##==>> Decrypted Result: {}", String::from_utf8_lossy(&decrypted_result));
                            },
                            Err(error) => eprintln!("{}{}", "##==>>>> ERROR: ".red(), error),
                        }
                    }
                } else {
                    eprintln!("{}'{}'", "##==>>>> ERROR: File Not Found: ".red(), path.display());
                }
            } else {
                println!("##==> INFO! A file or path must be passed to the program. Try running 'dave crypt --help' for more information");
            }
        },
        Some(("hash", matches)) => {
            if let Some(passed_path) = matches.get_one::<String>("filename") {
                let path = Path::new(passed_path);
                if path.exists() {
                    println!("{}", "##==> Path Exists! Continuing ...".green());
                    // Deal With Determining Hashing Algorithm to Use
                    let mut hash_type = HashType::Sha256;
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
                        Ok(hash_result) => {
                            let encoded_string = hex::encode(hash_result);
                            println!("#==>> {} Checksum: {}", hash_type, encoded_string);
                        },
                        Err(error) => eprintln!("{}{}", "##==>>>> ERROR: ".red(), error),
                    };
                } else {
                    eprintln!("{}'{}'", "##==>>>> ERROR: File Not Found: ".red(), passed_path);
                }
            } else {
                println!("##==> INFO! A file or path must be passed to the program. Try running 'dave hash --help' for more information");
            }
        },
        Some(("dgrep", matches)) => {
            let mut option = String::new();
            if let Some(gotten_option) = matches.get_one::<String>("option") {
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
                            filename.to_string(),
                        ).unwrap_or_else(|error| {
                            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error.red());
                            process::exit(1);
                        });
                        if let Err(error) = dave_grep::run(config) {
                            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        }
                    } else {
                        eprintln!("{}'{}'", "##==>>>> ERROR: File Not Found: ".red(), filename);
                    }
                } else {
                    println!("##==> INFO! A file or path must be passed to DGREP. Try running 'dave dgrep --help' for more information");
                }
            } else {
                println!("##==> INFO! A match pattern must be passed to DGREP. Try running 'dave dgrep --help' for more information");
            }
        },
        _ => { println!("##==> Try running the program with 'dave --help' to see a list of possible commands and options") },
    }

    let time = start.elapsed();
    println!(
        "\n##==> Dave Ran For {}.{}ms",
        time.as_secs(),
        time.subsec_millis(),
    )
}
