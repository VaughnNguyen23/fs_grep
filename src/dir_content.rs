use std::fs::read_dir;
use std::path::PathBuf;

pub struct DirContents {
    pub dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub child_dirs: Vec<PathBuf>,
    pub etc: Vec<PathBuf>,
}

impl DirContents {
    pub fn new(dir: PathBuf) -> DirContents {
        DirContents {
            dir,
            files: Vec::new(),
            child_dirs: Vec::new(),
            etc: Vec::new(),
        }
    }

    pub fn expand_path(&mut self) -> Result<(), std::io::Error> {
        let dir_contents = read_dir(&self.dir)?;
        for contents in dir_contents {
            let content = contents?;
            if content.path().is_file() {
                self.files.push(content.path());
            } else if content.path().is_dir() {
                self.child_dirs.push(content.path());
            } else {
                self.etc.push(content.path());
                continue;
            }
        }

        Ok(())
    }

    pub fn print_dir_contents(&self) {
        println!("Contents of {:?}", self.dir);
        for file in &self.files {
            println!("--f{:?}", file);
        }
        for dir in &self.child_dirs {
            println!("-d {:?}", dir);
        }
        for etc in &self.etc {
            println!("-f{:?}", etc);
        }
    }
}
