use std::fs;
use std::path::PathBuf;

pub(crate) struct Reader {}

impl Reader {
    pub fn read(file_array: &Vec<PathBuf>) -> String {
        let mut read_str = String::new();
        for path in file_array {
            let file_name = path
                .as_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            read_str += &file_name;
            read_str += "\n";
            let content = fs::read_to_string(path).unwrap();
            read_str += &content;
        }
        read_str
    }
}
