use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

/*
  This file is expected to provide backend for software to use required crates for it's runtime in main.rs while allowing for crate usage
*/

pub mod modules {
  pub mod detection;
  pub mod write;
}

#[derive(StructOpt)]
pub struct Cli {
    /// Output file for Dockerfile
    #[structopt(short = "o", long = "output")]
    pub file: Option<String>,
    /// Optimize Dockerfile for Gitpod
    #[structopt(short, long, parse(from_flag))]
    pub gitpod: bool,
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
