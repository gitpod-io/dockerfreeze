use std::env::vars;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;
use crate::modules::detection::get_distro;
use crate::*; // hotfix

pub fn write_linux_distro(file: &mut File) {
    let distro = get_distro();
    let args = Cli::from_args();
    match args.gitpod {
        true => file.write(b"FROM gitpod/workspace-full:latest\n").unwrap(),
        false => match_dist(&distro, file),
    };
}

pub fn write_env_vars(file: &mut File) {
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
