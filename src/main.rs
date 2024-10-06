mod czttl;
pub use crate::czttl::input_output;

use std::process;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_exists = Path::new(&args[1]);
    if !file_exists.exists() {
        println!("Error: {} is not a valid file!", args[1]);
        process::exit(1);
    }

    let str_to_print = input_output::read_text(&args[1]);
    input_output::clear_terminal();
    input_output::slow_print(&str_to_print, 50);

    println!();
    process::exit(0);
}
