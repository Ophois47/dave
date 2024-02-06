use std::error::Error;
use std::fs;
use colored::*;

pub struct Config {
	pub pattern: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(dargs: Vec<String>) -> Result<Config, &'static str> {

		let case_sensitive: bool;
		let gotten_case_insensitivity = dargs[0].clone();
	    if gotten_case_insensitivity == "i" || gotten_case_insensitivity == "insensitive" {
	        case_sensitive = true;
	    } else {
	        case_sensitive = false;
	    }
		let pattern = dargs[1].clone();
		let filename = dargs[2].clone();

		Ok(Config {
			pattern,
			filename,
			case_sensitive,
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
		search(&config.pattern, &contents)
	} else {
		search_case_insensitive(&config.pattern, &contents)
	};

	for line in results {
		if line == "" || line == " " {
			println!("{}", "##==>> No Matches Were Found For Your Pattern".red());
		}
		println!("{}", line);
	}

	Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.contains(pattern))
		.collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
	let pattern = pattern.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&pattern) {
			results.push(line);
		}
	}

	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let pattern = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
	}

	#[test]
	fn case_insensitive() {
		let pattern = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(pattern, contents));
	}
}