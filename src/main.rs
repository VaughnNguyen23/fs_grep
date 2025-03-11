use std::fs::read_dir;
use std::path::PathBuf;

struct FileContents {
    file_name: PathBuf,
    file_contents: String,
}

struct Grepper {
    input_dir: PathBuf,
    entries: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
    grep_files: FileContents,
    //file_types: Vec<String>
}

impl FileContents {
    fn new(file_name: PathBuf, file_contents: String) -> FileContents {
        FileContents {
            file_name,
            file_contents,
        }
    }
}

impl Grepper {
    fn read_contents(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut entries: Vec<PathBuf> = Vec::new();
        let dir_contents = read_dir(&self.input_dir)?;
        for content in dir_contents {
            let path = content?;
            entries.push(path.path());
        }
        Ok(entries)
    }

    fn new(input_dir: PathBuf) -> Grepper {
        Grepper {
            input_dir,
            entries: Vec::new(),
            dirs: Vec::new(),
            files: Vec::new(),
            grep_files: FileContents {
                file_name: PathBuf::new(),
                file_contents: String::new(),
            },
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
    let path = std::env::current_dir().unwrap();
    let mut grepper = Grepper::new(path);
    grepper.entries = grepper.expand_path().unwrap();
    grepper.dirs = grepper.collect_dirs();
    grepper.files = grepper.collect_files();
    grepper.depth_search_files().unwrap();
    grepper.print();

    //grep.entries.clear();
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
        let grepper = Grepper::new(temp_dir.clone());

        // Test expand_path
        let entries = grepper.expand_path()?;
        assert!(entries.contains(&file_path));

        // Cleanup
        fs::remove_file(file_path)?;
        fs::remove_dir(temp_dir)?;

        Ok(())
    }
}
