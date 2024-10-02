use colored::*;
use std::io;

// Wheel Horsepower Calculator
// HP = WHP * 1 / (1 – (DL - 1))
fn whp_to_hp(whp: u16, dtlf: f32) -> io::Result<u16> {
	let hp = whp as f32 * 1.0 / (1.0 - (dtlf - 1.0));
	Ok(hp as u16)
}

// WHP = HP / DL
fn hp_to_whp(hp: u16, dtlf: f32) -> io::Result<u16> {
	let whp: f32 = hp as f32 /  dtlf;
	Ok(whp as u16)
}

pub fn wheel_hp_calc(
	passed_horsepower: u16,
	conversion_type: &str,
	drive_train: &str,
) -> io::Result<()> {
	let drive_train_loss_factor: f32 = match drive_train {
		"RWD" => 1.15,
		"FWD" => 1.1,
		"AWD" => 1.2,
		_ => 0.0,
	};
	match conversion_type {
		"HP" => {
			println!("##==> Calculating WHP for {} HP ...", passed_horsepower);
			if let Ok(converted_value) = hp_to_whp(passed_horsepower, drive_train_loss_factor) {
				println!(
					"##==>> {} Engine HP is equal to {} WHP with a {} drivetrain",
					passed_horsepower,
					converted_value,
					drive_train,
				);
			}
		},
		"WHP" => {
			println!("##==> Calculating HP for {} WHP ...", passed_horsepower);
			if let Ok(converted_value) = whp_to_hp(passed_horsepower, drive_train_loss_factor) {
				println!(
					"##==>> {} WHP is equal to {} Engine HP with a {} drivetrain",
					passed_horsepower,
					converted_value,
					drive_train,
				);
			}
		},
		_ => println!("Idk what you're getting at weirdo."),
	};

	Ok(())
}

// Classic Calculator
pub fn dave_simple_calc_loop() -> io::Result<()> {
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
				eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Choice\n".red());
				continue
			},
		};

		if choice > 5 || choice < 1 {
			eprintln!("{}", "##==>>>> ERROR: Invalid Choice. Please Enter a Value Between 1 and 5\n".red());
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
				eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
				continue
			}
		};

		println!("##==> Enter the second value: ");
		let mut value2 = String::new();
		io::stdin().read_line(&mut value2)?;
		let value2: f64 = match value2.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
				continue
			}
		};

		match choice {
			1 => {
				let result = value1 + value2;
				let result_string = format!("##==>> Answer: {} + {} = {:.2}\n", value1, value2, result);
				println!("{}", result_string.yellow());
			},
			2 => {
				let result = value1 - value2;
				let result_string = format!("##==>> Answer: {} - {} = {:.2}\n", value1, value2, result);
				println!("{}", result_string.yellow());
			},
			3 => {
				let result = value1 * value2;
				let result_string = format!("##==>> Answer: {} x {} = {:.2}\n", value1, value2, result);
				println!("{}", result_string.yellow());
			},
			4 => {
				if value2 != 0.0 {
					let result = value1 / value2;
					let result_string = format!("##==>> Answer: {} / {} = {:.2}\n", value1, value2, result);
					println!("{}", result_string.yellow());
				} else {
					eprintln!("{}", "##==>>> Warning: Cannot Divide by 0\n".bright_yellow());
					continue
				}
			},
			_ => eprintln!("{}", "##==>>>> ERROR: Invalid Choice. Please Enter a Value Between 1 and 5\n".red()),
		}
	}
	Ok(())
}

pub fn dave_income_calc_loop() -> io::Result<()> {
	println!("{}", "-- Dave's Income Calculator --".cyan());
	println!("{}", "==============================".cyan());

	println!("##==> Enter your income: ");
	let mut income = String::new();
	io::stdin().read_line(&mut income)?;
	let income: f64 = match income.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
			return Ok(())
		}
	};

	println!("##==> Enter your expenses: ");
	let mut expenses = String::new();
	io::stdin().read_line(&mut expenses)?;
	let expenses: f64 = match expenses.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
			return Ok(())
		}
	};
	
	let result = income - expenses;
	let result_string = format!(
		"##==>> Income: ${:.2} - Expenses: ${:.2} = ${:.2}\n",
		income,
		expenses,
		result,
	);
	println!("{}", result_string.yellow());
	Ok(())
}

pub fn dave_interest_calc_loop() -> io::Result<()> {
	println!("{}", "-- Dave's Interest Calculator --".cyan());
	println!("{}", "================================".cyan());

	println!("##==> Enter the amount of money earning interest: ");
	let mut principal = String::new();
	io::stdin().read_line(&mut principal)?;
	let principal: f64 = match principal.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Amount\n".red());
			return Ok(())
		}
	};

	println!("##==> Enter the interest rate (%): ");
	let mut rate = String::new();
	io::stdin().read_line(&mut rate)?;
	let rate: f64 = match rate.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
			return Ok(())
		}
	};

	println!("##==> Enter the interest period (years): ");
	let mut time = String::new();
	io::stdin().read_line(&mut time)?;
	let time: f64 = match time.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("{}", "##==>>>> ERROR: Invalid Input. Please Enter a Valid Number\n".red());
			return Ok(())
		}
	};
	
	let total_accrued = principal * (1.0 + ((rate/100.0) * time));
	let interest = total_accrued - principal;
	let accrued_string = format!("##==>> Total Accrued Amount: ${:.2}", total_accrued);
	let interest_string = format!("##==>> Interest Paid: ${:.2}", interest);
	println!("{}", accrued_string.yellow());
	println!("{}", interest_string.yellow());
	Ok(())
}
