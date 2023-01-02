use super::{file_infomation::FileInfomation, *};
use std::collections::HashMap;
use std::fs;
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
}
