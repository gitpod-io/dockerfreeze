use std::fs::OpenOptions;
use structopt::StructOpt;
use std::process::exit;
use std::io::Write;

#[derive(StructOpt)]
struct Cli {
    /// Output file for Dockerfile
    #[structopt(short = "o", long = "output")]
    file: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    let _file = match args.file {
        Some(n) => match OpenOptions::new().write(true).create_new(true).open(n) {
            Ok(n) => {
                println!("\x1b[1;32mFile Created!\x1b[m");
                n
            }
            Err(_) => {
                println!("\x1b[1;31mError: File Already Exists!\x1b[m");
                exit(1);
            },
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
}
