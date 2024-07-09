use clap::{
    Command,
    Arg,
    ArgAction,
    ArgMatches,
    value_parser,
};
use colored::*;
use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use rand::{
    thread_rng,
    Rng,
};
use rand::distributions::Alphanumeric;
use sha2::Digest;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::borrow::BorrowMut;
use std::env;
use std::fs::{
    self,
    File,
    OpenOptions,
};
use std::io::{
    self,
    Read,
    stdout,
    Write,
};
use std::path::{
    Path,
    PathBuf,
};
use std::process;
use std::str::FromStr;
use std::time::Instant;
use davelib::config::*;
use davelib::dave_breakout::dave_breakout_main;
use davelib::dave_budget::DaveBudget;
use davelib::dave_calcs::*;
use davelib::dave_cars::dave_cars_main;
use davelib::dave_chip8::*;
use davelib::dave_conversions::*;
use davelib::dave_currency::dave_currency_conv;
use davelib::dave_db::DaveDatabase;
use davelib::dave_ecs::dave_ecs_main;
use davelib::dave_encrypt::*;
use davelib::dave_game::davegame_main;
use davelib::dave_graphics::{
    daves_animated_fox_main,
    daves_animated_foxes_main,
    daves_atmo_fog_main,
    daves_cube_main,
    daves_lights_main,
    daves_morph_main,
    daves_pbr_main,
    daves_render_viewer_main,
    daves_shapes_main,
};
use davelib::dave_grep::{
    self,
    Config,
};
use davelib::dave_guess::guess_number;
use davelib::dave_gui::dave_gui;
use davelib::dave_hash::*;
use davelib::dave_land::dave_game_loop;
use davelib::dave_machine::*;
use davelib::dave_notes::*;
use davelib::dave_parse::parse_handle_file;
use davelib::dave_perceptron::daves_perceptron;
use davelib::dave_port_scan::port_scan_main;
use davelib::dave_quiz::*;
use davelib::dave_rep_max::dave_rep_max_calc;
use davelib::dave_scrape::*;
use davelib::dave_skybox::daves_skybox_main;
use davelib::dave_snake::Game;
use davelib::dave_stress_tests::{
    davemark_main,
    st_too_many_buttons_main,
    st_too_many_lights_main,
};
use davelib::dave_tic_tac_toe::tic_tac_toe_main;
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
            .arg(Arg::new("bpath")
                .long("bpath")
                .value_name("path")
                .num_args(1)
                .help("Point to a new location for the budget file"))
            .arg(Arg::new("save")
                .long("save")
                .action(ArgAction::SetTrue)
                .help("Write this configuration to its default location or the path specified by config --path")))
        .subcommand(Command::new("size")
            .about("Check the size of a file or directory")
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("path")
                .num_args(1)))
        .subcommand(Command::new("find")
            .about("Find some shit in some shit")
            .arg(Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .help("Receive more information from output"))
            .arg(Arg::new("pattern")
                .value_parser(value_parser!(String))
                .value_name("pattern")
                .num_args(1)
                .help("The pattern for DGREP to match against"))
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("filename")
                .num_args(1)))
        .subcommand(Command::new("calc")
            .about("Use the program's calculator")
            .arg(Arg::new("simple")
                .long("simple")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Simple Calculator by Dave"))
            .arg(Arg::new("income")
                .long("income")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Simple Income Calculator by Dave"))
            .arg(Arg::new("interest")
                .long("interest")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Simple Interest Calculator by Dave")))
        .subcommand(Command::new("chip8")
            .about("Use Dave's very own rudimentary Chip8 emulator")
            .arg(Arg::new("pixel")
                .long("pixel")
                .short('p')
                .value_parser(value_parser!(String))
                .value_name("char/string")
                .num_args(1)
                .help("Choose the pixel string or character for the Chip8 Emulator to use"))
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("path")
                .num_args(1)))
        .subcommand(Command::new("quiz")
            .about("Take David's quiz")
            .arg(Arg::new("animals")
                .long("animals")
                .action(ArgAction::SetTrue)
                .help("Take the Quiz about Animals"))
            .arg(Arg::new("mil-av")
                .long("mil-av")
                .action(ArgAction::SetTrue)
                .help("Take the military aviation Quiz"))
            .arg(Arg::new("gen-av")
                .long("gen-av")
                .action(ArgAction::SetTrue)
                .help("Take the general aviation Quiz"))
            .arg(Arg::new("strek")
                .long("strek")
                .action(ArgAction::SetTrue)
                .help("Take the Star Trek Quiz"))
            .arg(Arg::new("swars")
                .long("swars")
                .action(ArgAction::SetTrue)
                .help("Take the Star Wars Quiz"))
            .arg(Arg::new("difficulty")
                .long("difficulty")
                .short('d')
                .num_args(1)
                .value_parser(value_parser!(String))
                .default_value("easy")
                .value_name("difficulty level")
                .help("Set the difficulty which affects the timer length for each question"))
            .arg(Arg::new("#")
                .long("#")
                .short('#')
                .num_args(1)
                .value_parser(value_parser!(usize))
                .default_value("5")
                .value_name("questions")
                .help("Set the number of questions to answer. Maximum 15")))
        .subcommand(Command::new("conv")
            .about("An amount of conversions the program will perform for you")
            .arg(Arg::new("F")
                .long("F")
                .short('f')
                .num_args(1)
                .value_parser(value_parser!(f32))
                .value_name("DEGREES")
                .help("Convert From Fahrenheit"))
            .arg(Arg::new("C")
                .long("C")
                .short('c')
                .num_args(1)
                .value_parser(value_parser!(f32))
                .value_name("DEGREES")
                .help("Convert From Celsius"))
            .arg(Arg::new("K")
                .long("K")
                .short('k')
                .num_args(1)
                .value_parser(value_parser!(f32))
                .value_name("DEGREES")
                .help("Convert From Kelvin"))
            .arg(Arg::new("LB")
                .long("LB")
                .num_args(1)
                .value_parser(value_parser!(f32))
                .value_name("LBS")
                .help("Convert Pounds to Kilograms"))
            .arg(Arg::new("KG")
                .long("KG")
                .num_args(1)
                .value_parser(value_parser!(f32))
                .value_name("KGS")
                .help("Convert Kilograms to Pounds"))
            .arg(Arg::new("MPH")
                .long("MPH")
                .num_args(1)
                .value_parser(value_parser!(f32))
                .value_name("MPH")
                .help("Convert Miles per Hour to Kilometers per Hour"))
            .arg(Arg::new("KPH")
                .long("KPH")
                .num_args(1)
                .value_parser(value_parser!(f32))
                .value_name("KPH")
                .help("Convert Kilometers per Hour to Miles per Hour")))
        .subcommand(Command::new("hash")
            .about("Hash a file using a preferred hashing algorithm")
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("path")
                .num_args(1))
            .arg(Arg::new("hash-type")
                .long("hash-type")
                .value_name("algorithm")
                .default_value("sha-256")
                .value_parser(["md5", "sha-256", "sha-384", "sha-512"])
                .num_args(1)
                .help("Chooses which hashing algorithm the program will use")))
        .subcommand(Command::new("guess")
            .about("Guess a number from 0 - 10 for funsies")
            .arg(Arg::new("number")
                .value_parser(value_parser!(u16))
                .value_name("value")
                .num_args(1)))
        .subcommand(Command::new("machine")
            .about("Run a series of machine learning algorithms")
            .arg(Arg::new("kmeans")
                .long("kmeans")
                .short('k')
                .action(ArgAction::SetTrue)
                .help("Run a standard NumPy compatible K-Means algorithm"))
            .arg(Arg::new("dbscan")
                .long("dbscan")
                .short('d')
                .action(ArgAction::SetTrue)
                .help("Run a standard NumPy compatible DBScan algorithm")))
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
                .value_name("passphrase")
                .help("Provide a secure Passphrase for the hashing algoritm to use"))
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("filename")
                .num_args(1)
                .help("The file or directory passed to DCRYPT for it to do all its crypty business with")))
        .subcommand(Command::new("scrape")
            .about("This program will scrape the internet for valuable and tasty information")
            .arg(Arg::new("weather")
                .long("weather")
                .action(ArgAction::SetTrue)
                .help("See the current weather"))
            .arg(Arg::new("top100")
                .long("top100")
                .action(ArgAction::SetTrue)
                .help("See the IMDB top 100 list"))
            .arg(Arg::new("sports")
                .long("sports")
                .value_parser(value_parser!(String))
                .value_name("sport")
                .num_args(1)
                .value_parser(["NBA", "NHL", "NFL", "MLB", "MLS", "WNBA", "NCAA-FB", "NCAA-BB"])
                .help("See the scores for today"))
            .arg(Arg::new("dcs")
                .long("dcs")
                .action(ArgAction::SetTrue)
                .help("Recent DCS: Digital Combat Simulator news")))
        .subcommand(Command::new("perceptron")
            .about("Behold Dave's glorious Perceptron in Rust. A Perceptron\nis a computer model or computerized machine devised to represent or\nsimulate the ability of the brain to recognize and discriminate"))
        .subcommand(Command::new("dave-land")
            .about("This is a text based adventure game by Dave"))
        .subcommand(Command::new("snake")
            .about("This is a classic Snake game by Dave"))
        .subcommand(Command::new("breakout")
            .about("This is a classic Break game by Dave, written with Bevy"))
        .subcommand(Command::new("davemark")
            .about("This is DaveMark. A stress testing program for modern machines"))
        .subcommand(Command::new("davegame")
            .about("Dave's Game"))
        .subcommand(Command::new("ecs")
            .about("Dave's Entity Component System in full action"))
        .subcommand(Command::new("cars")
            .about("A game of cars by Dave"))
        .subcommand(Command::new("ls")
            .about("Directory traversal")
            .arg(Arg::new("filename")
                .value_parser(value_parser!(String))
                .value_name("directory")
                .num_args(1)))
        .subcommand(Command::new("st-lights")
            .about("Dave's stress test of too many lights"))
        .subcommand(Command::new("st-buttons")
            .about("A series of 2D button stress test for your system")
            .arg(Arg::new("buttons")
                .long("buttons")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .default_value("110")
                .help("How many buttons to display"))
            .arg(Arg::new("img-frq")
                .long("img-frq")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .default_value("4")
                .help("How many Nth buttons will display images"))
            .arg(Arg::new("grid")
                .long("grid")
                .action(ArgAction::SetTrue)
                .help("Display the grid layout model"))
            .arg(Arg::new("borders")
                .long("borders")
                .action(ArgAction::SetTrue)
                .help("Display borders around each button"))
            .arg(Arg::new("text")
                .long("text")
                .action(ArgAction::SetTrue)
                .help("Display text to on each button")))
        .subcommand(Command::new("bevy")
            .about("A series of 3D graphical environments by Dave")
            .arg(Arg::new("cubeland")
                .long("cubeland")
                .action(ArgAction::SetTrue)
                .help("Enter the world of the cubes"))
            .arg(Arg::new("shapeland")
                .long("shapeland")
                .action(ArgAction::SetTrue)
                .help("Be dazzled by the world of the shapes"))
            .arg(Arg::new("morph")
                .long("morph")
                .action(ArgAction::SetTrue)
                .help("Colorful wiggle bar animation. Wee!"))
            .arg(Arg::new("pbr")
                .long("pbr")
                .action(ArgAction::SetTrue)
                .help("Configured Physically Based Rendering example"))
            .arg(Arg::new("fog")
                .long("fog")
                .action(ArgAction::SetTrue)
                .help("Behold! Atmospheric Fog simulation"))
            .arg(Arg::new("skybox")
                .long("skybox")
                .action(ArgAction::SetTrue)
                .help("Enter the skybox"))
            .arg(Arg::new("render")
                .long("render")
                .action(ArgAction::SetTrue)
                .help("Load and render glTF file models of your choice"))
            .arg(Arg::new("fox")
                .long("fox")
                .action(ArgAction::SetTrue)
                .help("Play with David's animated Fox"))
            .arg(Arg::new("foxes")
                .long("foxes")
                .action(ArgAction::SetTrue)
                .help("Play with David's animated Foxes"))
            .arg(Arg::new("lighting")
                .long("lighting")
                .action(ArgAction::SetTrue)
                .help("Bask in the glory of the lights made by Dave")))
        .subcommand(Command::new("gui")
            .about("This is Dave's gooey"))
        .subcommand(Command::new("tic-tac-toe")
            .about("This is a classic Tic-Tac-Toe game by Dave"))
        .subcommand(Command::new("my-sys")
            .about("This lets you view your current system info"))
        .subcommand(Command::new("port-scan")
            .about("This is a port scanner by Dave")
            .arg(Arg::new("target")
                .help("The target to scan")
                .index(1))
            .arg(Arg::new("concurrency")
                .help("Concurrency")
                .long("concurrency")
                .short('c')
                .default_value("1002"))
            .arg(Arg::new("verbose")
                .help("Display more detailed information")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue))
            .arg(Arg::new("full")
                .help("Scan all 65535 ports")
                .long("full")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("timeout")
                .help("Connection timeout")
                .long("timeout")
                .short('t')
                .default_value("3")))
        .subcommand(Command::new("get-rand")
            .about("Get a random value by supplying the minimum and maximum possible values")
            .arg(Arg::new("bounds")
                .num_args(2)
                .value_delimiter(' ')
                .value_names(["MIN_VALUE", "MAX_VALUE"])
                .value_parser(value_parser!(u16))
                .help("Pass a minimum and maximum value to get a random value from")))
        .subcommand(Command::new("parse")
            .about("Parse and get information for any sort of file.")
            .arg(Arg::new("file")
                .value_name("FILE")
                .num_args(1)
                .value_parser(value_parser!(String))
                .help("Pass the file you wish to have parsed")))
        .subcommand(Command::new("note")
            .about("This is a notes keeping program")
            .arg(Arg::new("add")
                .long("add")
                .short('a')
                .value_parser(value_parser!(String))
                .value_name("NOTE NAME")
                .num_args(1)
                .help("Add a new note"))
            .arg(Arg::new("list")
                .long("list")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("List existing notes"))
            .arg(Arg::new("overwrite")
                .long("overwrite")
                .short('o')
                .action(ArgAction::SetTrue)
                .help("Overwrite and wipe the existing Notes database"))
            .arg(Arg::new("complete")
                .long("complete")
                .short('c')
                .value_parser(value_parser!(u64))
                .value_name("ID #")
                .num_args(1)
                .help("Complete an existing note by passing the Note ID#, found when running note with '--list'")))
        .subcommand(Command::new("budget")
            .about("Budget your income and become WEALTHY. Thanks to Dave")
            .arg(Arg::new("new")
                .long("new")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Budget new will create a new budget. Wiping the old one"))
            .arg(Arg::new("income")
                .long("income")
                .short('i')
                .num_args(1)
                .value_name("AMOUNT")
                .value_parser(value_parser!(f64))
                .help("Add an amount of income to your budget"))
            .arg(Arg::new("expense")
                .long("expense")
                .short('e')
                .num_args(2)
                .value_name("amount")
                .value_delimiter(' ')
                .value_names(["expense", "amount"])
                .value_parser(value_parser!(String))
                .help("Subtract an expense from your budget. Pass a expense and amount"))
            .arg(Arg::new("summary")
                .long("summary")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Budget summary will print a summary of the current budget to the screen")))
        .subcommand(Command::new("currency")
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
                .value_name("VALUE")
                .help("Pass an amount to convert into another world currency"))
            .arg(Arg::new("currency")
                .num_args(1)
                .value_name("CODE")
                .value_parser(value_parser!(String))
                .value_parser(["USD", "EUR", "GBP", "JPY", "CAD", "CNY", "AUD", "CHF", "SEK", "INR", "KRW", "NOK", "NZD", "RUB", "BRL", "SAR", "ILS", "DKK", "PLN", "MXN"])
                .help("Pass a three letter ISO 4217 currency code to indicate the starting currency"))
            .arg(Arg::new("convert")
                .num_args(1)
                .value_name("CODE")
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
                .help("Enter the weight lifted during the movement"))
            .arg(Arg::new("reps")
                .value_name("repetitions completed")
                .num_args(1)
                .value_parser(value_parser!(u16))
                .help("Enter the amount of reps completed during the movement")))
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
    if let Some(budget_path) = matches.get_one::<String>("bpath") {
        let budget_path_buf = PathBuf::from(budget_path);
        config.set_budget_path(budget_path_buf);
    }

    // Deal With Config Arguments That are Flags or Bools

    // Deal With Saving Config to Proper File and Location
    if matches.get_flag("save") {
        let config_path = config.config_path();

        if let Err(error) = config.save() {
            eprintln!(
                "##==>>>> ERROR: Unable to save configuration: {}: {}",
                config_path.display(),
                error,
            );
            std::process::exit(1)
        }
        println!(
            "##==> Successfully Wrote Configuration to: {}",
            config_path.display(),
        );
        std::process::exit(0)
    }

    // Deal With Determining Hashing Algorithm to Use
    if let Some(hash_choice) = matches.get_one::<String>("hash-type") {
        let hash_choice_parsed = HashType::from_str(hash_choice);
        config.hash_type = hash_choice_parsed.unwrap();
    }
}

fn print_startup_message() {
    println!(
        "##==> Dave Version: {}, Release: {}, Patchlevel: {} ({})",
        VERSION[0],
        VERSION[1],
        VERSION[2],
        BUILD_DATE,
    );
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
    println!(
        "##==> INFO! Found Operating System '{}'. Configuring Terminal Environment ...",
        env::consts::OS,
    );
    if let Err(error) = setup_terminal() {
        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
    }
    println!();

    // Setup Files Necessary for Output
    let mut file_options = OpenOptions::new();
    file_options.write(true);
    file_options.create(true);

    // Create Files That Will Have Important Data Written to Them
    let _output_file = match file_options.append(true).open(find_output_file()) {
        Ok(output_file) => output_file,
        Err(error) => {
            eprintln!("##==>>>> ERROR: {}: {}", find_output_file().display(), error);
            return
        }
    };

    // Parse CLI Args
    let matches = argument_parser();

    // Get Important Data From Config
    let reader = CONFIG.read().unwrap();

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
        Some(("gui", _matches)) => {
            if let Err(error) = dave_gui() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("snake", _matches)) => {
            print!("{esc}c", esc = 27 as char);
            if let Err(error) = Game::new(stdout(), 15, 10).unwrap().run() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("breakout", _matches)) => {
            if let Err(error) = dave_breakout_main() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("tic-tac-toe", _matches)) => {
            if let Err(error) = tic_tac_toe_main() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("my-sys", _matches)) => {
            if let Err(error) = get_system_info() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("davemark", _matches)) => {
            if let Err(error) = davemark_main() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("davegame", _matches)) => {
            if let Err(error) = davegame_main() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("ecs", _matches)) => {
            if let Err(error) = dave_ecs_main() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("cars", _matches)) => {
            if let Err(error) = dave_cars_main() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("ls", matches)) => {
            if let Some(gotten_dir) = matches.get_one::<String>("filename") {
                if let Err(error) = dave_ls_main(gotten_dir.to_string()) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else {
                println!(
                    "##==>>>> ERROR: A valid path must be passed to the program. Try running 'dave ls --help' for more information",
                );
            }
        },
        Some(("find", matches)) => {
            if let Some(gotten_pattern) = matches.get_one::<String>("pattern") {
                if let Some(gotten_file) = matches.get_one::<String>("filename") {
                    let mut verbose: usize = 0;
                    if matches.get_flag("verbose") {
                        verbose = 1;
                    }

                    if let Err(error) = dave_find_main(
                        gotten_pattern.to_string(),
                        Path::new(gotten_file),
                        verbose,
                    ) {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    }
                } else {
                    println!(
                        "##==>>>> ERROR: A valid file must be passed to the program. Try running 'dave find --help' for more information",
                    );
                }
            } else {
                println!(
                    "##==>>>> ERROR: A valid pattern must be passed to the program. Try running 'dave find --help' for more information",
                );
            }
        },
        Some(("st-lights", _matches)) => {
            if let Err(error) = st_too_many_lights_main() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("st-buttons", matches)) => {
            // Gather User Arguments
            let num_buttons = matches.get_one::<usize>("buttons");
            let img_frq = matches.get_one::<usize>("img-frq");
            let grid = matches.get_flag("grid");
            let borders = matches.get_flag("borders");
            let text = matches.get_flag("text");

            if let Err(error) = st_too_many_buttons_main(
                num_buttons.unwrap(),
                img_frq.unwrap(),
                grid,
                borders,
                text,
            ) {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("bevy", matches)) => {
            if matches.get_flag("cubeland") {
                if let Err(error) = daves_cube_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("shapeland") {
                if let Err(error) = daves_shapes_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("morph") {
                if let Err(error) = daves_morph_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("pbr") {
                if let Err(error) = daves_pbr_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("fog") {
                if let Err(error) = daves_atmo_fog_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("skybox") {
                if let Err(error) = daves_skybox_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("render") {
                if let Err(error) = daves_render_viewer_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("fox") {
                if let Err(error) = daves_animated_fox_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("foxes") {
                if let Err(error) = daves_animated_foxes_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("lighting") {
                if let Err(error) = daves_lights_main() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
        },
        Some(("port-scan", matches)) => {
            let full = matches.get_flag("full");
            let verbose = matches.get_flag("verbose");
            let concurrency = matches
                .get_one::<String>("concurrency")
                .unwrap()
                .parse::<usize>()
                .unwrap_or(1002);
            let timeout = matches
                .get_one::<String>("timeout")
                .unwrap()
                .parse::<u64>()
                .unwrap_or(3);
            let default_target_string = "None".to_string();
            let target = matches
                .get_one::<String>("target")
                .unwrap_or(&default_target_string);

            if target == "None" {
                println!(
                    "##==>>>> ERROR: A valid IP Address must be passed to the program. Try running 'dave port-scan --help' for more information\n",
                );
                return
            }

            if verbose {
                let ports = if full {
                    String::from("all the 65535 ports")
                } else {
                    String::from("the most common 1002 ports")
                };
                println!(
                    "##==> Scanning {} of {}\n##==> Concurrency: {:?}\n##==> Timeout: {:?}\n",
                    &ports,
                    target,
                    concurrency,
                    timeout,
                );
            }

            if let Err(error) = port_scan_main(full, concurrency, timeout, target.to_string()) {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("get-rand", matches)) => {
            if let Some(bounds) = matches.get_many::<u16>("bounds") {
                let mut bounds_vec = vec![];
                for bound in bounds {
                    bounds_vec.push(bound);
                }
                let min_value = bounds_vec[0];
                let max_value = bounds_vec[1];

                if min_value > max_value {
                    eprintln!(
                        "{}",
                        "##==>>>> ERROR: Minimum value cannot be greater than max value\n".red(),
                    );
                    return
                }

                let random_value = generate_random_number(*min_value, *max_value);
                println!(
                    "##==>> Random Value Between {} and {}: {}",
                    min_value,
                    max_value,
                    random_value,
                );
            } else {
                println!("##==> A valid minimum and maximum value must be chosen. Try running 'dave get-rand --help' for more information")
            }
        },
        Some(("calc", matches)) => {
            if matches.get_flag("simple") {
                if let Err(error) = dave_simple_calc_loop() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else if matches.get_flag("income") {
                if let Err(error) = dave_income_calc_loop() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else if matches.get_flag("interest") {
                if let Err(error) = dave_interest_calc_loop() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
        },
        Some(("machine", matches)) => {
            if matches.get_flag("kmeans") {
                if let Err(error) = kmeans_task() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else if matches.get_flag("dbscan") {
                if let Err(error) = dbscan_task() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else {
                println!("##==> A valid Machine Learning algorithm must be chosen. Try running 'dave machine --help' for more information")
            }
        },
        Some(("conv", matches)) => {
            if let Some(fahrenheit_amount) = matches.get_one::<f32>("F") {
                let celsius_result = fahrenheit_to_celsius(*fahrenheit_amount);
                let kelvin_result = fahrenheit_to_kelvin(*fahrenheit_amount);
                println!(
                    "##==>> {} Degrees Fahrenheit is equal to {} Degrees Celsius and {:.2} Degrees Kelvin",
                    *fahrenheit_amount as i32,
                    celsius_result as i32,
                    kelvin_result,
                );
            } else if let Some(celsius_amount) = matches.get_one::<f32>("C") {
                let fahrenheit_result = celsius_to_fahrenheit(*celsius_amount);
                let kelvin_result = celsius_to_kelvin(*celsius_amount);
                println!(
                    "##==>> {} Degrees Celsius is equal to {} Degrees Fahrenheit and {:.2} Degrees Kelvin",
                    *celsius_amount as i32,
                    fahrenheit_result as i32,
                    kelvin_result,
                );
            } else if let Some(kelvin_amount) = matches.get_one::<f32>("K") {
                let fahrenheit_result = kelvin_to_fahrenheit(*kelvin_amount);
                let celsius_result = kelvin_to_celsius(*kelvin_amount);
                println!(
                    "##==>> {:.2} Degrees Kelvin is equal to {} Degrees Fahrenheit and {} Degrees Celsius",
                    kelvin_amount,
                    fahrenheit_result as i32,
                    celsius_result as i32,
                );
            } else if let Some(pounds_kilos_conversion_amount) = matches.get_one::<f32>("LB") {
                let result = pounds_to_kilos(*pounds_kilos_conversion_amount);
                println!(
                    "##==>> {}lbs is equal to {}kgs",
                    pounds_kilos_conversion_amount,
                    result as i32,
                );
            } else if let Some(kilos_pounds_conversion_amount) = matches.get_one::<f32>("KG") {
                let result = kilos_to_pounds(*kilos_pounds_conversion_amount);
                println!(
                    "##==>> {}kgs is equal to {}lbs",
                    kilos_pounds_conversion_amount,
                    result as i32,
                );
            } else if let Some(mph_kph_conversion_amount) = matches.get_one::<f32>("MPH") {
                let result = mph_to_kph(*mph_kph_conversion_amount);
                println!(
                    "##==>> {} MPH is equal to {} KPH",
                    mph_kph_conversion_amount,
                    result as i32,
                );
            } else if let Some(kph_mph_conversion_amount) = matches.get_one::<f32>("KPH") {
                let result = kph_to_mph(*kph_mph_conversion_amount);
                println!(
                    "##==>> {} KPH is equal to {} MPH",
                    kph_mph_conversion_amount,
                    result as i32,
                );
            }
        },
        Some(("parse", matches)) => {
            if let Some(passed_file) = matches.get_one::<String>("file") {
                let passed_path = PathBuf::from(passed_file);
                if passed_path.exists() {
                    if let Err(error) = parse_handle_file(passed_path) {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    }
                } else {
                    println!("##==> '{}' is not a valid path.", passed_file);
                }
            } else {
                println!("##==> A file must be passed to the program. Try running 'dave parse --help' for more information");
            }
        },
        Some(("scrape", matches)) => {
            if matches.get_flag("weather") {
                if let Err(error) = weather_scraper() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("top100") {
                if let Err(error) = imdb_top100_scraper() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if matches.get_flag("dcs") {
                if let Err(error) = dcs_news_scraper() {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
            if let Some(chosen_sport) = matches.get_one::<String>("sports") {
                if let Err(error) = scores_scraper(chosen_sport.to_string()) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            }
        },
        Some(("chip8", matches)) => {
            // Check if User Has Chosen a Different Pixel Style
            let mut pixel_choice = "█";
            if let Some(user_pixel) = matches.get_one::<String>("pixel") {
                if user_pixel.len() > 6 {
                    eprintln!(
                        "{}",
                        "##==>>>> ERROR: Pixel String Length Must Not Exceed 6 Characters. Using Default\n".red()
                    );
                } else if user_pixel.len() == 0 || user_pixel == " " {
                    eprintln!(
                        "{}",
                        "##==>>>> ERROR: Pixel String Length Cannot be Empty. Using Default\n".red()
                    );
                } else {
                    pixel_choice = user_pixel;
                }
            }

            // Get File and File Contents From User
            if let Some(passed_rom) = matches.get_one::<String>("filename") {
                let path = Path::new(passed_rom);
                if !path.exists() {
                    let bad_path_string = format!("##==>>>> ERROR: Invalid Path - '{}'", path.display());
                    eprintln!("{}", bad_path_string.red());
                } else if path.exists() && path.metadata().unwrap().is_file() {
                    let mut file = File::options()
                        .read(true)
                        .create(false)
                        .open(path)
                        .unwrap();
                    let mut file_contents: Vec<u8> = Vec::new();
                    if let Err(error) = file.read_to_end(&mut file_contents) {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    }

                    let mut chip_8 = Chip8::start(&file_contents[..]);

                    enable_raw_mode().unwrap();
                    let mut output = io::stdout();
                    execute!(output, EnterAlternateScreen, EnableMouseCapture).unwrap();

                    let crossterm = CrosstermBackend::new(output);
                    let mut terminal = Terminal::new(crossterm).unwrap();
                    if let Err(error) = run_dave_chip8_emulator(&mut terminal, &mut chip_8, pixel_choice.to_string()) {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    }

                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture,
                    ).unwrap();
                    disable_raw_mode().unwrap();

                    println!("{}", "!!! Thank you for using David's Chip8 Emulator !!!".yellow());
                }
            } else {
                println!("##==> A valid ROM file must be passed to the program. Try running 'dave chip8 --help' for more information");
            }
        },
        Some(("quiz", matches)) => {
            let mut total_questions = 5;
            if let Some(gotten_question_amount) = matches.get_one::<usize>("#") {
                if *gotten_question_amount < 16 {
                    total_questions = *gotten_question_amount;
                } else {
                    println!(
                        "##==> {} is not a valid number of questions. Defaulting to 5\n",
                        gotten_question_amount,
                    );
                }
            }
            let mut chosen_difficulty = "easy";
            if let Some(gotten_difficulty) = matches.get_one::<String>("difficulty") {
                match gotten_difficulty.as_str() {
                    "easy" => { chosen_difficulty = "easy" },
                    "medium" => { chosen_difficulty = "medium" },
                    "hard" => { chosen_difficulty = "hard" },
                    "god" => { chosen_difficulty = "god" },
                    _ => {
                        println!(
                        "##==> {} is not a valid difficulty. Defaulting to 'Easy'\n",
                            gotten_difficulty,
                        );
                    },
                };
            }
            if matches.get_flag("animals") {
                println!("{}", "^^^ David's Animal Quiz ^^^\n".green());
                let quiz_choice = "animals".to_string();
                if let Err(error) = dave_quiz(quiz_choice, total_questions, chosen_difficulty) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else if matches.get_flag("strek") {
                println!("{}", "*** David's Star Trek Quiz ***\n".yellow());
                let quiz_choice = "strek".to_string();
                if let Err(error) = dave_quiz(quiz_choice, total_questions, chosen_difficulty) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else if matches.get_flag("swars") {
                println!("{}", "### David's Star Wars Quiz ###\n".yellow());
                let quiz_choice = "swars".to_string();
                if let Err(error) = dave_quiz(quiz_choice, total_questions, chosen_difficulty) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else if matches.get_flag("mil-av") {
                println!("{}", "<<< David's Military Aviation Quiz >>>\n".yellow());
                let quiz_choice = "mil-av".to_string();
                if let Err(error) = dave_quiz(quiz_choice, total_questions, chosen_difficulty) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else if matches.get_flag("gen-av") {
                println!("{}", ">>> David's General Aviation Quiz <<<\n".yellow());
                let quiz_choice = "gen-av".to_string();
                if let Err(error) = dave_quiz(quiz_choice, total_questions, chosen_difficulty) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else {
                println!(
                    "##==> A valid quiz must be chosen. Try running 'dave quiz --help' for more information",
                );
            }
        },
        Some(("budget", matches)) => {
            let mut budget_file = match file_options.append(false).open(find_budget_file()) {
                Ok(budget_file) => budget_file,
                Err(error) => {
                    eprintln!(
                        "##==>>>> ERROR: {}: {}",
                        find_budget_file().display(),
                        error
                    );
                    std::process::exit(1)
                }
            };
            let budget_path = reader.budget_path();

            if matches.get_flag("new") {
                // Remove Budget File and Create New One
                // Until I Can Figure Out Extra Chars Problem
                fs::remove_file(budget_path.clone()).unwrap();
                let mut budget_file = match file_options.append(false).open(find_budget_file()) {
                    Ok(budget_file) => budget_file,
                    Err(error) => {
                        eprintln!(
                            "##==>>>> ERROR: {}: {}",
                            find_budget_file().display(),
                            error
                        );
                        std::process::exit(1)
                    }
                };
                // Create New Budget Object and Write to Budget File
                let budget = DaveBudget::new();
                if let Err(error) = write!(budget_file, "{}", serde_json::to_string(&budget).unwrap()) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    std::process::exit(1)
                }
                println!("##==>> New Budget Created!");
            }
            if let Some(income_amount) = matches.get_one::<f64>("income") {
                // Get JSON String From Budget File
                let budget_file_string: String = fs::read_to_string(budget_path.clone()).unwrap().parse().unwrap();
                // Have Serde Deserialize It Into Budget Object
                let mut budget:DaveBudget = match serde_json::from_str(&budget_file_string) {
                    Ok(budget) => budget,
                    Err(error) => {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        std::process::exit(1)
                    },
                };
                // Add Income and Update by Writing to Budget File
                budget.add_income(*income_amount);
                if let Err(error) = write!(budget_file, "{}", serde_json::to_string(&budget).unwrap()) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    std::process::exit(1)
                }
                println!("##==>> Budget Updated!");
            }
            if let Some(mut values) = matches.get_many::<String>("expense") {
                // Get JSON String From Budget File
                let budget_file_string: String = fs::read_to_string(budget_path.clone()).unwrap().parse().unwrap();
                // Have Serde Deserialize It Into Budget Object
                let mut budget:DaveBudget = match serde_json::from_str(&budget_file_string) {
                    Ok(budget) => budget,
                    Err(error) => {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        std::process::exit(1)
                    },
                };
                let tag = match values.next() {
                    Some(tag) => tag,
                    None => { std::process::exit(1) },
                };
                let amount = match values.next() {
                    Some(amount) => amount,
                    None => { std::process::exit(1) },
                };
                println!("Expense: {}, Amount: {}", tag, amount);
                budget.add_expense(String::from(tag), amount.parse::<f64>().unwrap());
                if let Err(error) = write!(budget_file, "{}", serde_json::to_string(&budget).unwrap()) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    std::process::exit(1)
                }
                println!("##==>> Budget Updated!");
            }
            if matches.get_flag("summary") {
                // Get JSON String From Budget File
                let budget_file_string: String = fs::read_to_string(budget_path).unwrap().parse().unwrap();
                // Have Serde Deserialize It Into Budget Object
                let budget:DaveBudget = match serde_json::from_str(&budget_file_string) {
                    Ok(budget) => budget,
                    Err(error) => {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        std::process::exit(1)
                    },
                };
                // Print Budget Information to Screen From Gotten
                // Budget Object
                println!("##==>> Amount of Income: ${}", budget.income);
                println!("##==>> Current Budget: ${}", budget.get_balance());
                if budget.expenses.len() > 0 {
                    for (expense, amount) in budget.expenses {
                        println!("##==>> Expense: {} - ${}", expense, amount);
                    }
                }
            }
        },
        Some(("dave-land", _matches)) => {
            if let Err(error) = dave_game_loop() {
                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
            }
        },
        Some(("currency", matches)) => {
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
                        println!("##==> A currency to convert your amount to must be passed to the program. Try running 'dave currency --help' for more information");
                    }
                } else {
                    println!("##==> A type of currency for your amount must be passed to the program. Try running 'dave currency --help' for more information");
                }
            } else {
                println!("##==> An amount must be passed to the program. Try running 'dave currency --help' for more information");
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
                    println!("##==> An amount of reps completed must be passed to the program. Try running 'dave drm --help' for more information");
                }
            } else {
                println!("##==> A amount of weight lifted must be passed to the program. Try running 'dave drm --help' for more information");
            }
        },
        Some(("guess", matches)) => {
            if let Some(passed_value) = matches.get_one::<u16>("number") {
                if let Err(error) = guess_number(*passed_value) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else {
                println!("##==> A guess must be passed to the program. Try running 'dave guess --help' for more information");
            }
        },
        Some(("size", matches)) => {
            if let Some(passed_directory) = matches.get_one::<String>("filename") {
                let path = Path::new(passed_directory);
                if let Err(error) = get_file_size(path) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
            } else {
                println!("##==> A file or path must be passed to the program. Try running 'dave size --help' for more information");
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
                println!("##==> A file or path must be passed to the program. Try running 'dave crypt --help' for more information");
            }
        },
        Some(("note", matches)) => {
            // Create or Get Database
            let mut db: sled::Db = match sled::open("./dave_conf/var/daves_notes") {
                Ok(db) => db,
                Err(error) => {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                    std::process::exit(1)
                },
            };

            // Clear Database and Return
            if matches.get_flag("overwrite") {
                match db.clear() {
                    Ok(_) => {
                        println!("##==> Database Overwritten Successfully");
                        return
                    },
                    Err(error) => {
                        eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        std::process::exit(1)
                    },
                }
            }
            if matches.get_flag("list") {
                println!("##==> Current Notes:");
                // Iterate Over Database 
                // FIXME: so ID #'s are in Order
                // Use sort with predicate
                let iter_db = db.iter().values().rev();
                for member in iter_db {
                    if let Ok(ref value) = member {
                        let dave_note: DaveNote = bincode::deserialize(&value).unwrap();
                        let status = if dave_note.completed { "[X]" } else { "[ ]" };
                        println!("{} {} - {}", status, dave_note.id, dave_note.title);
                    }
                }
            }
            if let Some(note_label) = matches.get_one::<String>("add") {
                println!("##==> Adding Note ...");
                // Create New Note
                let mut dave_note = DaveNote::new();
                // Set Known Values for New Note
                dave_note.title = note_label.to_string();
                dave_note.completed = false;

                // Determine ID number by counting members of database
                // and adding 1
                let count = db.len();
                dave_note.id = count as u64 + 1;

                // Create Key Value by Hashing Label String
                let mut hasher = sha2::Sha256::new();
                hasher.update(&note_label);
                let hash_value = hasher.finalize().to_vec();

                // Update the Database
                if let Err(error) = DaveDatabase::update(&mut db, dave_note, &hash_value) {
                    eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                }
                println!("##==> Note Added Successfully");
            }
            if let Some(note_id) = matches.get_one::<u64>("complete") {
                // Iterate Over Members of Database and Find Matching ID Number
                let iter_db = db.iter().values();
                for member in iter_db {
                    if let Ok(ref value) = member {
                        // Deserialize Member Into DaveNote and Check ID #
                        let mut dave_note: DaveNote = bincode::deserialize(&value).unwrap();
                        // If ID #'s Equal, Correct Note Found
                        if dave_note.id == *note_id {
                            println!("##==> Found Note with ID #{} - {}!", note_id, dave_note.title);
                            if dave_note.completed {
                                println!("##==> Warning! You already did that. You're senile");
                                std::process::exit(0)
                            }
                            // Set Note Completion to True
                            dave_note.completed = true;

                            // Create New Key for Updated Note by Hashing Label String
                            let mut hasher = sha2::Sha256::new();
                            hasher.update(&dave_note.title);
                            let hash_value = hasher.finalize().to_vec();

                            // Update Database with Modified Entry
                            if let Err(error) = DaveDatabase::update(&mut db, dave_note, &hash_value) {
                                eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                            }
                            println!("##==> Note Updated Successfully");
                        }
                    }
                }
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
                println!("##==> A file or path must be passed to the program. Try running 'dave hash --help' for more information");
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
                            process::exit(1)
                        });
                        if let Err(error) = dave_grep::run(config) {
                            eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
                        }
                    } else {
                        eprintln!("{}'{}'", "##==>>>> ERROR: File Not Found: ".red(), filename);
                    }
                } else {
                    println!("##==> A file or path must be passed to DGREP. Try running 'dave dgrep --help' for more information");
                }
            } else {
                println!("##==> A match pattern must be passed to DGREP. Try running 'dave dgrep --help' for more information");
            }
        },
        _ => { println!("##==> Try running the program with 'dave --help' to see a list of possible commands and options") },
    }

    let time = start.elapsed();
    println!(
        "\n##==> Dave Ran For {}.{}s",
        time.as_secs(),
        time.subsec_millis(),
    )
}
