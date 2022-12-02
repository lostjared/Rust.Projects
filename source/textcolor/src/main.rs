use colored::Colorize;

fn main() -> std::io::Result<()> {
    println!("{}: is red", "Color".red());
    println!("{}: is blue bold", "Color2".blue().bold());
    Ok(())
}
