use std::fs::{
	self,
	OpenOptions,
};
use std::io::{
	self,
	Write,
};
use std::path::Path;
use tabled::{
	builder::Builder,
	settings::Style,
};

const TOTAL_ROWS: usize = 3;
const TOTAL_COLUMNS: usize = 3;
const MAX_FILL: usize = TOTAL_ROWS * TOTAL_COLUMNS;

// Board Drawing Functions
fn clearscreen() {
	print!("\x1B[2J\x1B[1;1H");
}

fn fill_box(
	board: &mut Vec<Vec<char>>,
	x: usize,
	y: usize,
	player_char: char,
) -> Result<(), String> {
	if x >= TOTAL_ROWS || y >= TOTAL_COLUMNS {
		return Err("Filling an out of bounds box".to_string());
	}
	board[x][y] = player_char;
	Ok(())
}

fn print_board(board: &Vec<Vec<char>>) {
	let mut builder = Builder::default();
	for i in 0..TOTAL_ROWS {
		let mut row: Vec<char> = Vec::new();
		for j in 0..TOTAL_COLUMNS {
			if board[i][j] == ' ' {
				let box_num = i * TOTAL_ROWS + j + 1;
				let box_num_char = (b'0' + box_num as u8) as char;
				row.push(box_num_char);
			} else {
				row.push(board[i][j]);
			}
		}
		builder.push_record(row);
	}
	
	let table = builder.build().with(Style::modern()).to_string();
	println!("{}", table);
}

fn create_board() -> Vec<Vec<char>> {
	vec![vec![' '; TOTAL_COLUMNS]; TOTAL_ROWS]
}

// Game State Logic
fn check_winner(board: &Vec<Vec<char>>) -> char {
	if is_win(board, 'X') {
		return 'X'
	} else if is_win(board, 'O') {
		return 'O'
	}

	let mut filled_count = 0;
	for i in 0..TOTAL_ROWS {
		for j in 0..TOTAL_COLUMNS {
			if board[i][j] != ' ' {
				filled_count += 1;
			}
 		}
	}

	if filled_count == MAX_FILL {
		return 'D'
	}

	return ' '
}

fn is_win(board: &Vec<Vec<char>>, player_char: char) -> bool {
	for i in 0..TOTAL_ROWS {
		// Check Rows
		if board[i][0] == player_char && board[i][1] == player_char && board[i][2] == player_char {
			return true
		}
		// Check Columns
		if board[0][i] == player_char && board[1][i] == player_char && board[2][i] == player_char {
			return true
		}
	}
	// Check Diagonals
	if board[0][0] == player_char && board[1][1] == player_char && board[2][2] == player_char {
		return true
	}
	if board[0][2] == player_char && board[1][1] == player_char && board[2][0] == player_char {
		return true
	}
	// No Win Condition Encountered
	return false
}

// User Input Logic
fn ask_player_char() -> Result<char, String> {
	loop {
		println!("[***] Choose Your Symbol! [***]");
		println!("[*] First/Second (X/O)?:");
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.map_err(|_| "Failed to read line".to_string())?;
		let character = input
			.trim()
			.chars()
			.next()
			.ok_or("No Input Provided".to_string())
			.map(|c| c.to_ascii_uppercase())?;
		match character {
			'X' | 'O' => return Ok(character),
			c => {
				clearscreen();
				println!("[!] Invalid: {}", c)
			},
		}
	}
}

fn ask_player_move(
	board: &Vec<Vec<char>>,
	human_char: char,
) -> Result<[usize; 2], String> {
	loop {
		println!("[+] Your Move {} -> (1-9)?:", human_char);
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.map_err(|_| "Failed to read line".to_string())?;

		match input.trim().parse() {
			Ok(player_move) => {
				if player_move < 1 || player_move > 9 {
					println!("[!] Invalid: Out of Bounds");
					continue;
				}
				let player_move_array = move_num_to_array(player_move);
				if board[player_move_array[0]][player_move_array[1]] != ' ' {
                    println!("[!] Invalid: {} already filled", player_move)
                } else {
                    return Ok(player_move_array);
                }
			},
			Err(_) => println!("[!] Invalid: Please Enter a Number"),
		}
	}
}

fn move_array_to_num(move_arr: [usize; 2]) -> usize {
	return move_arr[0] * TOTAL_ROWS + move_arr[1] + 1
}

fn move_num_to_array(num: usize) -> [usize; 2] {
	let i: usize = (num - 1) / TOTAL_ROWS;
	let j: usize = (num - 1) % TOTAL_ROWS;
	return [i, j]
}

