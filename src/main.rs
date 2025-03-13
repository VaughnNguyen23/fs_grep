use std::env;
use std::fs::{File, read_dir};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

struct FileContents {
    query: String,
    files: Vec<PathBuf>,
}

impl FileContents {
    fn new(query: String, files: Vec<PathBuf>) -> FileContents {
        FileContents { query, files }
    }

    fn search_files(&self) -> io::Result<()> {
        for file in &self.files {
            let file_to_read = File::open(file)?;
            let reader = BufReader::new(file_to_read);

            for (index, line) in reader.lines().enumerate() {
                let line = line?;
                if line.contains(&self.query) {
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
    let path = std::env::current_dir().expect("Failed to get current directory");
    let mut grepper = ParseDirs::new(path);

    grepper.entries = grepper.expand_path().expect("Failed to expand path");
    grepper.dirs = grepper.collect_dirs();
    grepper.files = grepper.collect_files();
    grepper
        .depth_search_files()
        .expect("Failed to perform depth search for files");
    grepper.print();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <query> <file1> <file2> ...", args[0]);
        std::process::exit(1);
    }

    let query = args[1].clone();

    let file_contents = FileContents::new(query, grepper.files);

    if let Err(e) = file_contents.search_files() {
        eprintln!("application error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::process::Command;
    use tempdir::TempDir;

    #[test]
    fn test_search_single_file() {
        let dir = TempDir::new("test").unwrap();
        let file_path = dir.path().join("testfile.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "This is a test file.").unwrap();
        writeln!(file, "It contains some test data.").unwrap();
        writeln!(file, "Searching for a string.").unwrap();

        let output = Command::new(env!("CARGO_BIN_EXE_your_binary_name"))
            .arg("test")
            .arg(file_path.to_str().unwrap())
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("1: This is a test file."));
    }

    #[test]
    fn test_search_multiple_files() {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("testfile1.txt");
        let file_path2 = dir.path().join("testfile2.txt");
        let mut file1 = File::create(&file_path1).unwrap();
        let mut file2 = File::create(&file_path2).unwrap();
        writeln!(file1, "This is the first test file.").unwrap();
        writeln!(file2, "This is the second test file.").unwrap();
        writeln!(file2, "It contains the search string.").unwrap();

        let output = Command::new(env!("main.rs"))
            .arg("search")
            .arg(file_path1.to_str().unwrap())
            .arg(file_path2.to_str().unwrap())
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("2: It contains the search string."));
    }
}
