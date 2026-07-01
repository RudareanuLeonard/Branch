// use crate::get_files;
use crate::AppState;

pub fn display_files(appState: &AppState){
    // let files = get_files();
    println!("ui.rs - display_files");
    for file in &appState.files{
        println!("FILE = {}", file.file_name().to_string_lossy().into_owned());
    }
}
