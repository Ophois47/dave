use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

struct DaveEd {}

impl DaveEd {
	fn default() -> Self {
		DaveEd {}
	}

	fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		println!("************************************************************************");
		println!("*   Welcome to DaveEd! Dave's Very Own Text Editor (Hit 'Q' to Quit)   *");
		println!("************************************************************************\n");

		enable_raw_mode()?;
		for b in io::stdin().bytes() {
			match b {
				Ok(b) => {
					let c = b as char;
					if c.is_control() {
						println!("Binary: {0:08b} ASCII: {0:#03}\r", b);
					} else {
						println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
					}
					if c == 'q' || c == 'Q' {
						break;
					}
				},
				Err(error) => eprintln!("##==>>>> ERROR: {}", error),
			}
		}
		disable_raw_mode()?;
		Ok(())
	}
}

pub fn dave_ed_main() -> io::Result<()> {
	let editor = DaveEd::default();
	if let Err(error) = editor.run() {
		eprintln!("##==>>>> ERROR: {}", error);
	}
	Ok(())
}
