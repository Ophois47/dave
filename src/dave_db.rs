use sled::{Config, Db};
use crate::dave_notes::DaveNote;
use std::io;
use std::path::PathBuf;
use std::process::exit;

pub struct DaveDatabase;
impl DaveDatabase {
	// Configure A New Database
	pub fn config(db_location: PathBuf) -> io::Result<Db> {
		let config = match Config::new()
			.create_new(true)
			.path(db_location.to_owned())
			.open() {
				Ok(c) => c,
				Err(sled::Error::Io(_error)) => {
					let config = match sled::open(db_location.to_owned()) {
						Ok(config) => config,
						Err(error) => {
							eprintln!("{}", error);
							exit(1)
						}
					};
					config
				},
				Err(error) => {
					eprintln!("{}", error);
					exit(1)
				},
			};
		Ok(config)
	}

	// Insert Value Into Database
	pub fn update(db: &mut sled::Db, data: DaveNote, id: &[u8]) -> io::Result<()> {
		let bytes = match bincode::serialize(&data) {
			Ok(bytes) => bytes,
			Err(error) => {
				eprintln!("{}", error);
				std::process::exit(1)
			},
		};
		db.insert(id, bytes)?;
		Ok(())
	}
}
