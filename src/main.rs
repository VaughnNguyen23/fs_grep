use std::env;
use std::fs::{File, read_dir};
use std::io::{self, BufRead, BufReader};
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

    fn search_files(&mut self) -> io::Result<()> {
        for file in &self.files {
            let file_to_read = File::open(file)?;
            let reader = BufReader::new(file_to_read);

            for (index, line) in reader.lines().enumerate() {
                let line = line?;
                if line.contains(&self.query) {
                    self.matches.push(line.clone());
                    println!("{}:{}: {}", file.display(), index + 1, line);
                }
            }
        }

        Ok(())
    }
}

struct ParseDirs {
    input_dir: PathBuf,
    entries: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
}
impl ParseDirs {
    fn new(input_dir: PathBuf) -> ParseDirs {
        ParseDirs {
            input_dir,
            entries: Vec::new(),
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    fn expand_path(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut entries: Vec<PathBuf> = Vec::new();
        let dir_contents = read_dir(&self.input_dir)?;
        for content in dir_contents {
            let path = content?;
            entries.push(path.path());
        }
        Ok(entries)
    }

    fn collect_dirs(&self) -> Vec<PathBuf> {
        let mut dirs: Vec<PathBuf> = Vec::new();
        for dir in self.entries.clone() {
            if dir.is_dir() {
                dirs.push(dir);
            }
        }
        dirs
    }

    fn collect_files(&self) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = Vec::new();
        for file in self.entries.clone() {
            if file.is_file() {
                files.push(file);
            }
        }
        files
    }

    fn depth_search_files(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut depth_dirs: Vec<PathBuf> = Vec::new();
        for dirs in &self.dirs {
            for dir in read_dir(dirs)? {
                depth_dirs.push(dir?.path());
            }
        }
        Ok(depth_dirs)
    }

    fn print(&self) {
        for dir in self.dirs.clone() {
            println!("d- {dir:?}");
        }
        for file in self.files.clone() {
            println!("f- {file:?}")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().expect("Failed to get current directory");
    let mut grepper = ParseDirs::new(path);

    grepper.entries = grepper.expand_path().expect("Failed to expand path");
    grepper.dirs = grepper.collect_dirs();
    grepper.files = grepper.collect_files();
    grepper
        .depth_search_files()
        .expect("Failed to perform depth search for files");
    grepper.print();

    // if args.len() < 3 {
    //     eprintln!("usage: {} <query> <file1> <file2> ...", args[0]);
    //     std::process::exit(1);
    // }

    let query = args[1].clone();

    let mut file_contents = FileContents::new(query, grepper.files);

    if let Err(e) = file_contents.search_files() {
        eprintln!("application error: {}", e);
        std::process::exit(1);
    }
    println!("{:?}", file_contents.matches);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_file_contents_search() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "This is a test file.\nAnother line with test query.").unwrap();

        let query = "test".to_string();
        let files = vec![file_path];
        let mut file_contents = FileContents::new(query, files);

        file_contents.search_files().unwrap();
        assert_eq!(file_contents.matches.len(), 2);
    }

    #[test]
    fn test_parse_dirs() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("subdir");
        fs::create_dir(&subdir_path).unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let mut parser = ParseDirs::new(dir.path().to_path_buf());
        parser.entries = parser.expand_path().unwrap();
        parser.dirs = parser.collect_dirs();
        parser.files = parser.collect_files();

        assert_eq!(parser.dirs.len(), 1);
        assert_eq!(parser.files.len(), 1);
    }
}
