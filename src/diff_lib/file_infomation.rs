use core::panic;
use sha2::{Digest, Sha256};
use std::fs;
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
    fn set_path(&mut self, base_path: String, full_path: String) {
        self.full_path = full_path.clone();
        self.path = full_path.replace(&base_path, "");
        let filebinary = fs::read_to_string(full_path);
        match filebinary {
            Ok(filedata) => {
                let mut hasher = Sha256::new();
                hasher.update(filedata);
                self.file_hash = format!("{:X}", hasher.finalize());
                let mut path_hasher = Sha256::new();
                path_hasher.update(&self.path);
                self.path_hash = format!("{:X}", path_hasher.finalize());
            }
            Err(_) => {
                panic!("file load error!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::diff_lib;
    #[test]
    fn set_path() {
        let mut info = diff_lib::file_infomation::FileInfomation::new();
        info.set_path(
            "/Users/hayatoshimizu/develop/private/rust/dir_diff/".to_string(),
            "/Users/hayatoshimizu/develop/private/rust/dir_diff/Cargo.toml".to_string(),
        );
        assert_eq!(info.path, "Cargo.toml");
        println!("filehash => {}", info.file_hash);
        assert_eq!(
            info.file_hash,
            "097D0017DD7BCB4799909D9983BA42AFCFE2548BF870A72179DBE76CCBBE2C01"
        );
        assert_eq!(
            info.path_hash,
            "2E9D962A08321605940B5A657135052FBCEF87B5E360662BB527C96D9A615542"
        );
    }
}
