mod czttl;
pub use crate::czttl::input_output;

use std::process;
use std::env;
use std::path::Path;


fn main() {
    
    // TODO: fix; add help text and handling for no arguments, panics without arg
    //
    // is first arg a valid file?
    let args: Vec<String> = env::args().collect();
    //println!("past arg assign");
    
    println!("{}", args.len());

    if args.len() <= 1 {
        println!("A file location argument is required!");
        process::exit(1);
    }
    //println!("past arg empty check");
    //println!("{:?}", args);

    let file_exists = Path::new(&args[1]);
    if ! file_exists.exists() {
        println!("{} is not a file!", args[1]);
        process::exit(1);
    }

    // attempts to read the first argument as file
    // if succesful, clears terminal and prints slowly
    // otherwise exits
    // TODO: fix; if not utf8 it currently will panic
    // TODO: feature; add support for binary and hex read + print functionality via arguments
    let str_to_print= input_output::read_text(&args[1]);
    input_output::clear_terminal(); 
    input_output::slow_print(&str_to_print, 50);

    // exits after creating newline to avoid annoyance
    println!();
    process::exit(0);
}


