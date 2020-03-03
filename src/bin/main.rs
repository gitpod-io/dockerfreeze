// Created by Sean Hellum <seanhellum45@gmail.com> under MIT license (https://opensource.org/licenses/MIT) in 2020
// Refactored by Jacob Hrbek <kreyren@rixotstudio.cz> under MIT license (https://opensource.org/licenses/MIT) in 2020

/*
  Capture your local development environment into a dockerfile

  We are expecting the end-user to run this on their local system for us to generate a dockerfile
*/

use dockerfreeze::modules::write::write_env_vars;
use dockerfreeze::modules::write::write_linux_distro;
use dockerfreeze::Cli;
use std::fs::OpenOptions;
use std::process::exit;
use structopt::StructOpt;

use std::path::Path;
use os_detect::detect_os_from_path;
//use os_detect::detect_windows; -- Stubbed

fn main() {
    // Kernel detection
    // This exports OS enum with following data https://docs.rs/os-detect/0.2.2/os_detect/enum.OS.html
    // FIXME: Export in lib.rs so that this can be used as a crate on demand
    // FIXME-bench: I assume that implementing it this way is the most efficient? Bench required -- Krey
    if cfg!(target_os = "unix") {
        detect_os_from_path(Path::new("/"))
          .expect("Failed to detect os from path, unsupported kernel?");
    } else if cfg!(target_os = "windows") {
        // FIXME-QA: Use die() ?
        println!("Windows is currently not supported, fixme?");
        exit(1);
        //detect_windows(Path::new("c:/"));
    } else {
        // FIXME: Output the kernel
        println!("This {:?} kernel is not supported\n", "FIXME_DETECT_OS");
        exit(255);
    }

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
