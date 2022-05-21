use crate::cli::Cli;
use glob::glob;
use std::fs;
use std::vec::Vec;
use std::{path::Path, path::PathBuf, str::FromStr};

pub(crate) struct FileFilter<'a> {
    code_path: PathBuf,
    glob_base: &'a Vec<String>,
    excluse_strs: &'a Vec<String>,
}

impl<'a> FileFilter<'a> {
    pub fn new(args: &'a Cli) -> Self {
        FileFilter {
            code_path: PathBuf::from_str(&args.path).unwrap(),
            glob_base: &args.match_str,
            excluse_strs: &args.excluse_strs,
        }
    }

    pub fn run(&self) -> Vec<PathBuf> {
        let mut filter_file_array: Vec<PathBuf> = Vec::new();
        let path = self.code_path.as_path();
        self.filter_dir(path, &mut filter_file_array);
        filter_file_array
    }

    fn filter_dir(&self, path: &Path, ff_arr: &mut Vec<PathBuf>) {
        let glob_base = path.to_str().unwrap().to_string();
        glob_match(&glob_base, self.glob_base, ff_arr);
        let dir = path.read_dir().unwrap();
        for x in dir {
            if let Ok(path) = x {
                if let Ok(file_type) = path.file_type() {
                    let file_name = path.file_name().to_str().unwrap().to_string();
                    if file_type.is_dir() && !self.excluse_strs.contains(&file_name) {
                        self.filter_dir(&path.path(), ff_arr);
                    }
                }
            }
        }
    }
}

pub fn glob_match(glob_base: &String, glob_str_array: &Vec<String>, ff_arr: &mut Vec<PathBuf>) {
    if glob_str_array.is_empty() {
        let glob_str = glob_base.clone() + "/*";
        single_glob_match(&glob_str, ff_arr)
    } else {
        for str in glob_str_array {
            let glob_str = glob_base.clone() + "/" + str;
            single_glob_match(&glob_str, ff_arr);
        }
    }
}

fn single_glob_match(glob_str: &String, ff_arr: &mut Vec<PathBuf>) {
    for entry in glob(glob_str).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // covert to absolute path
                let path = fs::canonicalize(&path).unwrap();
                println!("{:?}", path.display());
                ff_arr.push(path);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
