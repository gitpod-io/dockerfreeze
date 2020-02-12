use std::env::vars;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Output file for Dockerfile
    #[structopt(short = "o", long = "output")]
    file: Option<String>,
}

fn write_env_vars(file: &mut File) {
    let mut count = 0;
    for (key, value) in vars() {
        let value = value.replace("\"", "\\\"");
        if value.contains(" ") || value.contains("{") || value.contains("}") {
            if count == 0 {
                &file
                    .write(format!("ENV {}=\"{}\" ", key, value).as_bytes())
                    .unwrap();
            } else {
                &file
                    .write(format!("{}=\"{}\" ", key, value).as_bytes())
                    .unwrap();
            }
        } else {
            if count == 0 {
                &file
                    .write(format!("ENV {}={} ", key, value).as_bytes())
                    .unwrap();
            } else {
                &file
                    .write(format!("{}={} ", key, value).as_bytes())
                    .unwrap();
            }
        }
        count+=1;
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
