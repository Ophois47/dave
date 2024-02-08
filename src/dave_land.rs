use std::io;
use crate::dave_land_lib;

pub fn dave_game_loop() -> io::Result<()> {
	// Introduction and Setup
	println!("*******************************************************");
	println!("* Welcome to Dave Land! A Place of Wonder and Whimsy. *");
	println!("*******************************************************");
	println!();
	println!("You awake in darkness, punctuated by the occasional blast of light that seems to be coming from your currently malfunctioning vid screen on the wall.");
	println!("An alarm blares in the distance, getting louder by the second as you adjust to having awoken so suddenly, until it becomes almost deafening.");
	println!();

	let mut command: dave_land_lib::Command;
	let mut world = dave_land_lib::World::new();
	let mut output: String;

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
	println!("Take Joy With You Everywhere!");

	Ok(())
}
