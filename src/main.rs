use std::env;
use std::fs::{File, read_dir};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

struct FileContents {
    query: String,
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
    matches: Vec<String>,
}

impl FileContents {
    fn new(query: String, files: Vec<PathBuf>, dirs: Vec<PathBuf>) -> FileContents {
        FileContents {
            query,
            files,
            dirs,
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

// struct ParseDirs {
//     input_dir: PathBuf,
//     entries: Vec<PathBuf>,
//     dirs: Vec<PathBuf>,
//     files: Vec<PathBuf>,
// }

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
}

// impl ParseDirs {
//     fn new(input_dir: PathBuf) -> ParseDirs {
//         ParseDirs {
//             input_dir,
//             entries: Vec::new(),
//             dirs: Vec::new(),
//             files: Vec::new(),
//         }
//     }
//
//     fn expand_path(&self) -> Result<Vec<PathBuf>, std::io::Error> {
//         let mut entries: Vec<PathBuf> = Vec::new();
//         let dir_contents = read_dir(&self.input_dir)?;
//         for content in dir_contents {
//             let path = content?;
//             entries.push(path.path());
//         }
//         Ok(entries)
//     }
//
//     fn collect_dirs(&self) -> Vec<PathBuf> {
//         let mut dirs: Vec<PathBuf> = Vec::new();
//         for dir in self.entries.clone() {
//             if dir.is_dir() {
//                 dirs.push(dir);
//             }
//         }
//         dirs
//     }
//
//     fn collect_files(&self) -> Vec<PathBuf> {
//         let mut files: Vec<PathBuf> = Vec::new();
//         for file in self.entries.clone() {
//             if file.is_file() {
//                 files.push(file);
//             }
//         }
//         files
//     }
//
//     fn depth_search_files(&self) -> Result<Vec<PathBuf>, std::io::Error> {
//         let mut depth_dirs: Vec<PathBuf> = Vec::new();
//         for dirs in &self.dirs {
//             for dir in read_dir(dirs)? {
//                 depth_dirs.push(dir?.path());
//             }
//         }
//         Ok(depth_dirs)
//     }
//
//     fn print(&self) {
//         for dir in self.dirs.clone() {
//             println!("d- {dir:?}");
//         }
//         for file in self.files.clone() {
//             println!("f- {file:?}")
//         }
//     }
// }
fn main() {
    // let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().expect("Failed to get current directory");
    let mut dir_contents = DirContents::new(path);
    dir_contents.expand_path().unwrap();
    for file in dir_contents.files {
        println!("--f{:?}", file);
    }
    for dir in dir_contents.child_dirs {
        println!("-d {:?}", dir);
    }
    for etc in dir_contents.etc {
        println!("-f{:?}", etc);
    }
    // let mut grepper = ParseDirs::new(path);

    // grepper.entries = grepper.expand_path().expect("Failed to expand path");
    // grepper.dirs = grepper.collect_dirs();
    // grepper.files = grepper.collect_files();
    // grepper
    //     .depth_search_files()
    //     .expect("Failed to perform depth search for files");
    // grepper.print();
    //
    // if args.len() < 3 {
    //     eprintln!("usage: {} <query> <file1> <file2> ...", args[0]);
    //     std::process::exit(1);
    // }
    // let query = args[1].clone();

    // let mut file_contents = FileContents::new(query, grepper.files, grepper.dirs);
    //
    // if let Err(e) = file_contents.search_files() {
    //     eprintln!("application error: {}", e);
    //     std::process::exit(1);
    // }
    // println!("{:?}", file_contents.matches);
}
