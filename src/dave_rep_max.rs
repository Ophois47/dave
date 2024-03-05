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

pub fn dave_rep_max_calc(weight: u16, reps: u16) -> io::Result<()> {
	println!("##==>>> Passed Weight: {}", weight);
	println!("##==>>> Passed Reps: {}", reps);
	println!();
	
	let bform = brzycki_formula(weight, reps);
	println!("##==>> Brzycki Formula: {}", bform);
	let eform = epley_formula(weight, reps);
	println!("##==>> Epley Formula: {}", eform);
	let laform = lander_formula(weight, reps);
	println!("##==>> Lander Formula: {}", laform);
	let loform = lombardi_formula(weight, reps);
	println!("##==>> Lombardi Formula: {}", loform);
	println!();

	let average_max = (bform + eform + laform + loform)/4;
	println!("##==>> Your 1RM: {}", average_max);
	let ninetyfive_percent = 0.95 * weight as f32;
	println!("##==>> Your 95% Max: {}", ninetyfive_percent as u16);
	let ninety_percent = 0.90 * weight as f32;
	println!("##==>> Your 90% Max: {}", ninety_percent as u16);
	let eightyfive_percent = 0.85 * weight as f32;
	println!("##==>> Your 85% Max: {}", eightyfive_percent as u16);
	let eighty_percent = 0.80 * weight as f32;
	println!("##==>> Your 80% Max: {}", eighty_percent as u16);
	let seventyfive_percent = 0.75 * weight as f32;
	println!("##==>> Your 75% Max: {}", seventyfive_percent as u16);
	let seventy_percent = 0.70 * weight as f32;
	println!("##==>> Your 70% Max: {}", seventy_percent as u16);
	let sixty_percent = 0.60 * weight as f32;
	println!("##==>> Your 60% Max: {}", sixty_percent as u16);
	let fifty_percent = 0.50 * weight as f32;
	println!("##==>> Your 50% Max: {}", fifty_percent as u16);

	Ok(())
}
