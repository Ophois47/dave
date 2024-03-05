use std::io;

fn brzycki_formula(weight: u16, reps: u16) -> u16 {
	// weight / (1.0278 - 0.0278 * reps)
	let rep_max = weight as f32 / (1.0278 - 0.0278 * reps as f32);
	rep_max as u16
}

fn epley_formula(weight: u16, reps: u16) -> u16 {
	// weight * (1 + 0.0333 * reps)
	let rep_max = weight as f32 * (1 as f32 + 0.0333 * reps as f32);
	rep_max as u16
}

fn lander_formula(weight: u16, reps: u16) -> u16 {
	// (100 * weight) / (101.3 - 2.67123 * reps)
	let rep_max = (100 as f32 * weight as f32) / (101.3 - 2.67123 * reps as f32);
	rep_max as u16
}

fn lombardi_formula(weight: u16, reps: u16) -> u16 {
	// weight * reps ^ 0.1
	let rep_max = weight as f64 * (reps as f64).powf(0.1);
	rep_max as u16
}

fn convert_lb_to_kg(weight: u16) -> u16 {
	let kg_weight = weight as f32/2.2046;
	kg_weight as u16
}

pub fn dave_rep_max_calc(weight: u16, reps: u16, uom: &str) -> io::Result<()> {
	let unit_of_measurement: &str = uom;
	println!("##==> If you are able to lift {} {} for {} reps ...", weight, unit_of_measurement, reps);
	println!();

	if unit_of_measurement == "lbs" {
		let bform = brzycki_formula(weight, reps);
		println!("##==>> Brzycki Formula: {} {}", bform, unit_of_measurement);
		let eform = epley_formula(weight, reps);
		println!("##==>> Epley Formula: {} {}", eform, unit_of_measurement);
		let laform = lander_formula(weight, reps);
		println!("##==>> Lander Formula: {} {}", laform, unit_of_measurement);
		let loform = lombardi_formula(weight, reps);
		println!("##==>> Lombardi Formula: {} {}", loform, unit_of_measurement);
		println!();

		let average_max = (bform + eform + laform + loform)/4;
		println!("##==>> Your 1RM: {} {}", average_max, unit_of_measurement);
		let ninetyfive_percent = 0.95 * weight as f32;
		println!("##==>> Your 95% Max: {} {}", ninetyfive_percent as u16, unit_of_measurement);
		let ninety_percent = 0.90 * weight as f32;
		println!("##==>> Your 90% Max: {} {}", ninety_percent as u16, unit_of_measurement);
		let eightyfive_percent = 0.85 * weight as f32;
		println!("##==>> Your 85% Max: {} {}", eightyfive_percent as u16, unit_of_measurement);
		let eighty_percent = 0.80 * weight as f32;
		println!("##==>> Your 80% Max: {} {}", eighty_percent as u16, unit_of_measurement);
		let seventyfive_percent = 0.75 * weight as f32;
		println!("##==>> Your 75% Max: {} {}", seventyfive_percent as u16, unit_of_measurement);
		let seventy_percent = 0.70 * weight as f32;
		println!("##==>> Your 70% Max: {} {}", seventy_percent as u16, unit_of_measurement);
		let sixty_percent = 0.60 * weight as f32;
		println!("##==>> Your 60% Max: {} {}", sixty_percent as u16, unit_of_measurement);
		let fifty_percent = 0.50 * weight as f32;
		println!("##==>> Your 50% Max: {} {}", fifty_percent as u16, unit_of_measurement);
	} else {
		let bform = brzycki_formula(weight, reps);
		println!("##==>> Brzycki Formula: {} {}", convert_lb_to_kg(bform), unit_of_measurement);
		let eform = epley_formula(weight, reps);
		println!("##==>> Epley Formula: {} {}", convert_lb_to_kg(eform), unit_of_measurement);
		let laform = lander_formula(weight, reps);
		println!("##==>> Lander Formula: {} {}", convert_lb_to_kg(laform), unit_of_measurement);
		let loform = lombardi_formula(weight, reps);
		println!("##==>> Lombardi Formula: {} {}", convert_lb_to_kg(loform), unit_of_measurement);
		println!();

		let average_max = (bform + eform + laform + loform)/4;
		println!("##==>> Your 1RM: {} {}", convert_lb_to_kg(average_max), unit_of_measurement);
		let ninetyfive_percent = 0.95 * weight as f32;
		println!("##==>> Your 95% Max: {} {}", convert_lb_to_kg(ninetyfive_percent as u16), unit_of_measurement);
		let ninety_percent = 0.90 * weight as f32;
		println!("##==>> Your 90% Max: {} {}", convert_lb_to_kg(ninety_percent as u16), unit_of_measurement);
		let eightyfive_percent = 0.85 * weight as f32;
		println!("##==>> Your 85% Max: {} {}", convert_lb_to_kg(eightyfive_percent as u16), unit_of_measurement);
		let eighty_percent = 0.80 * weight as f32;
		println!("##==>> Your 80% Max: {} {}", convert_lb_to_kg(eighty_percent as u16), unit_of_measurement);
		let seventyfive_percent = 0.75 * weight as f32;
		println!("##==>> Your 75% Max: {} {}", convert_lb_to_kg(seventyfive_percent as u16), unit_of_measurement);
		let seventy_percent = 0.70 * weight as f32;
		println!("##==>> Your 70% Max: {} {}", convert_lb_to_kg(seventy_percent as u16), unit_of_measurement);
		let sixty_percent = 0.60 * weight as f32;
		println!("##==>> Your 60% Max: {} {}", convert_lb_to_kg(sixty_percent as u16), unit_of_measurement);
		let fifty_percent = 0.50 * weight as f32;
		println!("##==>> Your 50% Max: {} {}", convert_lb_to_kg(fifty_percent as u16), unit_of_measurement);
	}

	Ok(())
}
