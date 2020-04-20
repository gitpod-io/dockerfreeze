use std::fs::File;
use std::io::Write;
use structopt::StructOpt;
use std::fs::OpenOptions;
use std::process::exit;

#[derive(StructOpt)]
pub struct Cli {
    /// Output file for Dockerfile
    #[structopt(short = "o", long = "output")]
    pub file: Option<String>,
    /// Optimize Dockerfile for Gitpod
    #[structopt(short, long, parse(from_flag))]
    pub gitpod: bool,
    /// Do Not Add programs from path
    #[structopt(short, long, parse(from_flag))]
    pub no_path: bool
}

pub fn match_dist(distro: &str, file: &mut File) -> usize {
    match distro {
        "ubuntu" => file.write(b"FROM ubuntu:latest\n").unwrap(),
        "alpine" => file.write(b"FROM alpine:latest\n").unwrap(),
        "debian" => file.write(b"FROM debian:latest\n").unwrap(),
        _ => {
            println!(
                "\x1b[33mUnknown Distro \"{}\" Re-routing to Ubuntu\x1b[0m",
                distro
            );
            file.write("FROM ubuntu:latest\n".as_bytes()).unwrap()
        }
    }
}

pub fn initialize(args: &Cli) -> File {
    let file = match &args.file {
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
    file
}