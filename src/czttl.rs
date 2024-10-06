pub mod input_output {
    use std::fs;
    use std::io::stdout;
    use std::io::Write;
    use std::str;
    use std::thread;
    use std::time::Duration;

    // Reads the content of the file at the provided path as a String.
    // If the file is valid UTF-8, its content is returned.
    // If the file is not valid UTF-8, an error message is returned instead of panicking.
    pub fn read_text(path: &str) -> String {
        let text_file = fs::read(path).expect("Error reading file!");

        match str::from_utf8(&text_file) {
            Ok(content) => content.to_string(),
            Err(_) => String::from("Error: File is not valid UTF-8"),
        }
    }

    // Clears the terminal screen and resets the cursor position to the top-left (0, 0).
    // Uses ANSI escape codes to clear the screen and move the cursor.
    // Works in most modern terminals without requiring any external dependencies.
    pub fn clear_terminal() {
        // "\x1B[2J" clears the screen, and "\x1B[H" moves the cursor to (0, 0)
        print!("\x1B[2J\x1B[H");
        stdout().flush().expect("Error occurred in clear_terminal function");
    }

    // Prints the provided string character by character with a delay between each character.
    // The delay is based on the input_time in milliseconds.
    // For whitespace characters (spaces and tabs), the delay is halved to improve reading speed.
    pub fn slow_print(input_string: &str, input_time: u64) {
        let sleep_time = Duration::from_millis(input_time);

        for c in input_string.chars() {
            if c.is_whitespace() && (c == ' ' || c == '\t') {
                // Reduce the sleep time for spaces and tabs to improve readability speed
                print!("{}", c);
                stdout().flush().expect("Error occurred in slow_print function");
                thread::sleep(sleep_time / 2);
            } else {
                // Normal delay for all other characters
                print!("{}", c);
                stdout().flush().expect("Error occurred in slow_print function");
                thread::sleep(sleep_time);
            }
        }
    }
}