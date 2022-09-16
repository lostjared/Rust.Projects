use clap::{App, Arg};

struct Arguments {
    in_file: String,
    html: bool,
}

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

fn parse_text<T>(mut reader: T)
where
    T: std::io::BufRead + Sized,
{
    let mut input: String = String::new();
    reader.read_to_string(&mut input).expect("read string");
    let rlex = rs_lex::rlex::Scanner::new(&input);
    for i in rlex {
        println!("{:?} -> {}", i.get_type(), i.get_string());
    }
}

fn parse_text_output_html<T>(mut reader: T)
where
    T: std::io::BufRead + Sized,
{
    let mut input: String = String::new();
    reader.read_to_string(&mut input).expect("read string");
    let rlex = rs_lex::rlex::Scanner::new(&input);
    println!("<!doctype html><html lang=\"en\"><head><title>Source Code</title></head><body>");
    println!("<table border=\"1\"><tr><th>Type</th><th>Token</th></tr>\n");
    for i in rlex {
        println!(
            "<tr><th>{:?}</th><th>{}</th></tr>",
            i.get_type(),
            to_html(i.get_string())
        );
    }
    println!("</table></body></html>");
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    if args.in_file == "<STDIN>" {
        if args.html {
            parse_text_output_html(std::io::stdin().lock());
        } else {
            parse_text(std::io::stdin().lock());
        }
    } else {
        let f = std::fs::File::open(args.in_file)?;
        let r = std::io::BufReader::new(f);
        if args.html {
            parse_text_output_html(r);
        } else {
            parse_text(r);
        }
    }
    Ok(())
}
