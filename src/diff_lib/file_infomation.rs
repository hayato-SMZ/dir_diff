use core::panic;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
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

    pub fn set_path(&mut self, base_path: String, full_path: String) {
        self.full_path = full_path.clone();
        self.path = full_path.replace(&base_path, "");
        let mut target_path = Path::new(&full_path);
        let mut file = match File::open(&target_path) {
            Ok(file) => file,
            Err(why) => panic!("can't open {}", why),
        };

        let mut s = String::new();
        let file_str = match file.read_to_string(&mut s) {
            Err(why) => panic!("cant read {}", why),
            Ok(file_str) => file_str,
        };
        let mut hasher = Sha256::new();
        hasher.update(s);
        self.file_hash = format!("{:X}", hasher.finalize());
        let mut path_hasher = Sha256::new();
        path_hasher.update(&self.path);
        self.path_hash = format!("{:X}", path_hasher.finalize());
    }

    pub fn get_path_hash(self) -> String {
        return self.path_hash;
    }

    pub fn get_path(self) -> String {
        return self.path;
    }

    pub fn compare(&mut self, target_file: String) -> bool {
        let full = Path::new(&target_file);
        let mut file = match File::open(target_file) {
            Ok(file) => file,
            Err(why) => panic!("can't open {}", why),
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("cant read {}", why),
            Ok(file_str) => file_str,
        };
        let mut hasher = Sha256::new();
        hasher.update(s);
        let target_hash: String = format!("{:X}", hasher.finalize());
        if &target_hash == &self.file_hash {
            self.compared = true;
            return true;
        }

        return false;
        // match filebinary {
        //     Ok(filedata) => {
        //         let mut hasher = Sha256::new();
        //         hasher.update(filedata);
        //         let target_hash: String = format!("{:X}", hasher.finalize());
        //         if &target_hash == &self.file_hash {
        //             self.compared = true;
        //             return true;
        //         } else {
        //             return false;
        //         }
        //     }
        //     Err(_) => return false,
        // };
    }
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
            format!("{}", current.display()),
            format!("{}", current_file.display()),
        );
        assert_eq!(info.path, "/test.txt");
        println!("filehash => {}", info.file_hash);
        assert_eq!(
            info.file_hash,
            "8A5EC8575E94A85847DB04ABFCC8BB82D1191D79790527EEC2254B7DB1E64172"
        );
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
            format!("{}", current_dir.display()),
            format!("{}", current_file.display()),
        );

        assert_eq!(info.compare(format!("{}", target_file.display())), true);

        assert_eq!(info.compare(format!("{}", error_file.display())), false);
    }
}
