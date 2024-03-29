use colored::*;
use rand::seq::SliceRandom;
use std::fs;
use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn test_question(question: &str, answer: &str, timeout: u32) -> Option<bool> {
	print!("Q: {}\nA: ", question);
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

	let result = receiver
		.recv_timeout(Duration::new(timeout as u64, 0))
			.or_else(|o| {
				// If Error, Print This and Re-Wrap Error
				Err(o)
			})
		.ok() // 'ok' Changes 'Result<A,B>' into 'Option<A>'
		.map(|buffer| buffer == answer); // Use Mapping Function to Transform Option<String> to Option<bool>

	// Function Will Return 1 of 3 Things
	// Some(true) if Answer Correct
	// Some(false) if Answer Wrong
	// None if Timeout Triggered
	result
}

pub fn dave_quiz(quiz_choice: String, total_questions: usize, user_difficulty: &str) -> io::Result<()> {
	// Determine Which Quiz Was Chosen by User
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

	// Time Given For Each Question Based
	// on Chosen User Difficulty
	let mut timeout = 60;
	match user_difficulty {
		"easy" => { timeout = 60 },
		"medium" => { timeout = 30 },
		"hard" => { timeout = 10 },
		"god" => { timeout = 5 },
		_ => {},
	};

	// Open and Read Contents From Quiz File
	let mut quiz_file = fs::File::open(quiz_tsv_filename)?;
	let mut buffer = String::new();
	quiz_file.read_to_string(&mut buffer)?;

	// Get Random Sample of Questions Based on
	// User Input
	let mut random_questions = vec![];
	let _ = buffer.lines()
		.map(|line| {
			random_questions.push(line);
		})
		.count();
	let sample_questions: Vec<_> = random_questions
        .choose_multiple(&mut rand::thread_rng(), total_questions)
        .collect();

    // Use sample_questions as Questions List
    let mut score = 0;
    for question in sample_questions {
    	// Seperate Question and Answer from Each Sample Question String
    	let mut q_a = question.split('\t').map(|s| s.to_string());
    	let question = q_a.next().expect("##==> INFO! No Question Found");
    	let answer = q_a.next().expect("##==> INFO! No Answer Found");
    	// Test_Question Takes User Input to Check for Match with Actual Answer
    	match test_question(&question, &answer, timeout) {
    		// User Answer == Question Answer, Increment Score
    		Some(true) => score += 1,
    		// User Answer != Question Answer, Inform User and Show Correct Answer
    		Some(false) => {
				println!("{}", "##==> Incorrect!".red());
				let answer_string = format!("##==>> Correct Answer: {}\n", answer);
				println!("{}", answer_string.green());
    		},
    		// User Ran Out of Time, Inform User and Return
    		None => {
    			println!("{}", "\n##==>>>> You ran out of time!".red());
    			break;
    		},
    	}
    }

    // Determine Score by Comparing it to Total Questions
	println!("\n##==>> Score: {} / {}", score, total_questions);
	if score == total_questions {
		println!("{}", "!!! You Are Master Champion !!!".yellow());
	}
	Ok(())
}
