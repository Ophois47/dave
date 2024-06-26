use age::{DecryptError, EncryptError};
use age::secrecy::Secret;
use std::fs::File;
use std::io::{
	self,
	Error,
	ErrorKind,
	Read,
	Write,
};
use std::path::Path;

pub fn dave_encrypt(passphrase: &str, path: &Path) -> io::Result<Vec<u8>> {
	let mut file = File::open(path)?;
	let mut buffer = vec![];
	file.read_to_end(&mut buffer)?;
	let plain_text = String::from_utf8_lossy(&buffer);

	// Encrypt the plaintext to a ciphertext using given passphrase
	let encrypted = {
		let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));
		let mut encrypted = vec![];
		let mut writer = match encryptor.wrap_output(&mut encrypted) {
			Ok(writer) => writer,
			Err(EncryptError::Io(_)) => {
				return Err(Error::new(ErrorKind::Other, "IO Error"))
			},
			_ => unreachable!(),
		};
		writer.write_all(plain_text.as_bytes())?;
		writer.finish()?;
		encrypted
	};

	let encrypted_file_name = format!("{}_encrypted", path.display());
	let mut encrypted_file = File::create(encrypted_file_name)?;
	encrypted_file.write_all(&encrypted)?;

	Ok(encrypted)
}

pub fn dave_decrypt(passphrase: &str, path: &Path) -> io::Result<Vec<u8>> {
	let mut file = File::open(path)?;
	let mut buffer = vec![];
	file.read_to_end(&mut buffer)?;

	// Decrypt ciphertext to plaintext again using same passphrase
	let decrypted = {
		let decryptor = match age::Decryptor::new(&buffer[..]) {
			Ok(age::Decryptor::Passphrase(d)) => d,
			Err(DecryptError::InvalidHeader) => {
				return Err(Error::new(ErrorKind::Other, "File Not Encrypted"))
			},
			_ => unreachable!(),
		};

		let mut decrypted = vec![];
		let mut reader = match decryptor.decrypt(&Secret::new(passphrase.to_owned()), None) {
			Ok(reader) => reader,
			Err(DecryptError::DecryptionFailed) => {
				return Err(Error::new(ErrorKind::Other, "Incorrect Passphrase"))
			},
			_ => unreachable!(),
		};
		if let Err(error) = reader.read_to_end(&mut decrypted) {
			eprintln!("##==>>>> ERROR: {}", error);
		}
		decrypted
	};

	Ok(decrypted)
}