// AI Logic Functions
fn minimax(
	board: &mut Vec<Vec<char>>,
	is_maximizing: bool,
	depth: isize,
	ai_char: char,
	human_char: char,
) -> isize {
	let result = check_winner(board);
	if result != ' ' {
		if result == 'D' {
			return 0
		} else if result == human_char {
			return -100
		} else {
			return 100
		}
	}

	if is_maximizing {
		let mut best_score = -100;
		for i in 0..TOTAL_ROWS {
			for j in 0..TOTAL_COLUMNS {
				if board[i][j] == ' ' {
					board[i][j] = ai_char;
					let score = minimax(
						board,
						false,
						depth + 1,
						ai_char,
						human_char,
					);
					board[i][j] = ' ';
					if score > best_score {
						best_score = score;
					}
				}
			}
		}

		return best_score - depth
	} else {
		let mut best_score = 100;
        for i in 0..TOTAL_ROWS {
            for j in 0..TOTAL_COLUMNS {
                if board[i][j] == ' ' {
                    board[i][j] = human_char;
                    let score = minimax(
                    	board,
                    	true,
                    	depth + 1,
                    	ai_char,
                    	human_char,
                    );
                    board[i][j] = ' ';
                    if score < best_score {
                        best_score = score;
                    }
                }
            }
        }

        return best_score - depth
	}
}

fn ai_best_move(
	board: &mut Vec<Vec<char>>,
	ai_char: char,
	human_char: char,
) -> Result<[usize; 2], String> {
	let mut best_score = -100;
	let mut best_move: [usize; 2] = Default::default();

	for i in 0..TOTAL_ROWS {
		for j in 0..TOTAL_COLUMNS {
			if board[i][j] == ' ' {
                board[i][j] = ai_char;
                let score = minimax(board, false, 1, ai_char, human_char);
                board[i][j] = ' ';
                let move_num = move_array_to_num([i, j]);
                write_ai_log(
                	&format!("- {} -> Score: {}\n", move_num, score),
                )?;
                if score > best_score {
                    best_score = score;
                    best_move = [i, j];
                }
            }
		}
	}

	write_ai_log(
		&format!("AI's Best Move: {}\n\n", move_array_to_num(best_move)),
	)?;
	Ok(best_move)
}

fn write_ai_log(data: &str) -> Result<(), String> {
	// Open File in Append Mode
	let ai_log_file_string = "./dave_conf/var/dave_ai.log";
	if Path::new(ai_log_file_string).exists() {
		fs::remove_file(ai_log_file_string)
			.map_err(|_| "File cannot be removed".to_string())?;
	}

	let mut file = OpenOptions::new()
		.create_new(true)
		.write(true)
		.append(true)
		.open(ai_log_file_string)
		.map_err(|_| "Failed to open AI log file".to_string())?;

	// Write Data to File
	file.write_all(data.as_bytes())
		.map_err(|error| format!("Error writing to log file: {}", error))
}

// Main Game Loop
pub fn tic_tac_toe_main() -> io::Result<()> {
	if let Err(error) = play() {
		eprintln!("{}", error);
		std::process::exit(1)
	}
	Ok(())
}

fn play() -> Result<(), String> {
	let ref mut board = create_board();
	clearscreen();
	println!("[*] Welcome to Dave's Tic-Tac-Toe Game [*]");

	let human_char = ask_player_char()?;
	let ai_char = if human_char == 'X' { 'O' } else { 'X' };

	let mut filled_box_count = 0;
	let mut winner = ' ';
	let mut ai_last_move = 0;
	let mut is_player_turn = human_char == 'X';

	while filled_box_count < MAX_FILL {
		if is_player_turn {
			clearscreen();
			print_board(board);
			if ai_last_move == 0 {
				println!("[*] AI is Waiting on Your Move ...");
			} else {
				println!("[+] AI Move : {} -> {}", ai_char, ai_last_move);
			}

			let human_move = ask_player_move(board, human_char)?;
			fill_box(board, human_move[0], human_move[1], human_char)?;
		} else {
			let ai_move = ai_best_move(board, ai_char, human_char)?;
			fill_box(board, ai_move[0], ai_move[1], ai_char)?;
			ai_last_move = move_array_to_num(ai_move);
		}

		match check_winner(board) {
			' ' => (),
			w => {
				winner = w;
				break;
			},
		}
		is_player_turn = !is_player_turn;
		filled_box_count += 1;
	}

	clearscreen();
	if winner == human_char {
		println!("[*] YOU ({}) WIN [*]", human_char);
	} else if winner == ai_char {
		println!("[*] YOU ({}) LOSE [*]", human_char);
	} else {
		println!("[*] DRAW! [*]");
	}
	print_board(board);

	Ok(())
}
