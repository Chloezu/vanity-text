mod czttl;
pub use crate::czttl::input_output;

use std::process;
use std::env;
use std::path::Path;

fn main() {

    //  checks if there are any arguments
    let args: Vec<String> = env::args().collect();


    // if there are no arguments, prints usage and exits
    if args.len() <= 1 {
        println!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }


    //  println!("past arg empty check");
    //  println!("{:?}", args);
    let file_exists = Path::new(&args[1]);
    if !file_exists.exists() {
        println!("Error: {} is not a valid file!", args[1]);
        process::exit(1);
    }

    //  attempts to read the first argument as file
    //  if succesful, clears terminal and prints slowly
    //  otherwise exits

    let str_to_print = input_output::read_text(&args[1]);
    input_output::clear_terminal();
    input_output::slow_print(&str_to_print, 50);


    //  exits after creating newline to avoid annoyance
    println!();
    process::exit(0);
}
