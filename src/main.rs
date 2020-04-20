mod detection;
mod lib;
mod path;
mod write;

use crate::lib::initialize;
use crate::lib::Cli;
use crate::write::write_env_vars;
use crate::write::write_linux_distro;
use crate::write::add_programs;

use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let mut file = initialize(&args);
    write_linux_distro(&mut file);
    write_env_vars(&mut file);
    if !&args.no_path {
        add_programs(&mut file);
    }
}
