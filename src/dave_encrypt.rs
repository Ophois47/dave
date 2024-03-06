use age::secrecy::Secret;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn dave_encrypt(passphrase: String, file: &Path) -> io::Result<Vec<u8>> {
	let mut file = File::open(file)?;
	let mut buffer = vec![];
	file.read_to_end(&mut buffer)?;
	let plain_text = String::from_utf8_lossy(&buffer);

	// Encrypt the plaintext to a ciphertext using given passphrase
	let encrypted = {
		println!("##==> INFO! Now Encrypting ...");
		let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));
		let mut encrypted = vec![];
		let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
		writer.write_all(plain_text.as_bytes()).unwrap();
		writer.finish()?;
		encrypted
	};

	Ok(encrypted)
}

pub fn dave_decrypt(passphrase: &str, encrypted: &[u8]) -> io::Result<Vec<u8>> {
	// Decrypt ciphertext to plaintext again using same passphrase
	let decrypted = {
		println!("##==> INFO! Now Decrypting ...");
		let decryptor = match age::Decryptor::new(&encrypted[..]).unwrap() {
			age::Decryptor::Passphrase(d) => d,
			_ => unreachable!(),
		};

		let mut decrypted = vec![];
		let mut reader = decryptor.decrypt(&Secret::new(passphrase.to_owned()), None).unwrap();
		if let Err(error) = reader.read_to_end(&mut decrypted) {
			eprintln!("##==>>>> ERROR: {}", error);
		}
		decrypted
	};

	Ok(decrypted)
}
