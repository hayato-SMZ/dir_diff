use core::panic;
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, Read};

use std::path::Path;
pub struct FileInfomation {
    pub path: String,
    pub full_path: String,
    pub path_hash: String,
    pub file_hash: String,
    pub compared: bool,
}

impl Default for FileInfomation {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            full_path: "".to_string(),
            path_hash: "".to_string(),
            file_hash: "".to_string(),
            compared: false,
        }
    }
}

impl FileInfomation {
    pub fn new() -> FileInfomation {
        Default::default()
    }

    pub fn set_path(&mut self, base_path: &String, full_path: &str) {
        self.full_path = full_path.to_string();
        self.path = full_path.replace(base_path, "");
        let mut path_hasher = Sha256::new();
        path_hasher.update(&self.path);
        self.path_hash = format!("{:X}", path_hasher.finalize());
    }

    pub fn set_file_hash(&mut self, hash: String) {
        self.file_hash = hash;
        // let target_path = Path::new(&self.full_path);
        // self.file_hash = format!("{:X}", Self::calculate_hash(target_path));
    }

    pub fn compare(&mut self, target_hash: String) -> bool {
        // let full = Path::new(&target_file);
        // let target_hash = format!("{:X}", Self::calculate_hash(full));
        self.compared = true;
        if target_hash == self.file_hash {
            return true;
        }
        false
    }
}

pub fn calculate_hash(path_string: &String) -> String {
    //u64 {
    let file_path = Path::new(path_string);
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(why) => panic!("can't open {}", why),
    };
    let mut reader = BufReader::new(file);
    let mut hasher = DefaultHasher::new();
    let mut buffer = [0; 1024];
    while let Ok(n) = reader.read(&mut buffer) {
        hasher.write(&buffer);
        if n == 0 {
            break;
        }
    }
    format!("{:X}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use crate::diff_lib;
    use std::env;
    #[test]
    fn test_set_path() {
        let mut info = diff_lib::file_infomation::FileInfomation::new();
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("current is not found"),
        };
        current.push("test");
        current.push("source");
        let mut current_file = current.clone();
        current_file.push("test.txt");
        info.set_path(
            &format!("{}", current.display()),
            &format!("{}", current_file.display()),
        );
        assert_eq!(info.path, "\\test.txt");
        assert_eq!(info.file_hash, "");
        let file_hash =
            diff_lib::file_infomation::calculate_hash(&format!("{}", current_file.display()));
        assert_eq!(file_hash, "74EF815FC37249A1");
    }

    #[test]
    fn test_compare() {
        let mut info = diff_lib::file_infomation::FileInfomation::new();
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("current is not found"),
        };
        current.push("test");
        let mut current_file = current.clone();
        let mut current_dir = current.clone();
        let mut target_file = current.clone();
        let mut error_file = current.clone();
        current_file.push("source/test.txt");
        current_dir.push("source");
        target_file.push("target/test.txt");
        error_file.push("target/word_sample.docx");
        println!("error  =>   {}", current_file.display());
        info.set_path(
            &format!("{}", current_dir.display()),
            &format!("{}", current_file.display()),
        );
        let file_hash =
            diff_lib::file_infomation::calculate_hash(&format!("{}", current_file.display()));
        info.set_file_hash(file_hash);
        assert_eq!(
            info.compare(diff_lib::file_infomation::calculate_hash(&format!(
                "{}",
                target_file.display()
            ))),
            true
        );

        assert_eq!(
            info.compare(diff_lib::file_infomation::calculate_hash(&format!(
                "{}",
                error_file.display()
            ))),
            false
        );
    }
}
