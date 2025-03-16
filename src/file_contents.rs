use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct FileContents {
    pub query: String,
    pub files: Vec<PathBuf>,
    pub matches: Vec<String>,
}

impl FileContents {
    pub fn new(query: String, files: Vec<PathBuf>) -> FileContents {
        FileContents {
            query,
            files,
            matches: Vec::new(),
        }
    }
    // fn clear_files(&mut self) {
    //     self.files.clear();
    // }

    pub fn parse_files(&mut self) {
        for file in &self.files {
            let f = File::open(file).expect("Failed to open file");
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let line = line.expect("Failed to read line");
                if line.contains(&self.query) {
                    self.matches.push(line);
                }
            }
        }
    }

    pub fn print_matches(&self) {
        for line in &self.matches {
            println!("{}", line);
        }
    }
}
