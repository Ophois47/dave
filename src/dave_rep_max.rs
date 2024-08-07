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
	// weight * (reps ^ 0.1)
	let rep_max = weight as f64 * (reps as f64).powf(0.1);
	rep_max as u16
}

fn convert_lb_to_kg(weight: u16) -> u16 {
	let kg_weight = weight as f32/2.2046;
	kg_weight as u16
}

pub fn dave_rep_max_calc(
	mut weight: u16,
	reps: u16,
	unit_of_measurement: &str,
) -> io::Result<()> {
	println!(
		"##==> If you are able to lift {} {} for {} reps ...",
		weight,
		unit_of_measurement,
		reps,
	);
	println!();

	if unit_of_measurement == "kgs" {
		weight = convert_lb_to_kg(weight);
	}

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
	println!("##==>> 1RM: {} {}", average_max, unit_of_measurement);
	let ninetyfive_percent = 0.95 * average_max as f32;
	println!("##==>> 95% Max: {} {} (1-3 reps)", ninetyfive_percent as u16, unit_of_measurement);
	let ninety_percent = 0.90 * average_max as f32;
	println!("##==>> 90% Max: {} {} (4 reps)", ninety_percent as u16, unit_of_measurement);
	let eightyfive_percent = 0.85 * average_max as f32;
	println!("##==>> 85% Max: {} {} (5 reps)", eightyfive_percent as u16, unit_of_measurement);
	let eighty_percent = 0.80 * average_max as f32;
	println!("##==>> 80% Max: {} {} (6-8 reps)", eighty_percent as u16, unit_of_measurement);
	let seventyfive_percent = 0.75 * average_max as f32;
	println!("##==>> 75% Max: {} {} (10 reps)", seventyfive_percent as u16, unit_of_measurement);
	let seventy_percent = 0.70 * average_max as f32;
	println!("##==>> 70% Max: {} {} (12-15 reps)", seventy_percent as u16, unit_of_measurement);
	let sixty_percent = 0.60 * average_max as f32;
	println!("##==>> 60% Max: {} {} (15-20 reps)", sixty_percent as u16, unit_of_measurement);
	let fifty_percent = 0.50 * average_max as f32;
	println!("##==>> 50% Max: {} {} (20+ reps)", fifty_percent as u16, unit_of_measurement);
	println!();

	Ok(())
}
