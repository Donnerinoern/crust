use std::{error::Error, path::PathBuf};

pub struct Errors {
    path_errors: Vec<String>,
    errors: Vec<String>
}

impl Errors {
    pub fn new() -> Errors {
        Errors {
            path_errors: Vec::new(),
            errors: Vec::new()
        }
    }
    pub fn add_path_error() {
        
    }
}
