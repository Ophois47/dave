use sha2::Digest;
use std::error::Error;
use std::io;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

// Possible Hashing Algorithms
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum HashType {
	Md5,
	Sha256,
	Sha384,
	Sha512
}

impl Into<Box<dyn Hasher>> for HashType {
	fn into(self) -> Box<dyn Hasher> {
		match self {
			HashType::Md5 		=> Box::new(Md5Hash {}),
			HashType::Sha256 	=> Box::new(Sha256Hash {}),
			HashType::Sha384 	=> Box::new(Sha384Hash {}),
			HashType::Sha512 	=> Box::new(Sha512Hash {}),
		}
	}
}

impl fmt::Display for HashType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			HashType::Md5 => write!(f, "MD5"),
			HashType::Sha256 => write!(f, "SHA_256"),
			HashType::Sha384 => write!(f, "SHA_384"),
			HashType::Sha512 => write!(f, "SHA_512"),
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
			"sha-384" 	=> HashType::Sha384,
			"sha-512" 	=> HashType::Sha512,
			_ 			=> HashType::Sha256,
		};
		Ok(gotten_hash_type)
	}
}

pub trait Hasher {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>>;
}

pub struct Md5Hash;
pub struct Sha256Hash;
pub struct Sha384Hash;
pub struct Sha512Hash;

impl Hasher for Md5Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut md5_context = md5::Context::new();
		match std::fs::read(file) {
			Ok(bytes) => {
				md5_context.consume(&bytes);
			},
			Err(error) => {
				if error.kind() == std::io::ErrorKind::PermissionDenied {
					eprintln!("Please Run Again With Appropriate Permissions");
				}
				panic!("{}", error);
			}
		}

		let md5_digest = md5_context.compute();
		Ok(md5_digest.to_vec())
	}
}

impl Hasher for Sha256Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut hasher = sha2::Sha256::new();
		match std::fs::read(file) {
			Ok(bytes) => {
				hasher.update(&bytes);
			},
			Err(error) => {
				if error.kind() == std::io::ErrorKind::PermissionDenied {
					eprintln!("Please Run Again With Appropriate Permissions");
				}
				panic!("{}", error);
			}
		}

		let hash = hasher.finalize();
		Ok(hash.to_vec())
	}
}

impl Hasher for Sha384Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut hasher = sha2::Sha384::new();
		match std::fs::read(file) {
			Ok(bytes) => {
				hasher.update(&bytes);
			},
			Err(error) => {
				if error.kind() == std::io::ErrorKind::PermissionDenied {
					eprintln!("Please Run Again With Appropriate Permissions");
				}
				panic!("{}", error);
			}
		}

		let hash = hasher.finalize();
		Ok(hash.to_vec())
	}
}

impl Hasher for Sha512Hash {
	fn hash(&self, file: PathBuf) -> io::Result<Vec<u8>> {
		let mut hasher = sha2::Sha512::new();
		match std::fs::read(file) {
			Ok(bytes) => {
				hasher.update(&bytes);
			},
			Err(error) => {
				if error.kind() == std::io::ErrorKind::PermissionDenied {
					eprintln!("Please Run Again With Appropriate Permissions");
				}
				panic!("{}", error);
			}
		}

		let hash = hasher.finalize();
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
