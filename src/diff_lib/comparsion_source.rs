use super::file_infomation;
use super::file_infomation::FileInfomation;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::hash;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use tokio::task;
pub struct ComparsionSource {
    pub base_path: String,
    pub file_list: HashMap<String, FileInfomation>,
    pub compare_files: Vec<String>,
    pub compare_error: Vec<String>,
    pub notfound_error: Vec<String>,
    pub compare_count: u32,
    pub start_time: Instant,
}

impl Default for ComparsionSource {
    fn default() -> Self {
        Self {
            base_path: "".to_string(),
            file_list: HashMap::new(),
            compare_files: Vec::new(),
            compare_error: Vec::new(),
            notfound_error: Vec::new(),
            compare_count: 0,
            start_time: Instant::now(),
        }
    }
}

impl ComparsionSource {
    pub fn new() -> ComparsionSource {
        Default::default()
    }

    pub fn read_target_directory(&mut self, dir_path: &Path) {
        let children = fs::read_dir(dir_path).expect("dir Load Error");
        for child in children {
            let child = child.expect("Dir Entry error");
            let path = child.path();
            if path.is_dir() {
                self.read_target_directory(&path);
                // Self::read_target_directory(self, &path);
            } else {
                let mut file_item = FileInfomation::new();
                file_item.set_path(&self.base_path, &path.to_str().unwrap());
                self.file_list
                    .insert(file_item.path_hash.clone(), file_item);
                // Self::push_file_list(self, &path);
            }
        }
    }

    pub async fn compare_start(&mut self, target_path: String) {
        self.compare_count = 0;
        let path = Path::new(&target_path);
        self.read_compare_dir_path(path);
        Self::compare_hashes(self, &target_path).await;
    }

    // compare_filesのpathのハッシュ化と、該当のファイルのハッシュ化を行う
    pub async fn compare_hashes(&mut self, base_path: &String) {
        let mut tasks = Vec::new();

        for item in self.compare_files.iter_mut() {
            let full_path = item.clone();
            let base = base_path.clone();
            let task = task::spawn(async move {
                let absolute_path = full_path.replace(&base, "");
                let mut hasher = Sha256::new();
                hasher.update(&absolute_path);
                let path_hash: String = format!("{:X}", hasher.finalize());
                let hash = file_infomation::calculate_hash(&full_path);
                (absolute_path, path_hash, hash)
            });
            tasks.push(task);
        }

        let results = futures::future::join_all(tasks).await;

        for result in results {
            if let Ok((absolute_path, path_hash, hash)) = result {
                if self.file_list.contains_key(&path_hash) {
                    let compare_result = self.file_list.get_mut(&path_hash).unwrap().compare(hash);
                    if !compare_result {
                        self.compare_error.push(absolute_path);
                    }
                } else {
                    self.notfound_error.push(absolute_path);
                }
            }
        }
    }

    pub fn read_compare_dir_path(&mut self, target_path: &Path) {
        let children = fs::read_dir(target_path).expect("compare dir read error");
        for child in children {
            let child = child.expect("dir entry error");
            let path = child.path();
            if path.is_dir() {
                Self::read_compare_dir_path(self, &path);
            } else {
                self.compare_files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    // pub async fn compare_dir(&mut self, target_path: &Path, base_path: &String) {
    //     let mut tasks = Vec::new();
    //     for child in self.co {
    //         let child = child.expect("dir entry error");
    //         let path = child.path();
    //         if path.is_dir() {
    //             let task = task::spawn(async move {
    //                 Self::compare_dir(self, &path, base_path).await;
    //             });
    //             tasks.push(task);
    //         } else {
    //             let absolute_path = path.to_str().unwrap().replace(base_path, "");
    //             let mut hasher = Sha256::new();
    //             hasher.update(&absolute_path);
    //             self.compare_count += 1;
    //             let task = task::spawn(async move {
    //                 if let Err(x) = Self::compare(
    //                     self,
    //                     path.to_str().unwrap().to_string(),
    //                     format!("{:X}", hasher.finalize()),
    //                 ) {
    //                     if x == -1 {
    //                         self.notfound_error.push(absolute_path);
    //                     } else {
    //                         self.compare_error.push(absolute_path);
    //                     }
    //                 }
    //             });
    //             tasks.push(task);
    //         }
    //     }
    //     futures::future::join_all(tasks).await;
    // }

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
                if let Err(x) = Self::compare(
                    self,
                    path.to_str().unwrap().to_string(),
                    format!("{:X}", hasher.finalize()),
                ) {
                    if x == -1 {
                        self.notfound_error.push(absolute_path);
                    } else {
                        self.compare_error.push(absolute_path);
                    }
                }
            }
        }
    }

