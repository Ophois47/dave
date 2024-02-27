use std::error::Error;
use std::fs;
use regex::Regex;
use colored::*;

pub struct Config {
	pub case_sensitive: bool,
	pub regex: bool,
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
	    if gotten_option == "i" || gotten_option == "I" || gotten_option == "insensitive" {
	        case_sensitive = true;
	    } else {
	        case_sensitive = false;
	    }
	    let regex: bool;
	    if gotten_option == "r" || gotten_option == "R" || gotten_option == "regex" {
	    	regex = true;
	    } else {
	    	regex = false;
	    }
		let pattern = gotten_pattern;
		let filename = gotten_filename;

		Ok(Config {
			case_sensitive,
			regex,
			pattern,
			filename,
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let filename = config.filename.clone();
	let contents = fs::read_to_string(config.filename)?;

	if !config.regex {
		let results = if config.case_sensitive {
			search(&config.pattern, &contents)
		} else {
			search_case_insensitive(&config.pattern, &contents)
		};
		if results.is_empty() {
			println!("{}", "##==> There Were No Matches For Your Pattern".red());
		} else {
			for line in results {
				println!("{}", line.yellow());
			}
		}
	} else {
		let regex = Regex::new(&config.pattern)?;
		if regex.is_match(&contents) {
			let matches: Vec<_> = regex.find_iter(&contents).map(|m| m).collect();
			println!(
				"##==>> Found REGEX Match For Pattern '{}' in Path '{}'\n",
				&config.pattern.yellow(),
				filename.yellow(),
			);
			for mat in matches {
				println!("{}", "MATCH FOUND:".yellow());
				println!("Match Start: {}", mat.start());
				println!("Match End: {}", mat.end());
				println!("Match Length: {}", mat.len());
				println!("Match Range: {:?}", mat.range());
				println!("Match Str: {}", mat.as_str());
				println!();
			}
		} else {
			println!("{}", "##==> There Were No REGEX Matches For Your Pattern".red());
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
