use std::env;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

struct FileContents {
    query: String,
    files: Vec<PathBuf>,
    matches: Vec<String>,
}

impl FileContents {
    fn new(query: String, files: Vec<PathBuf>) -> FileContents {
        FileContents {
            query,
            files,
            matches: Vec::new(),
        }
    }
    // fn clear_files(&mut self) {
    //     self.files.clear();
    // }

    fn parse_files(&mut self) {
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

    fn print_matches(&self) {
        for line in &self.matches {
            println!("{}", line);
        }
    }
}

struct DirContents {
    dir: PathBuf,
    files: Vec<PathBuf>,
    child_dirs: Vec<PathBuf>,
    etc: Vec<PathBuf>,
}

impl DirContents {
    fn new(dir: PathBuf) -> DirContents {
        DirContents {
            dir,
            files: Vec::new(),
            child_dirs: Vec::new(),
            etc: Vec::new(),
        }
    }

    fn expand_path(&mut self) -> Result<(), std::io::Error> {
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
    fn print_dir_contents(&self) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().expect("Failed to get current directory");
    let mut dir_contents = DirContents::new(path);
    let mut file_contents = FileContents::new(args[1].clone(), dir_contents.files.clone());
    dir_contents.expand_path().unwrap();
    dir_contents.print_dir_contents();
    file_contents.parse_files();
    file_contents.print_matches();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_file_contents_new() {
        let query = String::from("test");
        let files = vec![PathBuf::from("test_file.txt")];
        let fc = FileContents::new(query.clone(), files.clone());
        assert_eq!(fc.query, query);
        assert_eq!(fc.files, files);
        assert!(fc.matches.is_empty());
    }

    #[test]
    fn test_file_contents_parse_files() {
        let query = String::from("test");
        let file_path = Path::new("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "this is a test line").unwrap();
        writeln!(file, "this is another line").unwrap();

        let files = vec![file_path.to_path_buf()];
        let mut fc = FileContents::new(query.clone(), files);
        fc.parse_files();
        assert_eq!(fc.matches.len(), 1);
        assert!(fc.matches[0].contains(&query));

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_dir_contents_new() {
        let dir = PathBuf::from("test_dir");
        let dc = DirContents::new(dir.clone());
        assert_eq!(dc.dir, dir);
        assert!(dc.files.is_empty());
        assert!(dc.child_dirs.is_empty());
        assert!(dc.etc.is_empty());
    }

    #[test]
    fn test_dir_contents_expand_path() {
        let dir_path = Path::new("test_dir");
        fs::create_dir(dir_path).unwrap();
        let file_path = dir_path.join("test_file.txt");
        File::create(&file_path).unwrap();

        let mut dc = DirContents::new(dir_path.to_path_buf());
        dc.expand_path().unwrap();
        assert_eq!(dc.files.len(), 1);
        assert!(dc.child_dirs.is_empty());
        assert!(dc.etc.is_empty());

        fs::remove_file(file_path).unwrap();
        fs::remove_dir(dir_path).unwrap();
    }
}
