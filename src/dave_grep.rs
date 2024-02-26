use std::error::Error;
use std::fs;
use colored::*;

pub struct Config {
	pub case_sensitive: bool,
	pub pattern: String,
	pub filename: String,
}

impl Config {
	pub fn new(
		gotten_option: String,
		gotten_pattern: String,
		gotten_filename: String
	) -> Result<Config, &'static str> {
		let case_sensitive: bool;
		let gotten_case_insensitivity = gotten_option;
	    if gotten_case_insensitivity == "i" || gotten_case_insensitivity == "insensitive" {
	        case_sensitive = true;
	    } else {
	        case_sensitive = false;
	    }
		let pattern = gotten_pattern;
		let filename = gotten_filename;

		Ok(Config {
			case_sensitive,
			pattern,
			filename,
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

	if results.is_empty() {
		println!("{}", "##==> There Were No Matches to Your Pattern".red());
	} else {
		for line in results {
			println!("{}", line.green());
		}
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
