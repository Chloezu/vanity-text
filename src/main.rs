//use colored::Colorize;

mod czttl;
pub use crate::czttl::input_output;

use std::process;
use std::env;
use std::path::Path;

//mod local_text;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let file_exists = Path::new(&args[1]);
    if ! file_exists.exists() {
        println!("{} is not a file!", args[1]);
        process::exit(1);
    }


    // TODO: change this to be an argument call, so the user can put whatever file is wanted
    let str_to_print= input_output::read_text(&args[1]);
    
    // This will clear the terminal with a print command
    input_output::clear_terminal(); 
    
    // slowly prints the text to the screen
    input_output::slow_print(&str_to_print, 50);

    println!("");
    process::exit(0);
}


