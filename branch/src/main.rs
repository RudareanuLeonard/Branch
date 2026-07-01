mod ui;
mod state;
mod input;
mod filesystem;

use filesystem::scan_current_directory;
// use state::get_files;
// use ui::display_files;
use input::keyboard_read;
use state::AppState;
use ui::display_files;


fn main() {
    println!("Hello, this is MAIN file!");

    let files = scan_current_directory();

    let mut appState = AppState{
        files: files,
        selected_index: 0
    }; 


    appState.get_files();

    println!("main appState files len = {}", appState.files.len());

    
    // display_files();
    println!("");
    println!("");
    println!("");

    display_files(&appState);

    // keyboard_read();

}   