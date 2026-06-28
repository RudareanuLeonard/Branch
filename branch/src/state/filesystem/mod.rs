use std::fs;
use fs::DirEntry;

pub fn scan_current_directory() -> Vec<DirEntry>{

    let dir_path = fs::read_dir("../test_dir").unwrap(); //unwrap get inner result of Res
    let mut v_files = Vec::new();
    for path in dir_path{
       v_files.push(path.unwrap());
    }

    v_files
}