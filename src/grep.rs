use crate::dir_content::DirContents;
use crate::file_contents::FileContents;
use std::result;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Grep {
    pub dir_contents: DirContents,
    pub file_contents: FileContents,
    pub matches: Vec<String>,
}

impl Grep {
    pub fn new(dir_contents: DirContents, file_contents: FileContents) -> Grep {
        Grep {
            dir_contents,
            file_contents,
            matches: Vec::new(),
        }
    }

    pub fn search(&mut self) {
        self.dir_contents.expand_path().unwrap();
        self.dir_contents.print_dir_contents();
        self.file_contents.parse_files();
        self.file_contents.print_matches();
    }

    pub fn mt_search(&mut self) -> result::Result<(), std::io::Error> {
        let query = self.file_contents.query.clone();
        let matches = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for dir in &self.dir_contents.child_dirs {
            let dir = dir.clone();
            let query = query.clone();
            let matches = Arc::clone(&matches);

            let handle = thread::spawn(move || {
                let mut dir_contents = DirContents::new(dir);
                if dir_contents.expand_path().is_ok() {
                    let mut file_contents = FileContents::new(query, dir_contents.files);
                    file_contents.parse_files();
                    let mut matches_lock = matches.lock().unwrap();
                    matches_lock.extend(file_contents.matches);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_matches = Arc::try_unwrap(matches).unwrap().into_inner().unwrap();
        for m in final_matches {
            println!("{}", m);
        }

        Ok(())
    }
}
