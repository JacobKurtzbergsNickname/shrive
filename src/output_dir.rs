// src/output_dir.rs
// The module "output_dir" contains the function that creates a folder in the output directory based on the input file name.
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The function `create_folder_in_output_dir` takes a file_name, extracts the base name,
/// converts it to a folder name format, creates a folder in the output directory with that name, and
/// returns the path to the created folder.
/// 
/// Arguments:
/// 
/// * `input_file`: The function `create_folder_in_output_dir` takes file_name as `&str` as input,
/// processes it to create a folder name, creates a new directory in the "src/output" directory with
/// 
/// Returns:
/// 
/// The function `create_folder_in_output_dir` returns a `Result` containing a `PathBuf`. The `PathBuf`
/// represents the path to the newly created folder in the output directory.
pub fn create_folder_in_output_dir(file_name: &str) -> io::Result<PathBuf> {
    let base_name = file_name.trim_end_matches(".txt");
    println!("Base name: {}", base_name);

    let folder_name = build_folder_name(base_name);
    let output_dir = Path::new("src/output").join(folder_name);
    println!("Output directory: {:?}", output_dir);

    fs::create_dir_all(&output_dir)?;
    println!("Folder created successfully");

    Ok(output_dir)
}

fn build_folder_name(base_name: &str) -> String {
    let single_words = split_words(base_name);
    let folder_name = capitalize_words(single_words).join(" ");
    println!("Folder name: {}", folder_name);
    return folder_name;
}

fn split_words(base_name: &str) -> Vec<String> {
    base_name.replace("_", " ").split_whitespace().map(|s| s.to_string()).collect()
}

fn capitalize_words(words: Vec<String>) -> Vec<String> {
    words.iter().map(|word| capitalize_word(word)).collect()
}

fn capitalize_word(word: &str) -> String {
    let mut c = word.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
