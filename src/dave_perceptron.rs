use std::io;

fn activate_sigmoid(slope: f64, x: f64) -> f64 {
	let e: f64 = std::f64::consts::E;
	return 1.0/(1.0 + e.powf(-0.5 * slope * x));
}

pub fn daves_perceptron() -> io::Result<()> {
	// Define Data
	let data: Vec<[f64; 3]> = vec![
		[1.0, 1.0, 1.0],
		[0.0, 0.0, 1.0],
		[1.0, 0.0, 0.0],
		[0.0, 0.0, 0.0],
		[1.0, 1.0, 1.0]
	];

	let bias: f64 = 1.0;
	let weights: [f64; 4] = [0.6, 0.2, 0.4, 0.5];

	for entry in data.iter() {
		let mut sum: f64 = 0.0;
		let test: [f64; 4] = weights;

		for (index, value) in entry.iter().enumerate() {
			let result: f64;
			if index == 0 {
				result = test[index] * bias;
			} else {
				result = test[index] * value;
			}

			sum += result;
		}

		println!("##==>> Weighted Sum: {}", sum);
		let result: f64 = activate_sigmoid(1.0, sum);
		println!("##==>> Activation Function: {}", result);
	}

	// Bias
	println!("##==> Bias: {}", bias);

	// Weights
	println!("##==> Weights:");
	let mut weight_num = 1;
	for weight in weights.iter() {
		println!("#=> Weight #{}: {}", weight_num, weight);
		weight_num += 1;
	}

	Ok(())
}
