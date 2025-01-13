use std::path::Path;

use crate::{
    content::{
        create_story_files, 
        create_table_of_contents,
        extract_contents,
        extract_file_name
    }, 
    output_dir::create_folder_in_output_dir
};

pub fn process_file_impl(input_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Extract the file name from the input path and convert it to a string
    let file_name = extract_file_name(input_file)?;

    // Create a folder in the output directory
    let path = create_folder_in_output_dir(file_name)?;
    let stories = extract_contents(input_file)?;
    create_table_of_contents(path.clone(), stories.clone())?;
    create_story_files(path, stories, input_file)?;
    Ok(())
}

