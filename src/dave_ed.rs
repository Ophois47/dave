use std::cmp;
use std::env;
use std::fs;
use std::io::{
	self,
	stdout,
	Error,
	Write,
};
use std::time::{
	Duration,
	Instant,
};
use termion::{
	color,
	event::Key,
	input::TermRead,
	raw::{
		IntoRawMode,
		RawTerminal,
	},
};
use unicode_segmentation::UnicodeSegmentation;

//
// Terminal
//
struct Size {
	width: u16,
	height: u16,
}

struct Terminal {
	size: Size,
	_stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
	fn default() -> Result<Self, std::io::Error> {
		let size = termion::terminal_size()?;
		Ok(Self {
			size: Size {
				width: size.0,
				height: size.1.saturating_sub(2),
			},
			_stdout: stdout().into_raw_mode()?,
		})
	}

	fn size(&self) -> &Size {
		&self.size
	}
}

//
// Highlighting
//
#[derive(PartialEq, Clone, Copy, Debug)]
enum Type {
	None,
	Number,
	Match,
	String,
	Character,
	Comment,
	MultilineComment,
	PrimaryKeywords,
	SecondaryKeywords,
}

impl Type {
	fn to_color(self) -> impl color::Color {
		match self {
			Type::Number => color::Rgb(220, 163, 163),
			Type::Match => color::Rgb(38, 139, 210),
			Type::String => color::Rgb(211, 54, 130),
			Type::Character => color::Rgb(108, 113, 196),
			Type::Comment | Type::MultilineComment => color::Rgb(133, 153, 0),
			Type::PrimaryKeywords => color::Rgb(181, 137, 0),
			Type::SecondaryKeywords => color::Rgb(42, 161, 152),
			_ => color::Rgb(255, 255, 255),
		}
	}
}

//
// Row
//
#[derive(Default)]
struct Row {
	string: String,
	highlighting: Vec<Type>,
	is_highlighted: bool,
	len: usize,
}

impl From<&str> for Row {
	fn from(slice: &str) -> Self {
		Self {
			string: String::from(slice),
			highlighting: Vec::new(),
			is_highlighted: false,
			len: slice.graphemes(true).count(),
		}
	}
}

impl Row {
	fn render(&self, start: usize, end: usize) -> String {
		let end = cmp::min(end, self.string.len());
		let start = cmp::min(start, end);
		let mut result = String::new();
		let mut current_highlighting = &Type::None;

		for (index, grapheme) in self.string[..]
			.graphemes(true)
			.enumerate()
			.skip(start)
			.take(end - start)
		{
			if let Some(c) = grapheme.chars().next() {
				let highlighting_type = self
					.highlighting
					.get(index)
					.unwrap_or(&Type::None);
				if highlighting_type != current_highlighting {
					current_highlighting = highlighting_type;
					let start_highlight = format!("{}", termion::color::Fg(highlighting_type.to_color()));
					result.push_str(&start_highlight[..]);
				}
				if c == '\t' {
					result.push_str(" ");
				} else {
					result.push(c);
				}
			}
		}

		let end_highlight = format!("{}", termion::color::Fg(color::Reset));
		result.push_str(&end_highlight[..]);
		result
	}

	fn len(&self) -> usize {
		self.len
	}

	#[allow(dead_code)]
	fn is_empty(&self) -> bool {
		self.len == 0
	}

	fn insert(&mut self, at: usize, c: char) {
		if at >= self.len() {
			self.string.push(c);
			self.len += 1;
			return;
		}

		let mut result: String = String::new();
		let mut length = 0;

		for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
			length += 1;
			if index == at {
				length += 1;
				result.push(c);
			}
			result.push_str(grapheme);
		}

