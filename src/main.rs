use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;
use structopt::StructOpt;
use std::fs::File;
use std::env::vars;

#[derive(StructOpt)]
struct Cli {
    /// Output file for Dockerfile
    #[structopt(short = "o", long = "output")]
    file: Option<String>,
}

fn write_env_vars(file: &mut File) {
    for (key, value) in vars() {
        if value.contains(" ") {
            &file.write(format!("ENV {}=\"{}\"\n", key, value).as_bytes()).unwrap();
        } else {
            &file.write(format!("ENV {}={}\n", key, value).as_bytes()).unwrap();
        }
    }
}

fn main() {
    let args = Cli::from_args();
    let mut file = match args.file {
        Some(n) => match OpenOptions::new().write(true).create_new(true).open(n) {
            Ok(n) => {
                println!("\x1b[1;32mFile Created!\x1b[m");
                n
            }
            Err(_) => {
                println!("\x1b[1;31mError: File Already Exists!\x1b[m");
                exit(1);
            }
        },
        None => {
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("Dockerfile")
            {
                Ok(n) => {
                    println!("\x1b[1;32mFile Created!\x1b[m");
                    n
                }
                Err(_) => {
                    println!("\x1b[1;31mError: File Already Exists!\x1b[m");
                    exit(1);
                }
            }
        }
    };
    write_env_vars(&mut file);
}
