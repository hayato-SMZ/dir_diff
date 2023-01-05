use super::file_infomation::FileInfomation;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
pub struct ComparsionSource {
    pub base_path: String,
    pub file_list: HashMap<String, FileInfomation>,
    pub compare_error: Vec<String>,
    pub compare_count: u32,
}

impl Default for ComparsionSource {
    fn default() -> Self {
        Self {
            base_path: "".to_string(),
            file_list: HashMap::new(),
            compare_error: Vec::new(),
            compare_count: 0,
        }
    }
}

impl ComparsionSource {
    pub fn new() -> ComparsionSource {
        Default::default()
    }

    pub fn set_base_path(&mut self, path: String) {
        if Path::new(&path).exists() {
            println!("path is exist");
            self.base_path = path.clone();
        } else {
            println!("path is not exitst");
            let mut current = match env::current_dir() {
                Ok(path) => path,
                Err(_) => panic!("current is not found"),
            };
            current.push(&path);
            self.base_path = current.to_str().unwrap().to_string();
        };
    }

    pub fn push_file_list(&mut self, file_path: &Path) {
        let mut file_items = FileInfomation::new();
        file_items.set_path(
            self.base_path.clone(),
            file_path.to_str().unwrap().to_string(),
        );
        self.file_list
            .insert(file_items.path_hash.clone(), file_items);
    }

    pub fn read_target_directory(&mut self, dir_path: &Path) {
        let children = fs::read_dir(dir_path).expect("dir Load Error");
        for child in children {
            let child = child.expect("Dir Entry error");
            let path = child.path();
            if path.is_dir() {
                Self::read_target_directory(self, &path);
            } else {
                Self::push_file_list(self, &path);
            }
        }
    }

    pub fn compare_start(&mut self, target_path: String) {
        self.compare_count = 0;
        let path = Path::new(&target_path);
        Self::compare_dir(self, path, &target_path);
    }

    pub fn compare_dir(&mut self, target_path: &Path, base_path: &String) {
        let children = fs::read_dir(target_path).expect("compare dir read error");
        for child in children {
            let child = child.expect("dir entry error");
            let path = child.path();
            if path.is_dir() {
                Self::compare_dir(self, &path, base_path);
            } else {
                let absolute_path = path.to_str().unwrap().replace(base_path, "");
                let mut hasher = Sha256::new();
                hasher.update(&absolute_path);
                self.compare_count += 1;
                if !Self::compare(
                    self,
                    path.to_str().unwrap().to_string(),
                    format!("{:X}", hasher.finalize()),
                ) {
                    self.compare_error.push(absolute_path);
                }
            }
        }
    }

    pub fn read_base_path(&mut self, taraget_path: String) {
        Self::set_base_path(self, taraget_path.clone());
        // self.base_path = taraget_path.clone();
        let base = Path::new(&taraget_path);
        self.file_list = HashMap::new();
        Self::read_target_directory(self, &base);
    }

    pub fn compare(&mut self, target_path: String, target_hash: String) -> bool {
        if !self.file_list.contains_key(&target_hash) {
            println!("notfound {}", target_hash);
            println!("hashkey => {:?}", self.file_list.keys());
            return false;
        } else {
            let compare_result = self
                .file_list
                .get_mut(&target_hash)
                .unwrap()
                .compare(target_path);
            return compare_result;
        }
    }

    pub fn not_compared_list(&self) -> Vec<String> {
        let mut not_compared: Vec<String> = Vec::new();

        for item in self.file_list.iter() {
            if !item.1.compared {
                not_compared.push(item.1.path.clone());
            }
        }
        return not_compared;
    }

    pub fn result_output(self, out_file: String, target_path: String) {
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("out dir is not found"),
        };
        let filename = if out_file == "" {
            "diff_output.txt"
        } else {
            &out_file
        };
        current.push(filename);
        let mut file = File::create(&current).expect("out file open error");
        let not_compared_list = Self::not_compared_list(&self);
        let mut out_info:String = format!("base path: {}\ntarget path: {}\nbase file count: {}\ncompare count: {}\nerror count: {}\nFiles not compared: {}\n", self.base_path, &target_path, self.file_list.len(), self.compare_count, self.compare_error.len(), &not_compared_list.len());
        out_info = format!(
            "{}\nerrorList:\n\t{}\n\nnot compared:\n\t{}",
            out_info,
            self.compare_error.join("\n\t"),
            not_compared_list.join("\n\t")
        );
        file.write_all(out_info.as_bytes()).expect("write error");
        file.flush().expect("output error");
        println!("output result => {}", format!("{}", current.display()));
    }
}

#[cfg(test)]
mod tests {
    use crate::diff_lib;
    use sha2::{Digest, Sha256};
    use std::env;

    #[test]
    fn test_read_target() {
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("current is not found"),
        };
        current.push("test");
        current.push("source");
        let mut source_loader = diff_lib::comparsion_source::ComparsionSource::new();
        let target_path: String = format!("{}", current.display());
        println!("target => {}", &target_path);
        source_loader.read_base_path(target_path);
        let file_list = source_loader.file_list;
        println!("keys => {:?}", file_list.keys().len());
        assert_eq!(file_list.keys().len(), 4);
    }

    #[test]
    fn test_compare() {
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("current is not found"),
        };
        current.push("test");
        let mut target = current.clone();
        current.push("source");
        let mut source_loader = diff_lib::comparsion_source::ComparsionSource::new();
        let target_path: String = format!("{}", current.display());
        source_loader.read_base_path(target_path);

        target.push("target");
        let mut target_file_path = target.clone();
        target_file_path.push("test.txt");
        let absolute_path = target_file_path
            .to_str()
            .unwrap()
            .replace(target.to_str().unwrap(), "");
        let mut hasher = Sha256::new();
        hasher.update(&absolute_path);

        let compare_result = source_loader.compare(
            target_file_path.to_str().unwrap().to_string(),
            format!("{:X}", hasher.finalize()),
        );
        assert_eq!(compare_result, true);
        let not_compared_list = source_loader.not_compared_list();
        assert_eq!(not_compared_list.len(), 3);
    }
}