		self.len = length;
		self.string = result;
	}

	fn delete(&mut self, at: usize) {
		if at >= self.len() {
			return;
		}

		let mut result: String = String::new();
		let mut length = 0;

		for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
			if index != at {
				length += 1;
				result.push_str(grapheme);
			}
		}
		self.len = length;
		self.string = result;
	}

	fn append(&mut self, new: &Self) {
		self.string = format!("{}{}", self.string, new.string);
		self.len += new.len;
	}

	fn split(&mut self, at: usize) -> Self {
		let mut row: String = String::new();
		let mut length = 0;
		let mut splitted_row: String = String::new();
		let mut splitted_length = 0;

		for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
			if index < at {
				length += 1;
				row.push_str(grapheme);
			} else {
				splitted_length += 1;
				splitted_row.push_str(grapheme);
			}
		}

		self.string = row;
		self.len = length;
		self.is_highlighted = false;

		Self {
			string: splitted_row,
			len: splitted_length,
			is_highlighted: false,
			highlighting: Vec::new(),
		}
	}

	fn as_bytes(&self) -> &[u8] {
		self.string.as_bytes()
	}

	fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
		if at > self.len || query.is_empty() {
			return None;
		}

		let start = if direction == SearchDirection::Forward {
			at
		} else {
			0
		};

		let end = if direction == SearchDirection::Forward {
			self.len
		} else {
			at
		};

		let substring: String = self.string[..]
			.graphemes(true)
			.skip(start)
			.take(end - start)
			.collect();
		let matching_byte_index = if direction == SearchDirection::Forward {
			substring.find(query)
		} else {
			substring.rfind(query)
		};

		if let Some(matching_byte_index) = matching_byte_index {
			for (grapheme_index, (byte_index, _)) in substring[..].grapheme_indices(true).enumerate() {
				if matching_byte_index == byte_index {
					return Some(start + grapheme_index);
				}
			}
		}
		None
	}

	fn highlight_match(&mut self, word: &Option<String>) {
		if let Some(word) = word {
			if word.is_empty() {
				return;
			}
			let mut index = 0;
			while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
				if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count()) {
					for i in search_match..next_index {
						self.highlighting[i] = Type::Match;
					}
					index = next_index;
				} else {
					break;
				}
			}
		}
	}

	fn highlight_str(
		&mut self,
		index: &mut usize,
		substring: &str,
		chars: &[char],
		hl_type: Type,
	) -> bool {
		if substring.is_empty() {
			return false;
		}
		for (substring_index, c) in substring.chars().enumerate() {
			if let Some(next_char) = chars.get(index.saturating_add(substring_index)) {
				if *next_char != c {
					return false;
				}
			} else {
				return false;
			}
		}

		for _ in 0..substring.len() {
			self.highlighting.push(hl_type);
			*index += 1;
		}
		true
	}

	fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[String],
        hl_type: Type,
    ) -> bool {
        if *index > 0 {
            let prev_char = chars[*index - 1];
            if !is_separator(prev_char) {
                return false;
            }
        }
        for word in keywords {
            if *index < chars.len().saturating_sub(word.len()) {
                let next_char = chars[*index + word.len()];
                if !is_separator(next_char) {
                    continue;
                }
            }

            if self.highlight_str(index, &word, chars, hl_type) {
                return true;
            }
        }
        false
    }

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.primary_keywords(),
            Type::PrimaryKeywords,
        )
    }

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.secondary_keywords(),
            Type::SecondaryKeywords,
        )
    }

    fn highlight_char(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.characters() && c == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let closing_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };
                if let Some(closing_char) = chars.get(closing_index) {
                    if *closing_char == '\'' {
                        for _ in 0..=closing_index.saturating_sub(*index) {
                            self.highlighting.push(Type::Character);
                            *index += 1;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlighting.push(Type::Comment);
                        *index += 1;
                    }
                    return true;
                }
            };
        }
        false
    }

    fn highlight_multiline_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let closing_index =
                        if let Some(closing_index) = self.string[*index + 2..].find("*/") {
                            *index + closing_index + 4
                        } else {
                            chars.len()
                        };
                    for _ in *index..closing_index {
                        self.highlighting.push(Type::MultilineComment);
                        *index += 1;
                    }
                    return true;
                }
            };
        }
        false
    }

    fn highlight_string(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.strings() && c == '"' {
            loop {
                self.highlighting.push(Type::String);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.highlighting.push(Type::String);
            *index += 1;
            return true;
        }
        false
    }

    fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.numbers() && c.is_ascii_digit() {
            if *index > 0 {
                let prev_char = chars[*index - 1];
                if !is_separator(prev_char) {
                    return false;
                }
            }
            loop {
                self.highlighting.push(Type::Number);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        }
        false
    }

    pub fn highlight(
        &mut self,
        opts: &HighlightingOptions,
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let chars: Vec<char> = self.string.chars().collect();
        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlighting.last() {
                if *hl_type == Type::MultilineComment
                    && self.string.len() > 1
                    && self.string[self.string.len() - 2..] == *"*/"
                {
                    return true;
                }
            }
            return false;
        }
        self.highlighting = Vec::new();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;
        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };
            for _ in 0..closing_index {
                self.highlighting.push(Type::MultilineComment);
            }
            index = closing_index;
        }
        while let Some(c) = chars.get(index) {
            if self.highlight_multiline_comment(&mut index, &opts, *c, &chars) {
                in_ml_comment = true;
                continue;
            }
            in_ml_comment = false;
            if self.highlight_char(&mut index, opts, *c, &chars)
                || self.highlight_comment(&mut index, opts, *c, &chars)
                || self.highlight_primary_keywords(&mut index, &opts, &chars)
                || self.highlight_secondary_keywords(&mut index, &opts, &chars)
                || self.highlight_string(&mut index, opts, *c, &chars)
                || self.highlight_number(&mut index, opts, *c, &chars)
            {
                continue;
            }
            self.highlighting.push(Type::None);
            index += 1;
        }
        self.highlight_match(word);
        if in_ml_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }
        self.is_highlighted = true;
        false
    }
}

