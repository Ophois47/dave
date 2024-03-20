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
}
