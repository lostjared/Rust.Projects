pub mod log {

    pub struct Log {
        program_name: String,
        out_stream: Box<dyn std::io::Write>,
        echo: bool,
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
                echo: false,
            }
        }
        /// new standard error log
        pub fn new_stderr_log(name: &str) -> Self {
            Self {
                program_name: name.to_string(),
                out_stream: Box::new(std::io::BufWriter::new(std::io::stderr().lock())),
                echo: false,
            }
        }
        /// new log output file
        pub fn new_log_file(name: &str, output: &str, echo_value: bool) -> Self {
            let f = std::fs::File::create(output).expect("on create of file ");
            Self {
                program_name: name.to_string(),
                out_stream: Box::new(std::io::BufWriter::new(f)),
                echo: echo_value,
            }
        }
        /// information log
        pub fn i(&mut self, data: String) {
            self.log(data, "Information:".to_string())
        }
        /// error log
        pub fn e(&mut self, data: String) {
            self.log(data, "Error:".to_string())
        }
        /// standard log
        pub fn o(&mut self, data: String) {
            self.log(data, "".to_string());
        }

        pub fn w(&mut self, data: String) {
            self.log(data, "Warning:".to_string());
        }

        pub fn log(&mut self, data: String, level: String) {
            let t = the_time();
            write!(
                self.out_stream,
                "{}: {} - {} {}\n",
                self.program_name, t, level, data
            )
            .expect("On log write");
            if self.echo {
                println!("{}: {} - {} {}", self.program_name, t, level, data)
            }
        }

        /// fatal
        pub fn f(&mut self, data: String) {
            panic!(
                "{}: {} - Fatal: Error: {}\n",
                self.program_name,
                the_time(),
                data
            );
        }

        pub fn fd(&mut self) -> &mut Box<dyn std::io::Write> {
            &mut self.out_stream
        }
    }
}
