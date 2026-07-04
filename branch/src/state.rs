// mod filesystem;
// use crate::state::filesystem::scan_current_directory;
use crate::scan_current_directory;

use std::fs::DirEntry;

pub struct AppState{
    pub files: Vec<DirEntry>,
    pub selected_index: u32
}

impl AppState{
    pub fn get_files(&mut self){
        // self.files = scan_current_directory(dir_path);
        println!("state.rs - get_files, files len = {}", self.files.len());
    }

}

// fn main(){
//     let files = get_files();
//     println!("LEN FILES = {}", files.len())
// }