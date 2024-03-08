use iso_currency::Currency;
use std::io;

pub fn dave_currency_conv(
	amount: u16,
	country_code: &str,
	conversion: &str,
) -> io::Result<()> {
	match country_code {
		"USD" => {
			println!("##==> {} {}", amount, Currency::USD.name());
			println!("##==> {}{}", Currency::USD.symbol(), amount);
			println!("{} Used By: {:?}", Currency::USD.name(), Currency::USD.used_by());
			println!("Converting to {} ...", conversion);
		},
		"GBP" => {
			println!("##==> {} {}", amount, Currency::GBP.name());
			println!("##==> {}{}", Currency::GBP.symbol(), amount);
			println!("{} Used By: {:?}", Currency::USD.name(), Currency::USD.used_by());
			println!("Converting to {} ...", conversion);
		},
		"EUR" => {
			println!("##==> {} {}", amount, Currency::EUR.name());
			println!("##==> {}{}", Currency::EUR.symbol(), amount);
			println!("{} Used By: {:?}", Currency::USD.name(), Currency::USD.used_by());
			println!("Converting to {} ...", conversion);
		},
		_ => {
			println!("Idk wtf that is ...");
		},
	}

	Ok(())
}