fn is_separator(c: char) -> bool {
	c.is_ascii_punctuation() || c.is_ascii_whitespace()
}

//
// Document
//
#[derive(Default)]
struct Document {
	rows: Vec<Row>,
	file_name: Option<String>,
	dirty: bool,
	file_type: FileType,
}

impl Document {
	fn open(filename: &str) -> Result<Self, std::io::Error> {
		let contents = fs::read_to_string(filename)?;
		let file_type = FileType::from(filename);
		let mut rows = Vec::new();

		for value in contents.lines() {
			rows.push(Row::from(value));
		}

		Ok(Self {
			rows,
			file_name: Some(filename.to_string()),
			dirty: false,
			file_type,
		})
	}

	fn file_type(&self) -> String {
		self.file_type.name()
	}

	fn row(&self, index: usize) -> Option<&Row> {
		self.rows.get(index)
	}

	fn is_empty(&self) -> bool {
		self.rows.is_empty()
	}

	fn len(&self) -> usize {
		self.rows.len()
	}

	fn insert_newline(&mut self, at: &Position) {
		if at.y > self.rows.len() {
			return;
		}
		if at.y == self.rows.len() {
			self.rows.push(Row::default());
			return;
		}

		let current_row = &mut self.rows[at.y];
		let new_row = current_row.split(at.x);
		self.rows.insert(at.y + 1, new_row);
	}

	fn insert(&mut self, at: &Position, c: char) {
		if at.y > self.rows.len() {
			return;
		}
		self.dirty = true;
		if c == '\n' {
			self.insert_newline(at);
		} else if at.y == self.rows.len() {
			let mut row = Row::default();
			row.insert(0, c);
			self.rows.push(row);
		} else {
			let row = &mut self.rows[at.y];
			row.insert(at.x, c);
		}
		self.unhighlight_rows(at.y);
	}

	fn unhighlight_rows(&mut self, start: usize) {
		let start = start.saturating_sub(1);
		for row in self.rows.iter_mut().skip(start) {
			row.is_highlighted = false;
		}
	}

	fn delete(&mut self, at: &Position) {
		let len = self.rows.len();
		if at.y >= len {
			return;
		}
		self.dirty = true;

		if at.x == self.rows[at.y].len() && at.y + 1 < len {
			let next_row = self.rows.remove(at.y + 1);
			let row = &mut self.rows[at.y];
			row.append(&next_row);
		} else {
			let row = &mut self.rows[at.y];
			row.delete(at.x);
		}
		self.unhighlight_rows(at.y);
	}

	fn load(&mut self) -> Result<(), Error> {
		if let Some(file_name) = &self.file_name {
			let _file = fs::File::open(file_name)?;
			self.file_type = FileType::from(file_name);
			self.dirty = false;
		}
		Ok(())
	}

