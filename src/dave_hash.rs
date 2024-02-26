use sha3::{self, Digest};
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

// Possible Hashing Algorithms
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum HashType {
	Md5,
	Sha3_256,
	Sha3_384,
	Sha3_512
}

impl Into<Box<dyn Hasher>> for HashType {
	fn into(self) -> Box<dyn Hasher> {
		match self {
			HashType::Md5 		=> Box::new(Md5Hash {}),
			HashType::Sha3_256 	=> Box::new(Sha3_256Hash {}),
			HashType::Sha3_384 	=> Box::new(Sha3_384Hash {}),
			HashType::Sha3_512 	=> Box::new(Sha3_512Hash {}),
		}
	}
}

impl fmt::Display for HashType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			HashType::Md5 => write!(f, "MD5"),
			HashType::Sha3_256 => write!(f, "SHA3_256"),
			HashType::Sha3_384 => write!(f, "SHA3_384"),
			HashType::Sha3_512 => write!(f, "SHA3_512"),
		}
	}
}

#[derive(Debug)]
pub struct DaveError {
	source: DaveErrorPal,
}

impl fmt::Display for DaveError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Dave Error Occurred!")
	}
}

impl Error for DaveError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.source)
	}
}

#[derive(Debug)]
struct DaveErrorPal;

impl fmt::Display for DaveErrorPal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Dave Error Pal Occurred!")
	}
}

impl Error for DaveErrorPal {}

#[allow(dead_code)]
fn get_dave_error() -> Result<(), DaveError> {
	Err(DaveError { source: DaveErrorPal })
}

impl FromStr for HashType {
	type Err = DaveError;
	fn from_str(s: &str) -> Result<HashType, DaveError> {
		let gotten_hash_type = match s {
			"md5" 		=> HashType::Md5,
			"sha3-384" 	=> HashType::Sha3_384,
			"sha3-512" 	=> HashType::Sha3_512,
			_ 			=> HashType::Sha3_256,
		};
		Ok(gotten_hash_type)
	}
}

pub trait Hasher {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>>;
}

pub struct Md5Hash;
pub struct Sha3_256Hash;
pub struct Sha3_384Hash;
pub struct Sha3_512Hash;

impl Hasher for Md5Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut md5_context = md5::Context::new();
		let f = File::open(file.clone())?;
		// Find Length of File
		let file_length = f.metadata()?.len();

		// Decide on Reasonable Buffer Size
		let buf_len = file_length.min(1_000_000) as usize;
		let mut buffer = BufReader::with_capacity(buf_len, f);

		loop {
			// Get Chunk of File
			let part = buffer.fill_buf()?;

			// If Chunk Empty, EOF Reached
			if part.is_empty() {
				break;
			}
			// Add Chunk to Hasher
			md5_context.consume(part);

			// Tell Buffer Chunk Was Consumed
			let part_len = part.len();
			buffer.consume(part_len);
		}

		// Finalize md5.context + Put Into Digest
		let md5_digest = md5_context.compute();
		println!("##==>> MD5 Hash Value: {:x}", md5_digest);
		Ok(md5_digest.to_vec())
	}
}

impl Hasher for Sha3_256Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut hasher = sha3::Sha3_256::new();
		let mut f = File::open(file)?;

		// Read Entire File
		// const SIXY_FOUR_KB: usize = 65536;
		const ONE_MB: usize = 1048576;
		let mut buffer: [u8; ONE_MB] = [0; ONE_MB];

		loop {
			println!("Something's wonky about this.");
			let bytes_read = f.read(&mut buffer)?;
			if bytes_read == 0 { break; }

			// Hash File String
			hasher.update(&buffer[..bytes_read]);
		}

		// Finalize Hasher Object and Put Into Vec
		let hash = hasher.finalize();
		println!("##==>> Sha3-256 Hash Value: {:x}", hash);
		Ok(hash.to_vec())
	}
}

impl Hasher for Sha3_384Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut hasher = sha3::Sha3_384::new();
		let f = File::open(file)?;
		// Find Length of File
		let file_length = f.metadata()?.len();

		// Decide on Reasonable Buffer Size
		let buf_len = file_length.min(1_000_000) as usize;
		let mut buffer = BufReader::with_capacity(buf_len, f);

		loop {
			// Get Chunk of File
			let part = buffer.fill_buf()?;

			// If Chunk Empty, EOF Reached
			if part.is_empty() {
				break;
			}

			// Add Chunk to Hasher
			hasher.update(part);

			// Tell Buffer Chunk Was Consumed
			let part_len = part.len();
			buffer.consume(part_len);
		}

		// Finalize Hasher Object and Put Into Vec
		let hash = hasher.finalize();
		println!("##==>> Sha3-384 Hash Value: {:x}", hash);
		Ok(hash.to_vec())
	}
}

impl Hasher for Sha3_512Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut hasher = sha3::Sha3_512::new();
		let f = File::open(file)?;
		// Find Length of File
		let file_length = f.metadata()?.len();

		// Decide on Reasonable Buffer Size
		let buf_len = file_length.min(1_000_000) as usize;
		let mut buffer = BufReader::with_capacity(buf_len, f);

		loop {
			// Get Chunk of File
			let part = buffer.fill_buf()?;

			// If Chunk Empty, EOF Reached
			if part.is_empty() {
				break;
			}

			// Add Chunk to Hasher
			hasher.update(part);

			// Tell Buffer Chunk Was Consumed
			let part_len = part.len();
			buffer.consume(part_len);
		}

		// Finalize Hasher Object and Put Into Vec
		let hash = hasher.finalize();
		println!("##==>> Sha3-512 Hash Value: {:x}", hash);
		Ok(hash.to_vec())
	}
}

// Determine Which Hashing Algorithm to Use Depending
// on User Selection, Otherwise Default to Sha3-256
// Then Hash the Given File
pub fn hash_file(hash_type: HashType, path: PathBuf) -> io::Result<Vec<u8>> {
	let file = PathBuf::from(path.as_path());

	let hasher: Box<dyn Hasher> = hash_type.into();
	hasher.hash(file)
}
