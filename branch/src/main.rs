mod state;

use state::get_files;

fn main() {
    println!("Hello, this is MAIN file!");
    
    let files = get_files();
    println!("Files found: {}", files.len());
}   