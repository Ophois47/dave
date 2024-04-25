use std::fs;
use std::io;
use std::path::PathBuf;
use file_format::FileFormat;

fn parse_file(file: PathBuf) -> io::Result<()> {
	let fmt = FileFormat::from_file(file.clone())?;
	println!("##==> File: '{}'", file.display());
	println!("##==>> {} : {}", fmt.name(), fmt.short_name().unwrap_or(" "));
	match fs::read_to_string(file) {
		Ok(contents) => {
			println!("##==>> Contents of File:\n");
			println!("--------------------------------------------------------------------");
			println!("{}", contents);
			println!("--------------------------------------------------------------------");
		},
		_ => return Ok(()),
	}
	Ok(())
}

pub fn parse_handle_file(filename: PathBuf) -> io::Result<()> {
	let canonical_file = filename.canonicalize()?.clone();
	let metadata = fs::metadata(canonical_file)?;
	if metadata.is_file() {
		if let Err(error) = parse_file(filename) {
			eprintln!("##==>>>> ERROR: {}", error);
		}
	} else if metadata.is_dir() {
		println!("##==> Directory: '{}'", filename.display());
	}
	Ok(())
}
