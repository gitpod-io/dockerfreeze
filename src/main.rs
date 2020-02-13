use std::env::vars;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Output file for Dockerfile
    #[structopt(short = "o", long = "output")]
    file: Option<String>,
}

fn write_linux_distro(file: &mut File) {
    // Read /etc/os-release
    let mut content = String::new();
    let mut os_release = File::open("/etc/os-release").unwrap();
    let regex = regex::Regex::new(".*ID=(.*)\n").unwrap();
    os_release.read_to_string(&mut content).unwrap();
    let distro = regex.captures(&content).unwrap().get(1).unwrap().as_str();
    match distro {
        "ubuntu" => {
            file.write(b"FROM ubuntu:latest\n").unwrap();
        },
        "alpine" => {
            file.write(b"FROM alpine:latest\n").unwrap();
        },
        "debian" => {
            file.write(b"FROM debian:latest\n").unwrap();
        },
        _ => {
            println!("\x1b[33mUnknown Distro Re-routing to Ubuntu\x1b[0m");
            file.write("FROM ubuntu\n".as_bytes()).unwrap();
        }
    }
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
        count += 1;
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
    write_linux_distro(&mut file);
    write_env_vars(&mut file);
}
