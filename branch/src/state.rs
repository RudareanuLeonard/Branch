mod filesystem;
use crate::state::filesystem::scan_current_directory;

use std::fs::DirEntry;

struct AppState{
    files: Vec<DirEntry>
}

pub fn get_files() -> Vec<DirEntry>{
    let files = scan_current_directory();
    files
}

// fn main(){
//     let files = get_files();
//     println!("LEN FILES = {}", files.len())
// }