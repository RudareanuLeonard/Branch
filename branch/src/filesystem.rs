use std::fs;
use fs::DirEntry;
use std::fs::ReadDir;


pub fn scan_current_directory(dir_path: &mut ReadDir) -> Vec<DirEntry>{

    // let dir_path = fs::read_dir("../test_dir").unwrap(); //unwrap get inner result of Res
    // println!("DIR PATH = {}", dir_path.to_string_loosy());
    let mut v_files = Vec::new();
    for path in dir_path{
       v_files.push(path.unwrap());
    }

    v_files
}

// pub fn scan_next_dir(current_dir_path: String, new_dir_name: )