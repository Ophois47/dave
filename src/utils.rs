use std::io;
use std::fs;
use std::path::Path;
use bytesize::ByteSize;
use colored::*;
use rand::Rng;
use spinners::{Spinner, Spinners};
use walkdir::WalkDir;

pub fn generate_random_number() -> u16 {
    let mut rng = rand::thread_rng();
    let random_value: u16 = rng.gen_range(1..10);
    random_value
}

pub fn get_file_size(path: &Path) -> io::Result<()> {
    let file_metadata = fs::metadata(path)?;
    let stop_symbol = format!("{}", "🗸".green());

    if file_metadata.is_dir() {
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

        spinner.stop_with_symbol(&stop_symbol);
        println!("##==>> Directory Size: {}", ByteSize::b(total_size));
    } else if file_metadata.is_file() {
        println!("##==> Path '{}' Points to a File.", path.display());
        println!("##==> Calculating Size of File ...");

        let mut spinner = Spinner::new(Spinners::Arc, String::new());
        println!("##==>> Size of File: {}", ByteSize::b(file_metadata.len()));
        spinner.stop_with_symbol(&stop_symbol);
    } else {
        println!("{}", "##==>>> Warning! Idk WTF that is ... Where did you even find it?".red());
    }
    Ok(())
}
