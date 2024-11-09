/*

    This Rust program, named ldd-deploy, is a command-line utility designed to identify and copy all shared library dependencies (.dll files)
    of a given executable to a specified output directory, primarily for use in a MinGW or MSYS environment on Windows.

    How It Works:
    Argument Parsing:

    The program accepts three command-line arguments:
    input: The path to the executable whose dependencies you want to analyze.
    msys: The root path of the MSYS2 or MinGW installation, used to locate .dll files.
    output: The directory to which the required .dll files will be copied (defaults to the current directory if not specified).
    Dependency Analysis:

    The program uses the ldd command to list all dependencies of the specified executable.
    It then filters the output to exclude dependencies related to Windows system libraries.
    File Copying:

    For each dependency found, the program determines its full path within the MSYS environment.
    It then copies each .dll file from its original location to the specified output directory, maintaining the same filename.
    Error Handling:

    If any errors occur, such as missing dependencies or issues with copying files, the program outputs an error message and exits.
    Use Case:
    This program is useful when you need to deploy a Linux-compiled executable on Windows (under MinGW/MSYS) along with all its required .dll files,
    ensuring the executable can run independently without requiring users to set up an MSYS environment manually.

*/
use clap::{App, Arg};
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug)]
struct Arguments {
    pub input: PathBuf,
    pub output: PathBuf,
    pub msys: PathBuf,
}

fn parse_args() -> Arguments {
    let matches = App::new("ldd-deploy")
        .version("0.1.0")
        .author("Jared Bruni")
        .about("ldd-deploy")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .help("input executable")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("msys")
                .short('m')
                .long("msys")
                .help("msys path")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .help("output directory")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let input = PathBuf::from(matches.value_of("input").unwrap());
    let msys = PathBuf::from(matches.value_of("msys").unwrap());
    let output = matches
        .value_of("output")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    Arguments {
        input,
        output,
        msys,
    }
}

fn copy_dll(msys: &Path, input_loc: &str, output_dir: &Path) -> std::io::Result<()> {
    let pos = input_loc.find("=>").unwrap();
    let fname = &input_loc[0..pos].trim();
    let right = &input_loc[pos + 3..];
    let pos2 = right.find("(").unwrap();
    let loc = &right[1..pos2 - 1].trim();
    let src = msys.join(loc);
    let dst = output_dir.join(fname);
    println!("{} -> {}", src.display(), dst.display());
    std::fs::copy(&src, &dst)?;
    Ok(())
}

fn extract_dll(msys: &Path, input: &Path, output_dir: &Path) -> std::io::Result<()> {
    let command = format!("ldd \"{}\" | grep -vi windows", input.display());
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let mut output = String::new();
    if let Some(ref mut stdout) = child.stdout {
        stdout.read_to_string(&mut output)?;
    } else {
        eprintln!("An error has occurred..");
        std::process::exit(1);
    }
    let _ = child.wait();
    let mut istream = std::io::BufReader::new(output.as_bytes());
    let mut line = String::new();
    while istream.read_line(&mut line)? > 0 {
        copy_dll(msys, line.trim(), output_dir)?;
        line.clear();
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    extract_dll(&args.msys, &args.input, &args.output)?;
    Ok(())
}
