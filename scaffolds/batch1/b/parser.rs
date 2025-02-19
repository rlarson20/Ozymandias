// parser.rs
use std::fs::File;
use std::io::{Error, Read};

pub enum FileType {
    Markdown,
    Text,
    Pdf,
    // Add more file types as needed
}

pub struct Parser {
    file_path: String,
    file_type: FileType,
}

impl Parser {
    pub fn new(file_path: &str, file_type: FileType) -> Parser {
        Parser {
            file_path: file_path.to_string(),
            file_type,
        }
    }

    pub fn parse(&self) -> Result<String, Error> {
        // TO DO: implement parsing logic based on file type
        match self.file_type {
            FileType::Markdown => {
                // Parse markdown file
                Ok("".to_string())
            }
            FileType::Text => {
                // Parse text file
                Ok("".to_string())
            }
            FileType::Pdf => {
                // Parse pdf file
                Ok("".to_string())
            }
        }
    }

    pub fn read_file(&self) -> Result<String, Error> {
        let mut file = File::open(&self.file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
