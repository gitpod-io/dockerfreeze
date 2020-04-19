use crate::detection::get_distro;
use std::process::Command;
use std::str::from_utf8;
use rayon::prelude::*;

pub fn get_packages() -> Vec<String> {
    let distro = get_distro();
    match distro.as_str() {
        "ubuntu" => get_packages_from_cmd("apt list --installed"),
        "alpine" => get_packages_from_cmd("apk info"),
        "debian" => get_packages_from_cmd("apt list --installed"),
        _ => {
            println!(
                "\x1b[33mUnknown Distro \"{}\" Re-routing to Ubuntu\x1b[0m",
                distro
            );
            get_packages_from_cmd("apt list --installed")
        }
    }
}

fn get_packages_from_cmd(cmd: &str) -> Vec<String> {
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process")
        .stdout;
    let output = from_utf8(&cmd).unwrap();
    output
        .par_split('\n')
        .map(|s| s.to_string())
        .filter(|i| !i.is_empty())
        .map(|i| {
            i.clone()
                .par_split('/')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .get(0)
                .unwrap()
                .clone()
        })
        .collect()
}