	fn save(&mut self) -> Result<(), Error> {
		if let Some(file_name) = &self.file_name {
			let mut file = fs::File::create(file_name)?;
			self.file_type = FileType::from(file_name);
			for row in &mut self.rows {
				file.write_all(row.as_bytes())?;
				file.write_all(b"\n")?;
			}
			self.dirty = false;
		}
		Ok(())
	}

	fn is_dirty(&self) -> bool {
		self.dirty
	}

	fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
		if at.y >= self.rows.len() {
			return None;
		}
		let mut position = Position { x: at.x, y: at.y };
		let start = if direction == SearchDirection::Forward {
			at.y
		} else {
			0
		};
		let end = if direction == SearchDirection::Forward {
			self.rows.len()
		} else {
			at.y.saturating_add(1)
		};
		for _ in start..end {
			if let Some(row) = self.rows.get(position.y) {
				if let Some(x) = row.find(&query, position.x, direction) {
					position.x = x;
					return Some(position);
				}
				if direction == SearchDirection::Forward {
					position.y = position.y.saturating_add(1);
					position.x = 0;
				} else {
					position.y = position.y.saturating_sub(1);
					position.x = self.rows[position.y].len();
				}
			} else {
				return None;
			}
		}
		None
	}

	fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
		let mut start_with_comment = false;
		let until = if let Some(until) = until {
			if until.saturating_add(1) < self.rows.len() {
				until.saturating_add(1)
			} else {
				self.rows.len()
			}
		} else {
			self.rows.len()
		};

		for row in &mut self.rows[..until] {
			start_with_comment = row.highlight(
				&self.file_type.highlighting_options(),
				word,
				start_with_comment,
			);
		}
	}
}

//
// FileType
//
pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }
    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "as".to_string(),
                        "break".to_string(),
                        "const".to_string(),
                        "continue".to_string(),
                        "crate".to_string(),
                        "else".to_string(),
                        "enum".to_string(),
                        "extern".to_string(),
                        "false".to_string(),
                        "fn".to_string(),
                        "for".to_string(),
                        "if".to_string(),
                        "impl".to_string(),
                        "in".to_string(),
                        "let".to_string(),
                        "loop".to_string(),
                        "match".to_string(),
                        "mod".to_string(),
                        "move".to_string(),
                        "mut".to_string(),
                        "pub".to_string(),
                        "ref".to_string(),
                        "return".to_string(),
                        "self".to_string(),
                        "Self".to_string(),
                        "static".to_string(),
                        "struct".to_string(),
                        "super".to_string(),
                        "trait".to_string(),
                        "true".to_string(),
                        "type".to_string(),
                        "unsafe".to_string(),
                        "use".to_string(),
                        "where".to_string(),
                        "while".to_string(),
                        "dyn".to_string(),
                        "abstract".to_string(),
                        "become".to_string(),
                        "box".to_string(),
                        "do".to_string(),
                        "final".to_string(),
                        "macro".to_string(),
                        "override".to_string(),
                        "priv".to_string(),
                        "typeof".to_string(),
                        "unsized".to_string(),
                        "virtual".to_string(),
                        "yield".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "try".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(),
                        "char".to_string(),
                        "i8".to_string(),
                        "i16".to_string(),
                        "i32".to_string(),
                        "i64".to_string(),
                        "isize".to_string(),
                        "u8".to_string(),
                        "u16".to_string(),
                        "u32".to_string(),
                        "u64".to_string(),
                        "usize".to_string(),
                        "f32".to_string(),
                        "f64".to_string(),
                    ],
                },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    pub fn numbers(&self) -> bool {
        self.numbers
    }
    pub fn strings(&self) -> bool {
        self.strings
    }
    pub fn characters(&self) -> bool {
        self.characters
    }
    pub fn comments(&self) -> bool {
        self.comments
    }
    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }
    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }
    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}

//
// Editor
//
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

#[derive(PartialEq, Copy, Clone)]
enum SearchDirection {
	Forward,
	Backward,
}

#[derive(Default, Clone)]
struct Position {
	x: usize,
	y: usize,
}

struct StatusMessage {
	text: String,
	time: Instant,
}

impl StatusMessage {
	fn from(message: String) -> Self {
		Self {
			time: Instant::now(),
			text: message,
		}
	}
}

