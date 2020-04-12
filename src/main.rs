mod detection;
mod lib;
mod write;

use crate::lib::Cli;
use crate::write::write_env_vars;
use crate::write::write_linux_distro;
use std::env::current_dir;
use std::fs::OpenOptions;
use std::process::exit;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let mut file = match &args.file {
        Some(n) => match OpenOptions::new().write(true).create_new(true).open(n) {
            Ok(n) => {
                println!(
                    "\x1b[1;32mFile Created in {}/{}!\x1b[m",
                    current_dir()
                        .expect("Current Directory does not exist")
                        .to_str()
                        .unwrap(),
                    &args.file.unwrap()
                );
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
                    println!(
                        "\x1b[1;32mFile Created in {}/Dockerfile!\x1b[m",
                        current_dir()
                            .expect("Current Directory does not exist")
                            .to_str()
                            .unwrap()
                    );
                    n
                }
                Err(_) => {
                    println!("\x1b[1;31mError: File Already Exists!\x1b[m");
                    exit(1);
                }
            }
        }
    };
    write_linux_distro(&mut file);
    write_env_vars(&mut file);
}
