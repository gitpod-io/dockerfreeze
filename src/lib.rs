
//! FIXME-DOCS: Documentation

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

// Kernel detection
// FIXME: Export in invidual crate
// This exports 'OS' enum with following data https://docs.rs/os-detect/0.2.2/os_detect/enum.OS.html used to determine kernel and if applies distro used
pub fn kernel_detect() {
    use std::path::Path;
    use os_detect::detect_os_from_path;
    //use os_detect::detect_windows; -- Stubbed for windows support
    use std::process::exit;

    // FIXME-bench: I assume that implementing it this way is the most efficient? Bench required -- Krey
    if cfg!(target_os = "unix") {
        detect_os_from_path(Path::new("/"))
          .expect("Failed to detect os from path, unsupported kernel?");
    } else if cfg!(target_os = "windows") {
        // FIXME-QA: Use die() ?
        println!("Windows is currently not supported, fixme?");
        exit(1);
        //detect_windows(Path::new("c:/"))
        //.expect("Failed to detect windows");
    } else {
        // FIXME: Output the kernel
        println!("This {:?} kernel is not supported\n", "FIXME_DETECT_OS");
        exit(255);
}

// Detect File System Hierarchy
// FIXME: Export as an invidual crate?
pub fn detect_filesystem_hierarchy() {
    if cfg!(target_os = "unix") {
        // See File System Hierarchy Standard https://refspecs.linuxfoundation.org/fhs.shtml
        // FIXME-QA: What about lib32 and lib64?
        // FIXME-QA: Not all systems follow standard for /srv
        // FIXME-QA: Needs more definitions
        let fsh3_0 = ["/bin", "/boot", "/etc", "/dev", "/etc", "/home", "/lib", "/media", "/mnt", "/opt", "/root", "/run", "/sbin", "/srv", "/tmp", "/usr" ];

    } else if cfg!(target_os = "windows") {
        unimplemented!("Windows is not implemented in detect_filesystem_hierarchy function")
    } else {
        // FIXME: Add msg
        exit(255);
    }
}

// DO_NOT_MERGE: Stubbed for removal due replacement with os-detect
// pub fn match_dist(distro: &str, file: &mut File) -> usize {
//     match distro {
//         "ubuntu" => file.write(b"FROM ubuntu:latest\n").unwrap(),
//         "alpine" => file.write(b"FROM alpine:latest\n").unwrap(),
//         "debian" => file.write(b"FROM debian:latest\n").unwrap(),
//         _ => {
//             println!(
//                 "\x1b[33mUnknown Distro \"{}\" Re-routing to Ubuntu\x1b[0m",
//                 distro
//             );
//             file.write("FROM ubuntu:latest\n".as_bytes()).unwrap()
//         }
//     }
// }
