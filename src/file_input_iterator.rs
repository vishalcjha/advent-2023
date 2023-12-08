#![allow(dead_code)]
use std::{fs::read_to_string, path::PathBuf};

pub(super) struct FileContent(pub String);
impl FileContent {
    pub fn new(file_name: impl Into<String>) -> Self {
        let file_name = file_name.into();
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(format!("src/input/{}", file_name));
        let file_content = read_to_string(format!("{}", d.display())).unwrap();
        FileContent(file_content)
    }
}
