use std::fs::{File, read_dir};
use std::io::Write;
use std::path::PathBuf;

struct FileContents {
    file_name: PathBuf,
    file_contents: String,
    all_files: Vec<PathBuf>,
}

struct Grepper {
    input_dir: PathBuf,
    entries: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
    // grep_files: FileContents,
    //file_types: Vec<String>
}

impl FileContents {
    fn new(file_name: PathBuf, file_contents: String) -> FileContents {
        FileContents {
            file_name,
            file_contents,
            all_files: Vec::new(),
        }
    }

    fn read_file(&mut self) -> Result<(), std::io::Error> {
        let contents = std::fs::read_to_string(&self.file_name)?;
        self.file_contents = contents;
        Ok(())
    }

    fn write_file(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.file_name)?;
        file.write_all(self.file_contents.as_bytes())?;
        Ok(())
    }

    fn read_multiple_files(&self) -> Result<(), std::io::Error> {
        let mut file_contents: Vec<FileContents> = Vec::new();
        for file in self.all_files.clone() {
            let contents = std::fs::read_to_string(&file)?;
            file_contents.push(FileContents::new(file, contents));
        }
        Ok(())
    }

    fn search_whl_str(&self) {
        todo!();
    }
}

impl Grepper {
    fn new(input_dir: PathBuf) -> Grepper {
        Grepper {
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
    //fn get_file_types() {
    //    let mut file_ext = String::new();
    //    for char in "string.txt".chars() {
    //        if char == '.' {
    //            file_ext.push(char);
    //        }
    //        file_ext.push(char);
    //    }
    //    println!("{file_ext}")
    //}
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
    let mut grepper = Grepper::new(path);

    grepper.entries = grepper.expand_path().expect("Failed to expand path");
    grepper.dirs = grepper.collect_dirs();
    grepper.files = grepper.collect_files();
    grepper
        .depth_search_files()
        .expect("Failed to perform depth search for files");

    for file in &grepper.files {
        let mut file_contents = FileContents::new(file.clone(), String::new());
        file_contents.read_file().expect("Failed to read file");

        println!(
            "File: {:?}\nContents:\n{}",
            file_contents.file_name, file_contents.file_contents
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    // use std::path::PathBuf;

    #[test]
    fn test_grepper() -> Result<(), std::io::Error> {
        // Setup: create a temporary directory and files
        let temp_dir = std::env::temp_dir().join("grepper_test");
        fs::create_dir_all(&temp_dir)?;

        let file_path = temp_dir.join("test_file.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "Hello, world!")?;

        // Initialize Grepper
        let mut grepper = Grepper::new(temp_dir.clone());

        // Test expand_path
        grepper.entries = grepper.expand_path()?;
        grepper.dirs = grepper.collect_dirs();
        grepper.files = grepper.collect_files();

        assert!(grepper.files.contains(&file_path));

        // Test reading file contents
        for file in &grepper.files {
            let mut file_contents = FileContents::new(file.clone(), String::new());
            file_contents.read_file()?;
            assert_eq!(file_contents.file_contents, "Hello, world!\n");
        }

        // Cleanup
        fs::remove_file(file_path)?;
        fs::remove_dir(temp_dir)?;

        Ok(())
    }
}
