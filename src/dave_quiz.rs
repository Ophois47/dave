use colored::*;
use std::fs;
use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn test_question(question: &str, answer: &str, timeout: u32) -> Option<bool> {
	print!("{} ", question);
	// Flush Question to Display
	io::stdout().flush().expect("Failed to flush buffer");

	// Setup Transmitter and Receiver to use Between Threads
	let (transmitter, receiver) = mpsc::channel();

	// Spawn Thread With User Input Code
	thread::spawn(move || {
		// Read User Input Into Buffer
		let mut buffer = String::new();
		io::stdin()
			.read_line(&mut buffer)
			.expect("Failed to read user input");
		let buffer = buffer.trim().to_string();

		// Get Buffer Out of Thread by Sending it Into Transmitter
		transmitter.send(buffer).expect("Failed to send user input");
	});

	receiver
		.recv_timeout(Duration::new(timeout as u64, 0))
			.or_else(|o| {
				// If Error, Print This and Re-Wrap Error
				println!("{}", "\nYou ran out of time!".red());
				Err(o)
			})
		.ok() // 'ok' Changes 'Result<A,B>' into 'Option<A>'
		.map(|buffer| buffer == answer) // Use Mapping Function to Transform Option<String> to Option<bool>

	// Function Will Return 1 of 3 Things
	// Some(true) if Answer Correct
	// Some(false) if Answer Wrong
	// None if Timeout Triggered
}

pub fn dave_quiz(quiz_choice: String) -> io::Result<()> {
	let quiz_tsv_filename: &str;
	if quiz_choice == "animals" {
		quiz_tsv_filename = "./dave_conf/var/daves_quiz/animal_quiz.tsv";
	} else if quiz_choice == "strek" {
		quiz_tsv_filename = "./dave_conf/var/daves_quiz/strek_quiz.tsv";
	} else if quiz_choice == "swars" {
		quiz_tsv_filename = "./dave_conf/var/daves_quiz/swars_quiz.tsv";
	} else {
		std::process::exit(1);
	}

	// Time Given For Each Question
	let timeout = 40;

	// Open and Read Contents From Quiz File
	let mut quiz_file = fs::File::open(quiz_tsv_filename).unwrap();
	let mut buffer = String::new();
	quiz_file.read_to_string(&mut buffer).unwrap();

	// Count Number of Questions
	let total_questions = buffer.lines().count();

	let score = buffer.lines()
		.map(|line| {
			let mut q_a = line.split('\t').map(|s| s.to_string());
			let question = q_a.next().expect("No Question Found");
			let answer = q_a.next().expect("No Answer Found");
			(question, answer)
		})
		.map(|(question, answer)| test_question(&question, &answer, timeout))
		.take_while(|o| o.is_some())
		.map(|o| o.unwrap())
		.filter(|p| *p)
		.count();

	println!("Score: {} / {}", score, total_questions);
	if score == total_questions {
		println!("{}", "!!! You Are Master Champion !!!".yellow());
	}
	Ok(())
}
