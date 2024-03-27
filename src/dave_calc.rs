use colored::*;
use std::io;

pub fn dave_calc_loop() -> io::Result<()> {
	loop {
		println!("{}", "-- Dave's Calculator --".cyan());
		println!("{}", "=======================".cyan());
		println!("#=> 1. Addition");
		println!("#=> 2. Subtraction");
		println!("#=> 3. Multiplication");
		println!("#=> 4. Division");
		println!("#=> 5. Exit");
		println!("{}", "-----------------------".cyan());
		println!("#=> Input: ");

		let mut choice = String::new();
		io::stdin().read_line(&mut choice)?;
		let choice: u32 = match choice.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				eprintln!("{}", "##==>>> ERROR: Invalid Input. Please Enter a Valid Choice\n".red());
				continue
			},
		};

		if choice > 5 || choice < 1 {
			eprintln!("{}", "##==>>> ERROR: Invalid Choice. Please Enter a Value Between 1 and 5\n".red());
			continue
		} else if choice == 5 {
			println!("##==> Exiting ...");
			break
		}

		println!("##==> Enter the first value: ");
		let mut value1 = String::new();
		io::stdin().read_line(&mut value1)?;
		let value1: f64 = match value1.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				eprintln!("{}", "##==>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
				continue
			}
		};

		println!("##==> Enter the second value: ");
		let mut value2 = String::new();
		io::stdin().read_line(&mut value2)?;
		let value2: f64 = match value2.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				eprintln!("{}", "##==>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
				continue
			}
		};

		match choice {
			1 => {
				let result = value1 + value2;
				let result_string = format!("##==>> {} + {} = {}\n", value1, value2, result);
				println!("{}", result_string.yellow());
			},
			2 => {
				let result = value1 - value2;
				let result_string = format!("##==>> {} - {} = {}\n", value1, value2, result);
				println!("{}", result_string.yellow());
			},
			3 => {
				let result = value1 * value2;
				let result_string = format!("##==>> {} x {} = {}\n", value1, value2, result);
				println!("{}", result_string.yellow());
			},
			4 => {
				if value2 != 0.0 {
					let result = value1 / value2;
					let result_string = format!("##==>> {} / {} = {}\n", value1, value2, result);
					println!("{}", result_string.yellow());
				} else {
					eprintln!("{}", "##==>>> Warning: Cannot Divide by 0\n".bright_yellow());
					continue
				}
			},
			_ => eprintln!("{}", "##==>>> ERROR: Invalid Choice. Please Enter a Value Between 1 and 5\n".red()),
		}
	}
	Ok(())
}
