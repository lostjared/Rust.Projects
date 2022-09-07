pub mod program {

    pub trait Run {
        fn run(args: Vec<String>) -> std::io::Result<()>;
        fn init() -> std::io::Result<()> {
            Self::run(std::env::args().collect())
        }
    }
    pub struct Program {}
    impl Run for Program {
        fn run(args: Vec<String>) -> std::io::Result<()> {
            println!("Hello World from Program: {}!\n", args.len());
            Ok(())
        }
    }
}
