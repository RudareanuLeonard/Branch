use std::fs;
use std::env;

fn scan_current_directory(){

    let dir_path = fs::read_dir("../test_dir").unwrap(); //unwrap get inner result of Res
    for path in dir_path{
        // let mut path_name;
        // path_name = &path.unwrap().path().display();
        println!("File = {}", path.unwrap().path().display());
        // println!("Name: {}", path.unwrap().path().display())
    }


}

fn main() {
    println!("welcome to filesystem.rs");

    // println!("CWD = {:?}", env::current_dir().unwrap());
    scan_current_directory();
}