// src/lib.rs
// "shrive" is a command-line tool that processes text files and generates a table of contents and individual story files. The tool reads a text file, extracts the table of contents, and creates a folder in the output directory based on the file name. The folder name is derived from the input file name, with spaces and underscores replaced by spaces and the first letter of each word capitalized. The tool then generates a table of contents file and individual story files based on the extracted contents.
use rfd::FileDialog;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Internal modules
mod output_dir;
use output_dir::create_folder_in_output_dir;

mod content;
use content::{create_table_of_contents, create_story_files};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file dialog to choose the input file
    let input_file = FileDialog::new()
        .add_filter("Text files", &["txt"])
        .set_directory(".")
        .pick_file();

    // Call the shrive function with the input file
    if let Some(ref path) = input_file {
        shrive(path)?;
    } else {
        println!("No input file provided.");
    }

    Ok(())
}

fn shrive(input_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a folder in the output directory
    let path = create_folder_in_output_dir(input_file)?;
    let stories = extract_contents(input_file)?;
    
    // Clone the path before passing it to create_table_of_contents
    create_table_of_contents(path.clone(), stories.clone())?;
    
    // Use the original path for create_story_files
    create_story_files(path, stories, input_file)?;
    
    Ok(())
}

fn extract_contents(file_path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut titles = Vec::new();
    let mut in_contents = false;

    for line in reader.lines() {
        let line = line?;
        if line.trim() == "Contents" {
            in_contents = true;
            continue;
        }
        if in_contents {
            if line.trim().is_empty() {
                continue;
            }
            if titles.contains(&line.trim().to_string()) {
                break;
            }
            titles.push(line.trim().to_string());
        }
    }

    Ok(titles)
}