use std::io;
use std::fs;
use std::path::Path;
use bytesize::ByteSize;
use spinners::{Spinner, Spinners};
use walkdir::WalkDir;

pub fn get_file_size(path: &Path) -> io::Result<()> {
    let file_metadata = fs::metadata(path)?;

    if file_metadata.is_dir() {
        let mut files_read = 0;

        println!("##==> Path '{}' Points to a Directory.", path.display());
        println!("##==> Calculating Size of Directory");

        let mut spinner = Spinner::new(Spinners::Arc, String::new());
        let total_size = WalkDir::new(path)
            .min_depth(1)
            .max_depth(100)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .fold(0, |acc, m| acc + m.len());

        for _entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            // println!("{}", entry.path().display());
            files_read += 1;
        }
        spinner.stop_with_symbol("🗸");

        println!("##==>> Number of Files Read: {}", files_read);
        println!("##==>> Directory Size: {}", ByteSize::b(total_size));
    } else if file_metadata.is_file() {
        println!("##==> Path '{}' Points to a File.", path.display());
        println!("##==> Calculating Size of File ...");
        let mut spinner = Spinner::new(Spinners::Arc, String::new());
        println!("##==>> Size of File: {}", ByteSize::b(file_metadata.len()));
        spinner.stop_with_symbol("🗸");
    } else {
        println!("##==>>> Warning! Idk wtf that is ...");
    }
    Ok(())
}
