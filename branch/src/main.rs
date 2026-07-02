mod ui;
mod state;
mod input;
mod filesystem;
mod navigation;

use std::io;
use navigation::move_up_down;
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

    loop{
        display_files(&appState);
        println!("");
        println!("Enter a key");
        let mut key: String = String::new();

        io::stdin().read_line(&mut key).expect("Wrong input !");

        key = key.trim().to_string();

        if key == "w" || key == "s"{
            println!("Up or Down");
            move_up_down(&mut appState, &key)
        }
        else if key == "q"{
            break;
        }
        else{
            println!("Key = {}", key);
        }

    }

    

    // keyboard_read();

}   