// Created by Sean Hellum <seanhellum45@gmail.com> under MIT license (https://opensource.org/licenses/MIT) in 2020
// Refactored by Jacob Hrbek <kreyren@rixotstudio.cz> under MIT license (https://opensource.org/licenses/MIT) in 2020

/*
  Freeze your development environment as a Dockerfile

  FIXME: Abstract
*/

use dockerfreeze::modules::write::write_env_vars;
use dockerfreeze::modules::write::write_linux_distro;
use dockerfreeze::Cli;
use std::fs::OpenOptions;
use std::process::exit;
use structopt::StructOpt;

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
