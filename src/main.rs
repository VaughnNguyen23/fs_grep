use std::fs::read_dir;
use std::path::PathBuf;


struct Grepper {
    input_dir: PathBuf,
    entries: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
    //file_types: Vec<String>
}
impl Grepper {
    fn expand_path(&self) -> Vec<PathBuf> {
        let mut entries: Vec<PathBuf> = Vec::new();
        let dir_contents = read_dir(self.input_dir.clone());
        for content in dir_contents.unwrap() {
            let path = content.unwrap();
            entries.push(path.path());
        }
        entries
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
    fn depth_search_files(&self) -> Vec<PathBuf> {
        let mut depth_dirs: Vec<PathBuf> = Vec::new();
        for dirs in self.dirs.clone() {
            for dir in read_dir(dirs).unwrap() {
                depth_dirs.push(dir.unwrap().path()); 
            } 
        }
        depth_dirs
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

fn main() {
    let path = PathBuf::from("/");
    let mut grep = Grepper {
        input_dir: path, 
        entries: Vec::new(), 
        dirs: Vec::new(), 
        files: Vec::new(), 
        //file_types: Vec::new()
    };
    let dir_contents = grep.expand_path();
    grep.entries = dir_contents;
    grep.dirs = grep.collect_dirs();
    grep.files = grep.collect_files();
    grep.dirs = grep.depth_search_files();
    //grep.entries.clear();
    grep.entries = grep.dirs.clone();
    grep.files = grep.collect_files();
    grep.dirs = grep.collect_dirs();
    // again
    grep.entries = grep.dirs.clone();
    grep.files = grep.collect_files();
    grep.dirs = grep.collect_dirs();

    grep.print();
}
