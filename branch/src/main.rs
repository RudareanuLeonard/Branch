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
use std::fs;
use fs::DirEntry;



fn main() {
    println!("Hello, this is MAIN file!");

    let mut dir_path = fs::read_dir("../test_dir").unwrap(); //unwrap get inner result of Res // this is the initial path we are starting on

    
    

    let mut files = scan_current_directory(&mut dir_path);

    let mut appState = AppState{
        files: files,
        selected_index: 0
    }; 

    // appState.files = files;

    println!("main appState files len = {}", appState.files.len());

    
    // display_files();
    println!("");
    println!("");
    println!("");

    loop{ // TO DO: implement console clear to make it look smoother and not have files printed multiple times
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
        else if key == ""{
            println!("ENTER pressed");
            // let dir_path = fs::read_dir("../test_dir").unwrap();
            files = scan_current_directory(&mut dir_path);
            println!("New len of files = {}", files.len());
        }
        else{
            println!("Key = {}", key);
        }

    }

    

    // keyboard_read();

}   