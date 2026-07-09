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
use std::path::Path;
use std::path::PathBuf;



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
        // let index = 0 as usize;
        println!("MAIN INDEX = {}", appState.selected_index.to_string());
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
        else if key == ""{ // TO DO - PRESS ENTER IS VALID ONLY ON DIRECTORIES
            println!("ENTER pressed");
            // let dir_path = fs::read_dir("../test_dir").unwrap();
            let mut current_path: PathBuf = Path::new("../test_dir").to_path_buf();
            
            let index = appState.selected_index as usize;
            current_path = current_path.join(&appState.files[index].path());   

            
            let selected_entry = &appState.files[index];

            current_path = selected_entry.path();

            println!("DIR PATH = {}", current_path.to_string_lossy());

            let mut read_dir_path = fs::read_dir(&current_path).unwrap();

            files = scan_current_directory(&mut read_dir_path);
            appState.selected_index = 0;

            println!("New len of files = {}", files.len());
            println!("NOW WILL CHANGE THE appState.files");
            appState.files = files;
        }
        else if key == "b"{ // go to the prev directory
            // let mut current_path: PathBuf = Path::new("../test_dir").to_path_buf();
            let index = appState.selected_index as usize;
            println!("INDEX = {}", index.to_string());
            let selected_entry = &appState.files[index];

            let current_path = selected_entry.path();

            println!("DIR PATH = {}", current_path.to_string_lossy());
            println!("b pressed; current path = {}", current_path.display().to_string());

            //.parent()
            let parent_path = current_path.parent().unwrap();
            let mut parent_dir = std::fs::read_dir(parent_path).unwrap();
            let files = scan_current_directory(&mut parent_dir);
            appState.files = files;
            println!("b pressed; PARENT path = {}", parent_path.display().to_string());

        }
        else{
            println!("Key = {}", key);
        }

    }

    

    // keyboard_read();

}   