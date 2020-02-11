use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Output file for Dockerfile
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let args = Cli::from_args();
}
