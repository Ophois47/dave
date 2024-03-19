use std::io;

struct NoteItem {
	id: u64,
	title: String,
	completed: bool,
}

struct NoteList {
	items: Vec<NoteItem>,
}

impl NoteList {
	fn new() -> NoteList {
		NoteList { items: Vec::new() }
	}

	fn add_item(&mut self, title: String) {
		let id = self.items.len() as u64 + 1;
		let new_item = NoteItem {
			id,
			title: title.clone(),
			completed: false,
		};
		self.items.push(new_item);
		println!("##==> Added: {}", title);
	}

	fn list_items(&self) {
		if self.items.is_empty() {
			println!("##==> Your Notes List is empty");
		} else {
			println!("##==> Your Notes List:");
			for item in &self.items {
				let status = if item.completed { "[X]" } else { "[ ]" };
				println!("{} {} - {}", status, item.id, item.title);
			}
		}
	}

	fn complete_item(&mut self, id: u64) {
		if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
			item.completed = true;
			println!("##==> Completed: {}", item.title);
		} else {
			println!("##==> INFO! Item with ID {} not found", id);
		}
	}
}


pub fn dave_notes() -> io::Result<()> {
	let mut note_list = NoteList::new();

	loop {
		println!("1. Add Note");
		println!("2. List Notes");
		println!("3. Complete Note");
		println!("4. Exit");

		let mut choice = String::new();
		io::stdin().read_line(&mut choice)?;
		let choice: u32 = match choice.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match choice {
			1 => {
				println!("##==> Enter title for new Note:");
				let mut title = String::new();
				io::stdin().read_line(&mut title)?;
				note_list.add_item(title.trim().to_string());
			}
			2 => {
				note_list.list_items();
			}
			3 => {
				println!("##==> Enter the ID of the completed Note:");
				let mut id = String::new();
				io::stdin().read_line(&mut id)?;
				let id: u64 = match id.trim().parse() {
					Ok(num) => num,
					Err(_) => continue,
				};
				note_list.complete_item(id);
			}
			4 => {
				println!("##==> Exiting Dave's Notes ...");
				break;
			}
			_ => {
				println!("##==> Invalid Choice. Please Enter a Value Between 1-4");
			}
		}
	}
	Ok(())
}
