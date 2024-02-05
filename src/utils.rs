use std::io;
use std::fs;
use std::path::Path;
use bytesize::ByteSize;
use walkdir::WalkDir;

pub fn get_file_size(path: &Path) -> io::Result<()> {
    println!("Your Path is '{}'", path.display());

    let file_metadata = fs::metadata(path)?;
    if file_metadata.is_dir() {
        println!("Passed Path Points to a Directory.");
        println!("Exploring Contents ...\n");
        let mut files_read = 0;
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            println!("{}", entry.path().display());
            files_read += 1;
        }
        let total_size = WalkDir::new(path)
            .min_depth(1)
            .max_depth(100)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .fold(0, |acc, m| acc + m.len());

        println!("\n##==> Number of Files Read: {}", files_read);
        println!("##==> Directory Size: {}", ByteSize::b(total_size));
    } else if file_metadata.is_file() {
        println!("Passed Path Points to a File.");
        println!("Calculating Size of File ...");
        println!("Size of File: {}", ByteSize::b(file_metadata.len()));
    } else {
        println!("Idk wtf that is ...");
    }
    Ok(())
}

pub fn greeting(name: String) -> io::Result<()> {
    println!("Hey {}!", name);
    Ok(())
}
