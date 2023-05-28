use clap::{App, Arg};
use std::io::Write;

/// Arguments
struct Arguments {
    in_file: String,
    html: bool,
}
/// Parse the arguments
fn parse_args() -> Arguments {
    let m = App::new("rs_lex")
        .help("rs_lex")
        .about("rs_lex")
        .author("Jared Bruni <jaredbruni@protonmail.com>")
        .arg(
            Arg::with_name("file")
                .multiple(false)
                .required(false)
                .allow_invalid_utf8(true)
                .default_value("<STDIN>"),
        )
        .arg(
            Arg::with_name("html")
                .multiple(false)
                .required(false)
                .takes_value(false)
                .long("html")
                .short('h'),
        )
        .get_matches();
    let f = m.value_of_lossy("file").unwrap();
    let h = m.is_present("html");

    Arguments {
        in_file: f.to_string(),
        html: h,
    }
}
/// convert string to HTML
pub fn to_html(input: String) -> String {
    let mut s = String::new();
    for i in 0..input.len() as usize {
        let ch = input.chars().nth(i).unwrap();
        match ch {
            '<' => {
                s.push_str("&lt;");
            }
            '>' => {
                s.push_str("&gt;");
            }
            _ => {
                s.push(ch);
            }
        }
    }
    s
}
/// scan text
fn scan_text<T>(reader: T)
where
    T: std::io::BufRead + Sized,
{
    print!("> ");
    std::io::stdout().lock().flush().expect("on flush");
    for line in reader.lines() {
        let rlex = rs_lex::rlex::Scanner::new(&line.unwrap());
        for i in rlex {
            let id = format!("{:?}", i.get_type());
            println!("{:15} -> {}", id, i.get_string());
        }
        print!("> ");
        std::io::stdout().lock().flush().expect("on flush");
    }
}

/// scan text and output html to stdout (redirect to file with >)
fn scan_text_output_html<T>(mut reader: T)
where
    T: std::io::BufRead + Sized,
{
    let mut input: String = String::new();
    reader.read_to_string(&mut input).expect("read string");
    let rlex = rs_lex::rlex::Scanner::new(&input);
    println!("<!doctype html><html lang=\"en\"><head><title>Source Code</title></head><body>");
    println!(
        "<table border=\"1\"><tr><th>Line</th><th>Index</th><th>Type</th><th>Token</th></tr>\n"
    );
    let mut index = 1;
    for i in rlex {
        println!(
            "<tr><th>{}</th><th>{}</th><th>{:?}</th><th>{}</th></tr>",
            i.get_line(),
            index,
            i.get_type(),
            to_html(i.get_string())
        );
        index += 1;
    }
    println!("</table></body></html>");
}

/// main function
fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.in_file == "<STDIN>" {
        if args.html {
            scan_text_output_html(std::io::stdin().lock());
        } else {
            scan_text(std::io::stdin().lock());
        }
    } else {
        let f = std::fs::File::open(args.in_file)?;
        let r = std::io::BufReader::new(f);
        if args.html {
            scan_text_output_html(r);
        } else {
            scan_text(r);
        }
    }
    Ok(())
}
