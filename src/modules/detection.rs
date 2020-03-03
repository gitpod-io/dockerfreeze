use std::fs::File;
use std::io::Read;

pub fn get_distro() -> String {
    let mut content = String::new();
    let mut os_release = File::open("/etc/os-release").unwrap();
    let regex = regex::Regex::new(".*\nID=(.*)\n").unwrap();
    os_release.read_to_string(&mut content).unwrap();
    return regex
        .captures(&content)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_owned();
}
