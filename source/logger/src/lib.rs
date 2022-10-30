pub mod log {

    pub struct Log {
        program_name: String,
        out_stream: Box<dyn std::io::Write>,
    }

    pub fn the_time() -> String {
        let date = chrono::Local::now();
        date.format("%Y-%m-%d - %H:%M:%S").to_string()
    }

    impl Log {
        /// new standard output log
        pub fn new_stdout_log(name: &str) -> Self {
            Self {
                program_name: name.to_string(),
                out_stream: Box::new(std::io::BufWriter::new(std::io::stdout().lock())),
            }
        }
        /// new standard error log
        pub fn new_stderr_log(name: &str) -> Self {
            Self {
                program_name: name.to_string(),
                out_stream: Box::new(std::io::BufWriter::new(std::io::stderr().lock())),
            }
        }
        /// new log output file
        pub fn new_log_file(name: &str, output: &str) -> Self {
            let f = std::fs::File::create(output).expect("on create of file ");
            Self {
                program_name: name.to_string(),
                out_stream: Box::new(std::io::BufWriter::new(f)),
            }
        }
        /// information log
        pub fn i(&mut self, data: String) {
            write!(self.out_stream, "{}: {} - Info: {}\n", self.program_name, the_time(), data).expect("On log write");
        }
        /// error log
        pub fn e(&mut self, data: String) {
            write!(self.out_stream, "{}: {} - Error: {}\n", self.program_name, the_time(), data).expect("On log write");
        }
        /// standard log
        pub fn o(&mut self, data: String) {
            write!(self.out_stream, "{}: {} - {}\n", self.program_name, the_time(), data).expect("On log write");
        }
        /// fatal
        pub fn f(&mut self, data: String) {
             panic!("{}: {} - Fatal: Error: {}\n", self.program_name, the_time(), data);
        }

        pub fn fd(&mut self) -> &mut Box<dyn std::io::Write> {
            &mut self.out_stream
        }
        
    }
}
