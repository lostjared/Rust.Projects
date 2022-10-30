pub mod log {

    pub struct Log {
        out_stream: Box<dyn std::io::Write>,
    }

    impl Log {
        // new standard output log
        pub fn new_stdout_log() -> Self {
            Self {
                out_stream: Box::new(std::io::BufWriter::new(std::io::stdout().lock())),
            }
        }
        // new standard error log
        pub fn new_stderr_log() -> Self {
            Self {
                out_stream: Box::new(std::io::BufWriter::new(std::io::stderr().lock())),
            }
        }
        // new log output file
        pub fn new_log_file(output: &str) -> Self {
            let f = std::fs::File::create(output).expect("on create of file ");
            Self {
                out_stream: Box::new(std::io::BufWriter::new(f)),
            }
        }
        /// information log
        pub fn i(&mut self, data: &str) {
            let date = chrono::Local::now();
            let t = date.format("%Y-%m-%d][%H:%M:%S");
            write!(self.out_stream, "Info [{}]: {}", t, data).expect("On log write");
        }
        /// error log
        pub fn e(&mut self, data: &str) {
            let date = chrono::Local::now();
            let t = date.format("%Y-%m-%d][%H:%M:%S");
            write!(self.out_stream, "Error [{}]: {}", t, data).expect("On log write");
        }
        /// standard log
        pub fn o(&mut self, data: &str) {
            let date = chrono::Local::now();
            let t = date.format("%Y-%m-%d][%H:%M:%S");
            write!(self.out_stream, "[{}]: {}", t, data).expect("On log write");
        }
    }
}