    pub async fn read_base_path(&mut self, taraget_path: String) {
        // Self::set_base_path(self, taraget_path.clone());
        self.base_path = taraget_path.clone();
        let base = Path::new(&taraget_path);
        self.file_list = HashMap::new();
        println!("read target directory....");
        Self::read_target_directory(self, base);
        println!("calculate hash....");
        // file_listのループを回して、hashを計算する
        self.calculate_hashes().await;
    }

    pub async fn calculate_hashes(&mut self) {
        let mut tasks = Vec::new();

        for (_, item) in self.file_list.iter_mut() {
            let full_path = item.full_path.clone();
            let key = item.path_hash.clone();
            let task = task::spawn(async move {
                let hash = file_infomation::calculate_hash(&full_path);
                (key, hash)
            });
            tasks.push(task);
        }

        let results = futures::future::join_all(tasks).await;

        for result in results {
            if let Ok((key, hash)) = result {
                self.file_list
                    .get_mut(key.as_str())
                    .unwrap()
                    .set_file_hash(hash);
            }
        }
    }

    pub fn compare(&mut self, target_path: String, target_hash: String) -> Result<bool, i16> {
        if !self.file_list.contains_key(&target_hash) {
            // println!("notfound {}", target_hash);
            // println!("hashkey => {:?}", self.file_list.keys());
            // false
            Err(-1)
        } else {
            let handle = file_infomation::calculate_hash(&target_path);
            // let target_hash = ;
            let compare_result = self
                .file_list
                .get_mut(&target_hash)
                .unwrap()
                .compare(handle);
            if compare_result {
                Ok(true)
            } else {
                Err(-2)
            }
            // compare_result
        }
        // Err("error".to_string())
    }

    pub fn not_compared_list(&self) -> Vec<String> {
        let mut not_compared: Vec<String> = Vec::new();

        for (_, item) in self.file_list.iter() {
            if !item.compared {
                not_compared.push(item.path.clone());
            }
        }
        not_compared
    }

    pub fn result_output(self, out_file: String, target_path: String) {
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("out dir is not found"),
        };
        let filename = if out_file.is_empty() {
            "diff_output.txt"
        } else {
            &out_file
        };
        current.push(filename);
        let mut file = File::create(&current).expect("out file open error");
        //  out_info = String::new();
        let duration = self.start_time.elapsed();
        let not_compared_list = Self::not_compared_list(&self);
        let mut out_info: String = format!("process Time:{:?}\n\nbase path: {}\ntarget path: {}\nbase file count: {}\ncompare count: {}\nCompare error file count: {}\nNot found file count: {}\nNot compared file count: {}\n", duration, self.base_path, &target_path, self.file_list.len(), self.compare_count, self.compare_error.len(),self.notfound_error.len(), &not_compared_list.len());
        out_info = format!(
            "{}\nError files:\n\t{}\n\nNotfound filers:\n\t{}\n\nNot compared files:\n\t{}",
            out_info,
            self.compare_error.join("\n\t"),
            self.notfound_error.join("\n\t"),
            not_compared_list.join("\n\t")
        );
        file.write_all(out_info.as_bytes()).expect("write error");
        file.flush().expect("flush error");
        println!("output result => {}", current.display());
    }
}

#[cfg(test)]
mod tests {
    use crate::diff_lib;
    use sha2::{Digest, Sha256};
    use std::env;

    #[tokio::test]
    async fn test_read_target() {
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("current is not found"),
        };
        current.push("test");
        current.push("source");
        let mut source_loader = diff_lib::comparsion_source::ComparsionSource::new();
        let target_path: String = format!("{}", current.display());
        source_loader.read_base_path(target_path).await;
        let file_list = source_loader.file_list;
        assert_eq!(file_list.keys().len(), 4);
    }

    #[tokio::test]
    async fn test_compare() {
        let mut current = match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("current is not found"),
        };
        current.push("test");
        let mut target = current.clone();
        current.push("source");
        let mut source_loader = diff_lib::comparsion_source::ComparsionSource::new();
        let target_path: String = format!("{}", current.display());
        source_loader.read_base_path(target_path).await;

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
        assert_eq!(compare_result, Ok(true));
        let not_compared_list = source_loader.not_compared_list();
        assert_eq!(not_compared_list.len(), 3);
    }
}
