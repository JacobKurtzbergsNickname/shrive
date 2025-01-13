// src/lib.rs
// A crate for extracting contents from a text file containing a story collection and creating a table of contents and individual story files.
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::Path;

mod output_dir;
mod content;
mod process;
pub use process::process_file_impl;

#[pyfunction]
fn process_file(input_file: &str) -> PyResult<()> {
    // Call the internal implementation function
    process_file_impl(Path::new(input_file)).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Error processing file: {}", e))
    })
}

#[pymodule]
fn shrive(_py: Python, m: &PyModule) -> PyResult<()> {
    // Register the `process_file` function with the Python module
    m.add_function(wrap_pyfunction!(process_file, m)?)?;
    Ok(())
}
