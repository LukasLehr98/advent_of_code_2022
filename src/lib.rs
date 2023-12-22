use std::{env, fs};

pub fn read_file_input(file: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("./inputs").join(file);
    fs::read_to_string(filepath).unwrap()
}