struct DaveEd {
	should_quit: bool,
	terminal: Terminal,
	cursor_position: Position,
	offset: Position,
	document: Document,
	status_message: StatusMessage,
	quit_times: u8,
	highlighted_word: Option<String>,
}

impl DaveEd {
	fn run(&mut self) {
		loop {
			if let Err(error) = self.refresh_screen() {
				Self::die(error);
			}
			if self.should_quit {
				break;
			}
			if let Err(error) = self.process_keypress() {
				Self::die(error);
			}
		}
	}

	fn default() -> Self {
		let args: Vec<String> = env::args().collect();
		let mut initial_status = String::from("HELP: Ctrl + F = Find | Ctrl + S = Save | Ctrl + Q = Quit");
		let document = if let Some(file_name) = args.get(1) {
			let doc = Document::open(file_name);
			if let Ok(doc) = doc {
				doc
			} else {
				initial_status = format!("ERROR: Could Not Open File - {}", file_name);
				Document::default()
			}
		} else {
			Document::default()
		};

		Self {
			should_quit: false,
			terminal: Terminal::default().unwrap(),
			document,
			cursor_position: Position::default(),
			offset: Position::default(),
			status_message: StatusMessage::from(initial_status),
			quit_times: QUIT_TIMES,
			highlighted_word: None,
		}
	}

	fn load_file(file_name: String) -> Self {
		let initial_status = String::from("HELP: Ctrl + F = Find | Ctrl + S = Save | Ctrl + Q = Quit");
		let document = Document::open(file_name.as_str());

		Self {
			should_quit: false,
			terminal: Terminal::default().unwrap(),
			document: document.unwrap(),
			cursor_position: Position::default(),
			offset: Position::default(),
			status_message: StatusMessage::from(initial_status),
			quit_times: QUIT_TIMES,
			highlighted_word: None,
		}
	}

	fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Self::cursor_hide();
        Self::cursor_position(&Position::default());
        if self.should_quit {
            Self::clear_screen();
            println!("##==> Thank you for using DaveEd. Have a great day!\r");
        } else {
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height as usize),
                ),
            );
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Self::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Self::cursor_show();
        Self::flush()
    }

    fn show_help(&mut self) {
    	let help_string = format!("HELP: Ctrl + F = Find | Ctrl + S = Save | Ctrl + Q = Quit");
    	self.status_message = StatusMessage::from(help_string);
    }

    fn load(&mut self) {
    	let new_name = self.prompt("Load File: ", |_, _, _| {}).unwrap_or(None);
    	if new_name.is_none() {
    		self.status_message = StatusMessage::from("Must Enter File Name".to_string());
    		return;
    	}
    	self.document.file_name = new_name;

    	if self.document.load().is_ok() {
    		self.status_message = StatusMessage::from("File Loaded Successfully".to_string());
    		dave_ed_load_file(self.document.file_name.clone().unwrap());
    	} else {
    		self.status_message = StatusMessage::from("Error Loading File".to_string());
    	}
    }

	fn save(&mut self) {
		if self.document.file_name.is_none() {
			let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
			if new_name.is_none() {
				self.status_message = StatusMessage::from("Save Aborted".to_string());
				return;
			}
			self.document.file_name = new_name;
		}

		if self.document.save().is_ok() {
			self.status_message = StatusMessage::from("File Saved Successfully".to_string());
		} else {
			self.status_message = StatusMessage::from("Error Writing File".to_string());
		}
	}

	fn search(&mut self) {
		let old_position = self.cursor_position.clone();
		let mut direction = SearchDirection::Forward;
		let query = self.prompt(
			"Search (ESC to Cancel, Arrows to Navigate)",
			|editor, key, query| {
				let mut moved = false;
				match key {
					Key::Right | Key::Down => {
						direction = SearchDirection::Forward;
						editor.move_cursor(Key::Right);
						moved = true;
					}
					Key::Left | Key::Up => direction = SearchDirection::Backward,
					_ => direction = SearchDirection::Forward,
				}
				if let Some(position) = editor.document.find(&query, &editor.cursor_position, direction) {
					editor.cursor_position = position;
					editor.scroll();
				} else if moved {
					editor.move_cursor(Key::Left);
				}
				editor.highlighted_word = Some(query.to_string());
			},
		)
		.unwrap_or(None);

		if query.is_none() {
			self.cursor_position = old_position;
			self.scroll();
		}
		self.highlighted_word = None;
	}

	fn process_keypress(&mut self) -> Result<(), std::io::Error> {
		let pressed_key = Self::read_key()?;
		match pressed_key {
			Key::Ctrl('q') => {
				if self.quit_times > 0 && self.document.is_dirty() {
					self.status_message = StatusMessage::from(format!(
						"WARNING! File has unsaved changes! Press Ctrl + Q {} more times to quit.",
						self.quit_times,
					));
					self.quit_times -= 1;
					return Ok(());
				}
				self.should_quit = true
			}
			Key::Ctrl('s') => self.save(),
			Key::Ctrl('l') => self.load(),
			Key::Ctrl('f') => self.search(),
			Key::Ctrl('h') => self.show_help(),
			Key::Char(c) => {
				self.document.insert(&self.cursor_position, c);
				self.move_cursor(Key::Right);
			}
			Key::Delete => self.document.delete(&self.cursor_position),
			Key::Backspace => {
				if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
					self.move_cursor(Key::Left);
					self.document.delete(&self.cursor_position);
				}
			}
			Key::Up
			| Key::Down
			| Key::Left
			| Key::Right
			| Key::PageUp
			| Key::PageDown
			| Key::End
			| Key::Home => self.move_cursor(pressed_key),
			_ => (),
		}

		self.scroll();
		if self.quit_times < QUIT_TIMES {
			self.quit_times = QUIT_TIMES;
			self.status_message = StatusMessage::from(String::new());
		}

		Ok(())
	}

	fn scroll(&mut self) {
		let Position { x, y } = self.cursor_position;
		let width = self.terminal.size().width as usize;
		let height = self.terminal.size().height as usize;
		#[allow(unused_mut)]
		let mut offset = &mut self.offset;

		if y < offset.y {
			offset.y = y;
		} else if y >= offset.y.saturating_add(height) {
			offset.y = y.saturating_sub(height).saturating_add(1);
		}
		if x < offset.x {
			offset.x = x;
		} else if x >= offset.x.saturating_add(width) {
			offset.x = x.saturating_sub(width).saturating_add(1);
		}
	}

	fn move_cursor(&mut self, key: Key) {
		let terminal_height = self.terminal.size().height as usize;
		let Position { mut y, mut x } = self.cursor_position;
		let height = self.document.len();

		let mut width = if let Some(row) = self.document.row(y) {
			row.len()
		} else {
			0
		};

		match key {
			Key::Up => y = y.saturating_sub(1),
			Key::Down => {
				if y < height {
					y = y.saturating_add(1);
				}
			}
			Key::Left => {
				if x > 0 {
					x -= 1;
				} else if y > 0 {
					y -= 1;
					if let Some(row) = self.document.row(y) {
						x = row.len();
					} else {
						x = 0;
					}
				}
			}
			Key::Right => {
				if x < width {
					x += 1;
				} else if y < height {
					y += 1;
					x = 0;
				}
			}
			Key::PageUp => {
				y = if y > terminal_height {
					y.saturating_sub(terminal_height)
				} else {
					0
				}
			}
			Key::PageDown => {
				y = if y.saturating_add(terminal_height) < height {
					y.saturating_add(terminal_height)
				} else {
					height
				}
			}
			Key::Home => x = 0,
			Key::End => x = width,
			_ => (),
		}

		width = if let Some(row) = self.document.row(y) {
			row.len()
		} else {
			0
		};

		if x > width {
			x = width;
		}

		self.cursor_position = Position { x, y }
	}

	fn draw_welcome_message(&self) {
		let mut welcome_message = format!("DaveEd -- Version {}", VERSION);
		let width = self.terminal.size().width as usize;
		let len = welcome_message.len();

		let padding = width.saturating_sub(len) / 2;
		let spaces = " ".repeat(padding.saturating_sub(1));

		welcome_message = format!("~{}{}", spaces, welcome_message);
		welcome_message.truncate(width);
		println!("{}\r", welcome_message);
	}

	fn draw_row(&self, row: &Row) {
		let width = self.terminal.size().width as usize;
		let start = self.offset.x;
		let end = self.offset.x.saturating_add(width);
		let row = row.render(start, end);
		println!("{}\r", row);
	}

	fn draw_rows(&self) {
		let height = self.terminal.size().height;
		for terminal_row in 0..height {
			Self::clear_current_line();
			if let Some(row) = self.document.row(self.offset.y.saturating_add(terminal_row as usize)) {
				self.draw_row(row);
			} else if self.document.is_empty() && terminal_row == height / 3 {
				self.draw_welcome_message();
			} else {
				println!("~\r");
			}
		}
	}

	fn draw_status_bar(&self) {
		let mut status;
		let width = self.terminal.size().width as usize;
		let modified_indicator = if self.document.is_dirty() {
			" (modified)"
		} else {
			""
		};

		let mut file_name = "[No Name]".to_string();
		if let Some(name) = &self.document.file_name {
			file_name = name.clone();
			file_name.truncate(20);
		}
		status = format!("{} - {} lines{}", file_name, self.document.len(), modified_indicator);
		let line_indicator = format!(
			"{} | {}/{}",
			self.document.file_type(),
			self.cursor_position.y.saturating_add(1),
			self.document.len(),
		);

		let len = status.len() + line_indicator.len();
		status.push_str(&" ".repeat(width.saturating_sub(len)));
		status = format!("{}{}", status, line_indicator);
		status.truncate(width);
		Self::set_bg_color(STATUS_BG_COLOR);
		Self::set_fg_color(STATUS_FG_COLOR);
		println!("{}\r", status);
		Self::reset_fg_color();
		Self::reset_bg_color();
	}

	fn draw_message_bar(&self) {
		Self::clear_current_line();
		let message = &self.status_message;
		if Instant::now() - message.time < Duration::new(5, 0) {
			let mut text = message.text.clone();
			text.truncate(self.terminal.size().width as usize);
			print!("{}", text);
		}
	}

	fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
	where
		C: FnMut(&mut Self, Key, &String),
	{
		let mut result = String::new();
		loop {
			self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
			self.refresh_screen()?;
			let key = Self::read_key()?;
			match key {
				Key::Backspace => result.truncate(result.len().saturating_sub(1)),
				Key::Char('\n') => break,
				Key::Char(c) => {
					if !c.is_control() {
						result.push(c);
					}
				}
				Key::Esc => {
					result.truncate(0);
					break;
				}
				_ => (),
			}
			callback(self, key, &result);
		}
		self.status_message = StatusMessage::from(String::new());
		if result.is_empty() {
			return Ok(None);
		}
		Ok(Some(result))
	}

	fn die(e: std::io::Error) {
		Self::clear_screen();
		panic!("{}", e);
	}

	//
	// Terminal
	//
	fn clear_screen() {
		print!("{}", termion::clear::All);
	}

	fn read_key() -> Result<Key, std::io::Error> {
		loop {
			if let Some(key) = io::stdin().lock().keys().next() {
				return key;
			}
		}
	}

	fn clear_current_line() {
		print!("{}", termion::clear::CurrentLine);
	}

	fn set_bg_color(color: color::Rgb) {
		print!("{}", color::Bg(color));
	}

	fn reset_bg_color() {
		print!("{}", color::Bg(color::Reset));
	}

	fn set_fg_color(color: color::Rgb) {
		print!("{}", color::Fg(color));
	}

	fn reset_fg_color() {
		print!("{}", color::Fg(color::Reset));
	}

	fn cursor_position(position: &Position) {
		let Position { mut x, mut y } = position;
		x = x.saturating_add(1);
		y = y.saturating_add(1);
		let x = x as u16;
		let y = y as u16;
		print!("{}", termion::cursor::Goto(x, y));
	}

	fn flush() -> Result<(), std::io::Error> {
		io::stdout().flush()
	}

	fn cursor_hide() {
		print!("{}", termion::cursor::Hide);
	}

	fn cursor_show() {
		print!("{}", termion::cursor::Show);
	}
}

pub fn dave_ed_load_file(file_name: String) {
	DaveEd::load_file(file_name).run();
}

pub fn dave_ed_main() {
	DaveEd::default().run();
}
