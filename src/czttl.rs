pub mod input_output {
    use std::fs;
    use std::io::stdout;
    use std::io::Write;
    use std::str;
    use std::thread;
    use std::time::Duration;
    
    
    // Reads file to variable
    pub fn read_text(path: &str) -> String {
        let text_file = fs::read_to_string(path)
            .expect("Error reading file!");
        return text_file;
    }
    
    // *should* clear terminal is currently broken however.
    // TODO: Fix the cursor not being moved to the first possible line on the terminal,
    // it currently just starts from where the cursor's last position was
    pub fn clear_terminal() {
        print!("{}[2J", 27 as char);
        thread::sleep(Duration::from_millis(250));
        stdout()
            .flush()
            .expect("Error occured in clear_terminal function")
    }
    
    // takes text {input_string} and prints it character by character 
    // with a delay based on the {input_time} variable
    // TODO: maybeFeature; see about maybe adding a feature to skip
    // tab indents or chunks of spaces to allow for better reading
    // of files like python for instance
    pub fn slow_print(input_string: &str, input_time: u64) {
        let char_vec: Vec<char> = input_string.chars().collect();
        let sleep_time = Duration::from_millis(input_time);

        for c in char_vec {
            print!("{}", c);
            stdout()
                .flush()
                .expect("error occured in slow_print function");
            thread::sleep(sleep_time);
        }
    }

}
