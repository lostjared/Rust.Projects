use std::io::{BufRead, Write};

fn give<I, O>(input: &mut I, output: &mut O) -> String
where
    I: BufRead,
    O: Write,
{
    input_line("Enter something: ", input, output)
}
fn get(text: String) -> Option<String> {
    if text == "death" {
        return None;
    }
    Some(text)
}
fn input_line<I, O>(text: &str, input: &mut I, output: &mut O) -> String
where
    I: BufRead,
    O: Write,
{
    write!(output, "{}", text).expect("Error on write");
    output.flush().expect("Error on flush");
    let mut s = String::new();
    input.read_line(&mut s).expect("Error on Read line");
    s.trim_end().to_string()
}

fn main() -> std::io::Result<()> {
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    let mut actions = Vec::new();
    loop {
        let data = get(give(&mut input, &mut output));
        if let Some(data) = data {
            println!("Karma Returns: {}", data);
            actions.push(data);
        } else {
            break;
        }
    }
    println!("Eescape!");
    println!("looking back: ");
    for i in &actions {
        println!("{}", i);
    }
    Ok(())
}
