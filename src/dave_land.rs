use colored::*;
use std::io;
use crate::dave_land_lib;

// Game File Location
const GAME_FILE_LOC: &str = "./dave_land_file.ron";

fn init_game(file_loc: &str) -> Result<dave_land_lib::World, std::io::Error> {
	// Read game file, return world
	dave_land_lib::World::read_from_file(file_loc)
}

fn dave_do_game(mut world: dave_land_lib::World) -> io::Result<()> {
	// Introduction and Setup
	let mut command: dave_land_lib::Command;
	let mut output: String;

	println!("{}", "************************************************************".blue());
	println!("{}", "* Welcome to LV-426, the Second Moon of Zeta 2 Reticuli IV *".blue());
	println!("{}", "************************************************************".blue());
	println!();
	println!("You awaken in what seems to be absolute darkness, suddenly punctuated by the occasional blast of light that appears to be coming from a currently malfunctioning video screen on a nearby wall.");
	println!("An alarm blares in the distance, getting louder by the moment as you adjust to having awoken so suddenly, until the alarm becomes almost deafening. Your mouth feels dry, and you have an intense headache.");
	println!("Feeling a sudden surge of adrenaline, questions about the nature of your recent consciousness and current location swirl around violently in your mind. It's time to have a look around ...");
	println!();

	// Main Loop
	loop {
		command = dave_land_lib::get_input();
		output = world.update_state(&command);
		dave_land_lib::update_screen(output);

		if matches!(command, dave_land_lib::Command::Quit) {
			break;
		}
	}

	// Shutdown, Cleanup and Exit
	println!("{}", "... In Space, No One Can Hear You Scream ...".bright_green());

	Ok(())
}

pub fn dave_game_loop() -> io::Result<()> {
	let world_res = init_game(GAME_FILE_LOC);

	match world_res {
		Ok(world) => {
			// Run Game
			if let Err(error) = dave_do_game(world) {
				eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
			}
		}
		Err(error) => {
			// Shutdown, Exit With Error
			eprintln!("{}{}", "##==>>>> ERROR: ".red(), error);
		}
	}

	Ok(())
}
