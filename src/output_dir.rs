use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn create_folder_in_output_dir(input_file: &Path) -> io::Result<PathBuf> {
    // Convert the path to a string and print it
    let input_file_str = input_file.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"))?;
    println!("Selected file: {}", input_file_str);

    // Extract the file name
    let file_name = input_file.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;
    println!("File name: {}", file_name);

    // Remove the .txt extension
    let base_name = file_name.trim_end_matches(".txt");
    println!("Base name: {}", base_name);

    // Transform the casing
    let folder_name = base_name
        .replace("_", " ")
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    println!("Folder name: {}", folder_name);

    // Create the output directory path
    let output_dir = Path::new("src/output").join(folder_name);
    println!("Output directory: {:?}", output_dir);

    // Create the folder
    fs::create_dir_all(&output_dir)?;
    println!("Folder created successfully");

    Ok(output_dir)
}
