

pub mod program {

    pub trait Run {
        fn run(args: Vec<String>) -> std::io::Result<()>;
        fn init() -> std::io::Result<()> {
            let r = Self::run(std::env::args().collect());
            r
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