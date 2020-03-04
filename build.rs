fn main() {
    subprocess::Exec::shell("echo \"source ${PWD}/tools/dockerfreeze\" >> ~/.bashrc")
        .stdout(subprocess::Redirection::None)
        .join()
        .unwrap();
}
