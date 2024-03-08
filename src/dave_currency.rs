use iso_currency::Currency;
use std::io;

pub struct ExchangeResult {
	original_currency: String,
	original_amount: f32,
	final_currency: String,
	final_amount: f32,
}

fn match_from_currency(from_currency: &str, to_currency: &str, num: &f32) -> f32 {
	match &*from_currency.trim() {
		"USD" => convert_from_usd(&to_currency, &num),
		"EUR" => convert_from_eur(&to_currency, &num),
		"GBP" => convert_from_gbp(&to_currency, &num),
		"CAD" => convert_from_cad(&to_currency, &num),
		"JPY" => convert_from_jpy(&to_currency, &num),
		_ => 0.0,
	}
}

fn convert_from_usd(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_usd_rate = 0.78343;
	let eur_to_usd_rate = 0.91623;
	let jpy_to_usd_rate = 148.184;
	let cad_to_usd_rate = 1.34905;

	let amount: f32 = match to_currency {
		"EUR" => *num * eur_to_usd_rate,
		"GBP" => *num * gbp_to_usd_rate,
		"CAD" => *num * cad_to_usd_rate,
		"JPY" => *num * jpy_to_usd_rate,
		"USD" => *num,
		_ => 0.0,
	};

	amount
}

fn convert_from_jpy(to_currency: &str, num: &f32) -> f32 {
	let gbp_to_jpy_rate = 0.00529;
	let eur_to_jpy_rate = 0.00618;
	let cad_to_jpy_rate = 0.0091;
	let usd_to_jpy_rate = 0.00675;

	let amount: f32 = match to_currency {
		"USD" => *num * usd_to_jpy_rate,
		"EUR" => *num * eur_to_jpy_rate,
		"GBP" => *num * gbp_to_jpy_rate,
		"CAD" => *num * cad_to_jpy_rate,
		"JPY" => *num,
		_ => 0.0,
	};

	amount
}

fn convert_from_eur(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_eur_rate = 0.85499;
    let usd_to_eur_rate = 1.09128;
    let jpy_to_eur_rate = 161.719;
    let cad_to_eur_rate = 1.47219;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_eur_rate,
        "GBP" => *num * gbp_to_eur_rate,
        "CAD" => *num * cad_to_eur_rate,
        "JPY" => *num * jpy_to_eur_rate,
        "EUR" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_gbp(to_currency: &str, num: &f32) -> f32 {
    let jpy_to_gbp_rate = 189.129;
    let eur_to_gbp_rate = 1.16941;
    let cad_to_gbp_rate = 1.72174;
    let usd_to_gbp_rate = 1.27626;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_gbp_rate,
        "EUR" => *num * eur_to_gbp_rate,
        "CAD" => *num * cad_to_gbp_rate,
        "JPY" => *num * jpy_to_gbp_rate,
        "GBP" => *num,
        _ => 0.0,
    };

    amount
}

fn convert_from_cad(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_cad_rate = 0.58065;
    let eur_to_cad_rate = 0.67907;
    let jpy_to_cad_rate = 109.828;
    let usd_to_cad_rate = 0.74116;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_cad_rate,
        "EUR" => *num * eur_to_cad_rate,
        "GBP" => *num * gbp_to_cad_rate,
        "JPY" => *num * jpy_to_cad_rate,
        "CAD" => *num,
        _ => 0.0,
    };

    amount
}

pub fn dave_currency_conv(
	amount: f32,
	from_currency: &str,
	to_currency: &str,
) -> io::Result<()> {
	match from_currency {
		"USD" => {
			match Currency::from_code("USD") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"GBP" => {
			match Currency::from_code("GBP") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"EUR" => {
			match Currency::from_code("EUR") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					print!("##==> Nations that use the {}: ", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"JPY" => {
			match Currency::from_code("JPY") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		"CAD" => {
			match Currency::from_code("CAD") {
				Some(currency_code) => {
					println!("##==> {} {}", amount, currency_code.name());
					println!("##==> {}{}", currency_code.symbol(), amount);
					println!("##==> The {} is in use by:", currency_code.name());
					currency_code.used_by().iter().fold(true, |first, nation| {
						if !first { print!(", "); }
						print!("{}", nation.name());
						false
					});
					println!("\n##==> Converting to {} ...", to_currency);
				},
				_ => {},
			}
		},
		_ => {
			eprintln!("##==>>>> ERROR: Unknown Currency Type");
		},
	}

	let result = match_from_currency(from_currency, to_currency, &amount);
	let exchange_result = ExchangeResult {
        original_currency: String::from(&from_currency.trim().to_uppercase()),
        original_amount:  amount,
        final_currency: String::from(&to_currency.trim().to_uppercase()),
        final_amount: result
    };

	println!(
		"{} {} was converted to {} {}",
		exchange_result.original_currency,
		exchange_result.original_amount,
		exchange_result.final_currency,
		exchange_result.final_amount,
	);
	
	Ok(())
}
