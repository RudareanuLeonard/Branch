use std::fs;
use fs::DirEntry;
use std::fs::ReadDir;


pub fn scan_current_directory(dir_path: &mut ReadDir) -> Vec<DirEntry>{
    let mut v_files = Vec::new();
    
    for path in dir_path{
        println!("scan_current_directoty; path = {}", path.as_ref().unwrap().path().display());
       v_files.push(path.unwrap());
    }

    v_files
}

// pub fn scan_next_dir(current_dir_path: String, new_dir_name: )