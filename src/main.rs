use crate::dir_content::DirContents;
use crate::file_contents::FileContents;
use crate::grep::Grep;
use std::env;
mod dir_content;
mod file_contents;
mod grep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().expect("Failed to get current directory");
    let dir_contents = DirContents::new(path);
    let file_contents = FileContents::new(args[1].clone(), dir_contents.files.clone());
    let mut grep = Grep::new(dir_contents, file_contents);
    grep.search();
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, remove_dir_all, remove_file, write};
    use std::path::PathBuf;

    #[test]
    fn test_file_contents() {
        let query = "test".to_string();
        let file_path = PathBuf::from("test_file.txt");
        write(&file_path, "this is a test\nanother line").unwrap();
        let mut file_contents = FileContents::new(query, vec![file_path.clone()]);
        file_contents.parse_files();
        println!("Matches: {:?}", file_contents.matches);
        assert_eq!(file_contents.matches, vec!["this is a test"]);
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_dir_contents() {
        let dir_path = PathBuf::from("test_dir");
        create_dir_all(&dir_path).unwrap();
        let file_path = dir_path.join("test_file.txt");
        write(&file_path, "this is a test\nanother line").unwrap();
        let mut dir_contents = DirContents::new(dir_path.clone());
        dir_contents.expand_path().unwrap();
        println!("Files: {:?}", dir_contents.files);
        assert!(!dir_contents.files.is_empty());
        remove_file(file_path).unwrap();
        remove_dir_all(dir_path).unwrap();
    }

    #[test]
    fn test_grep() {
        let query = "test".to_string();
        let dir_path = PathBuf::from("test_dir");
        create_dir_all(&dir_path).unwrap();
        let file_path = dir_path.join("test_file.txt");
        write(&file_path, "this is a test\nanother line").unwrap();
        let mut dir_contents = DirContents::new(dir_path.clone());
        dir_contents.expand_path().unwrap();
        let file_contents = FileContents::new(query, dir_contents.files.clone());
        let mut grep = Grep::new(dir_contents, file_contents);
        grep.search();
        println!("Grep Matches: {:?}", grep.file_contents.matches);
        assert_eq!(grep.file_contents.matches, vec!["this is a test"]);
        remove_file(file_path).unwrap();
        remove_dir_all(dir_path).unwrap();
    }

    #[test]
    fn test_mt_search() {
        let query = "test".to_string();
        let dir_path = PathBuf::from("test_dir");
        create_dir_all(&dir_path).unwrap();
        let sub_dir_path = dir_path.join("sub_dir");
        create_dir_all(&sub_dir_path).unwrap();
        let file_path = sub_dir_path.join("test_file.txt");
        write(&file_path, "this is a test\nanother line").unwrap();
        let mut dir_contents = DirContents::new(dir_path.clone());
        dir_contents.expand_path().unwrap();
        let file_contents = FileContents::new(query, vec![]);
        let mut grep = Grep::new(dir_contents, file_contents);
        grep.mt_search().unwrap();
        remove_file(file_path).unwrap();
        remove_dir_all(sub_dir_path).unwrap();
        remove_dir_all(dir_path).unwrap();
    }
}
