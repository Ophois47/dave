#[derive(Serialize, Deserialize)]
pub struct DaveNote {
	pub id: u64,
	pub title: String,
	pub completed: bool,
}

impl DaveNote {
	pub fn new() -> DaveNote {
		DaveNote { id: 0, title: "".to_string(), completed: false }
	}

	/*pub fn add_item(&mut self, title: String) {
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
	}*/
}
