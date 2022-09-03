use to_html::html::convert_to_html;

fn main() -> std::io::Result<()> {
    println!("{}", convert_to_html("\"Hello World\" #include<iostream>"));
    Ok(())
}