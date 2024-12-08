// src/content.rs
// The "content" module contains functions for creating the table of contents and individual story files. The create_table_of_contents function generates a table of contents file based on the extracted stories. The create_story_files function creates individual story files based on the extracted contents. These functions are used in the main shrive function to process the input file and generate the output files.
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

pub fn create_table_of_contents(path: PathBuf, stories: Vec<String>) -> io::Result<()> {
    // Get the folder name from the path
    let folder_name = path.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid folder name"))?
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid folder name"))?;
    println!("Folder name: {}", folder_name);

    // Create the table of contents file path
    let toc_path = path.join("table_of_contents.md");
    println!("Table of Contents path: {:?}", toc_path);

    // Create the table of contents file
    let mut toc_file = fs::File::create(&toc_path)?;
    println!("Table of Contents file created successfully");

    // Write the folder name as a header with a blank line after it
    writeln!(toc_file, "# {}\n", folder_name)?;

    // Write the table of contents to the file with blank lines around the list
    writeln!(toc_file, "")?;
    for story in stories {
        writeln!(toc_file, "> - {}", story)?;
    }
    writeln!(toc_file, "")?;
    println!("Table of Contents written successfully");

    Ok(())
}

pub fn create_story_files(path: PathBuf, titles: Vec<String>, file_path: &Path) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let content: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    for title in &titles {
        let mut occurrences = 0;
        let mut story_text = String::new();
        let mut in_story = false;
        let mut last_line_blank = false;

        for line in &content {
            if line.trim() == title {
                occurrences += 1;
                if occurrences == 2 {
                    in_story = true;
                    continue;
                }
            }

            if in_story {
                if titles.contains(&line.trim().to_string()) {
                    break;
                }
                if line.trim().is_empty() {
                    if !last_line_blank {
                        story_text.push_str(line);
                        story_text.push('\n');
                        last_line_blank = true;
                    }
                } else {
                    story_text.push_str(line);
                    story_text.push('\n');
                    last_line_blank = false;
                }
            }
        }

        if in_story {
            let file_name = format!("{}.md", title.replace(" ", "_").to_lowercase());
            let file_path = path.join(file_name);
            let mut file = fs::File::create(file_path)?;
            writeln!(file, "# {}\n", title)?;
            file.write_all(story_text.as_bytes())?;
        }
    }

    Ok(())
}