// use crate::get_files;
use crate::AppState;

pub fn display_files(appState: &AppState){
    // let files = get_files();
    println!("ui.rs - display_files");

    for i in 0..appState.files.len(){
        let file = &appState.files[i];
        if i == appState.selected_index.try_into().unwrap(){
            println!("> FILE = {}", file.file_name().to_string_lossy().into_owned());
        }
        else{
            println!("FILE = {}", file.file_name().to_string_lossy().into_owned());
        }
        
    }
}
