use to_html::html::convert_to_html;
use to_html::html::url_decode;

fn main() -> std::io::Result<()> {
    println!("{}", convert_to_html("\"Hello World\" #include<iostream>"));
    println!("{}", url_decode("Hello+World%5A"));
    Ok(())
}
