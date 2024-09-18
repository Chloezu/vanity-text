pub mod input_output {
    use std::fs;
    use std::io::stdout;
    use std::io::Write;
    use std::str;
    use std::thread;
    use std::time::Duration;
    
    pub fn read_text(path: &str) -> String {
        let text_file = fs::read_to_string(path)
            .expect("Error reading file!");
        return text_file;
    }
    
    pub fn clear_terminal() {
        print!("{}[2J", 27 as char);
        thread::sleep(Duration::from_millis(250));
        stdout()
            .flush()
            .expect("Error occured in clear_terminal function")
    }

    pub fn slow_print(input_string: &str, input_time: u64) {
        let char_vec: Vec<char> = input_string.chars().collect();
        let sleep_time = Duration::from_millis(input_time);

        for c in char_vec {
            print!("{}", c);
            stdout()
                .flush()
                .expect("error occured in slow write function");
            thread::sleep(sleep_time);
        }
    }

}
