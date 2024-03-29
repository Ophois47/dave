use crate::dave_hash::*;
use lazy_static::lazy_static;
use serde_json::json;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::sync::RwLock;

// Default Filename for Configuration File
pub const CONF_FILE: &str = "dave.conf";

// Default Installation Directory
pub const ROOT_PATH: &str = "./dave_conf";

// Default Database Name
pub const DB_NAME: &str = "dave_db";

// Default Location of Output File
pub const OUTPUT_FILE: &str = "dave.out";

// Default Location of Budget File
pub const BUDGET_FILE: &str = "dave.budget";

lazy_static! {
	pub static ref CONFIG: RwLock<DaveConfig> = {
		RwLock::new(DaveConfig::load())
	};
}

// Default Configuration
lazy_static! {
	static ref DEFAULT_CONFIG: serde_json::Value = json!({
		"config_path": find_config_path(),
		"root_path": find_root_path(),
		"database_path": find_database_path(),
		"output_file": find_output_file(),
		"budget_file": find_budget_file(),
		"hash_type": HashType::Sha256,
	});
}

// Determines Appropriate Path to Config File
pub fn find_config_path() -> PathBuf {
	let mut config_path = PathBuf::new();
	let dave_home = ROOT_PATH.to_string();

	config_path.push(dave_home);
	config_path.push("etc/");
	config_path.push(CONF_FILE);
	config_path
}

// Determines Root Path
pub fn find_root_path() -> PathBuf {
	let mut root_path = PathBuf::new();
	let dave_home = ROOT_PATH.to_string();

	root_path.push(dave_home);
	root_path
}

// Determines Path to Database
pub fn find_database_path() -> PathBuf {
	let mut db_path = PathBuf::new();
	let dave_home = ROOT_PATH.to_string();

	db_path.push(dave_home);
	db_path.push("var/");
	db_path.push(DB_NAME);
	db_path
}

// Determines Appropriate Path to Output File
pub fn find_output_file() -> PathBuf {
	let mut output_file_path = PathBuf::new();
	let dave_home = ROOT_PATH.to_string();

	output_file_path.push(dave_home);
	output_file_path.push("var/");
	output_file_path.push(OUTPUT_FILE);
	output_file_path
}

// Determines Appropriate Path to Budget File
pub fn find_budget_file() -> PathBuf {
	let mut output_file_path = PathBuf::new();
	let dave_home = ROOT_PATH.to_string();

	output_file_path.push(dave_home);
	output_file_path.push("var/");
	output_file_path.push(BUDGET_FILE);
	output_file_path
}

// Runtime Configuration for Dave
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DaveConfig {
	// File Path to This Configuration
	config_path: PathBuf,
	// Root Path
	root_path: PathBuf,
	// Database Path
	database_path: PathBuf,
	// Output File Path
	output_file: PathBuf,
	// Budget File Path
	budget_file: PathBuf,
	// What Hashing Algorithm Dave Will Use
	pub hash_type: HashType,
}

impl Default for DaveConfig {
	fn default() -> Self {
		let default: Self = serde_json::from_value(DEFAULT_CONFIG.clone()).unwrap();
		default
	}
}

impl DaveConfig {
	pub fn load() -> Self {
		let config_path = find_config_path();

		let file = match File::open(&config_path) {
			Ok(file) => file,
			Err(error) => {
				println!("##==>> Warning! Config File Not Found, Using Defaults");
				println!("##==>> Warning! {}: {}\n", config_path.display(), error);
				return Self::default()
			}
		};

		match serde_json::from_reader(file) {
			Ok(deserialized) => deserialized,
			Err(error) => {
				println!("##==>> Warning! Failed to Parse Config File, Using Defaults");
				println!("##==>> Warning! {}: Parse Error: {}\n", &config_path.display(), error);
				Self::default()
			}
		}
	}

	pub fn save(&self) -> Result<(), Box<dyn Error>> {
		let file = OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(find_config_path())?;

		serde_json::to_writer_pretty(file, self)?;
		Ok(())
	}

	pub fn set_config_path(&mut self, path: PathBuf) {
		self.config_path = path;
	}

	pub fn config_path(&self) -> PathBuf {
		self.config_path.clone()
	}

	pub fn set_root_path(&mut self, path: PathBuf) {
		self.root_path = path;
	}

	pub fn root_path(&self) -> PathBuf {
		self.root_path.clone()
	}

	pub fn set_database_path(&mut self, path: PathBuf) {
		self.database_path = path;
	}

	pub fn database_path(&self) -> PathBuf {
		self.database_path.clone()
	}

	pub fn set_output_file(&mut self, path: PathBuf) {
		self.output_file = path;
	}

	pub fn output_file(&self) -> PathBuf {
		self.output_file.clone()
	}

	pub fn set_budget_path(&mut self, path: PathBuf) {
		self.budget_file = path;
	}

	pub fn budget_path(&self) -> PathBuf {
		self.budget_file.clone()
	}

	pub fn set_hash_type(&mut self, hash_type: HashType) {
		self.hash_type = hash_type;
	}

	pub fn hash_type(&self) -> HashType {
		self.hash_type
	}
}
