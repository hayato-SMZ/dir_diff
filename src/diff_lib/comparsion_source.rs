use super::{file_infomation::FileInfomation, *};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
pub struct ComparsionSource {
    pub base_path: String,
    pub file_list: HashMap<String, FileInfomation>,
}

impl Default for ComparsionSource {
    fn default() -> Self {
        Self {
            base_path: "".to_string(),
            file_list: HashMap::new(),
        }
    }
}

impl ComparsionSource {
    pub fn new() -> ComparsionSource {
        Default::default()
    }

    pub fn set_base_path(&mut self, hash_path: String) {
        self.base_path = hash_path;
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

    pub fn read_base_path(&mut self, taraget_path: String) {
        self.base_path = taraget_path.clone();
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
}

#[cfg(test)]
mod tests {
    use crate::diff_lib;
    use sha2::{Digest, Sha256};
    use std::env;
    use std::path::Path;

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
        assert_eq!(file_list.keys().len(), 3);
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
        println!("absolute path => {}", absolute_path);
        hasher.update(&absolute_path);

        let compare_result = source_loader.compare(
            target_file_path.to_str().unwrap().to_string(),
            format!("{:X}", hasher.finalize()),
        );
        assert_eq!(compare_result, true);
    }
}
