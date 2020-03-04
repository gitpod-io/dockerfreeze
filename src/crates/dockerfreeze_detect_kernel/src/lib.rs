//! FIXME-DOCS: Documentation

// Kernel detection
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
        //detect_windows(Path::new("c:/"));
    } else {
        // FIXME: Output the kernel
        println!("This {:?} kernel is not supported\n", "FIXME_DETECT_OS");
        exit(255);
    }
}