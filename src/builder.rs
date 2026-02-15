use std::ops::Deref;
use std::fmt::Display;
use std::path::{Path, PathBuf};

pub struct AAMBuilder {
    buffer: String,
}

impl AAMBuilder {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) {
        if !self.buffer.is_empty() {
            self.buffer.push('\n');
        }
        self.buffer.push_str(line);
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        std::fs::write(path, self.buffer.as_bytes()).unwrap();
    }
}

impl Deref for AAMBuilder {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl Default for AAMBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for AAMBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}