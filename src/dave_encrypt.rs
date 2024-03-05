use age::secrecy::Secret;
use std::io::{self, Read, Write};

pub fn dave_encrypt_decrypt(passphrase: String) -> io::Result<()> {
	// TODO: Read and Encrypt Files
	// let f = File::open(file.clone())?;
	let plain_text = b"Hey Dave!";

	// Encrypt the plaintext to a ciphertext using given passphrase
	let encrypted = {
		println!("##==> INFO! Now Encrypting ...");
		let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));
		let mut encrypted = vec![];
		let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
		writer.write_all(plain_text).unwrap();
		writer.finish().unwrap();

		encrypted
	};

	// Decrypt ciphertext to plaintext again using same passphrase
	let _decrypted = {
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
	println!("##==> INFO! Encryption/Decryption Completed");

	Ok(())
}
