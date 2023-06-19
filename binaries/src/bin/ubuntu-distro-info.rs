extern crate anyhow;
extern crate chrono;
extern crate clap;
extern crate distro_info;

use anyhow::Error;
use clap::{Arg, ArgAction, Command};
use distro_info::{DistroInfo, UbuntuDistroInfo};
use distro_info_binaries::{add_common_args, common_run, flag};

fn run() -> Result<(), Error> {
    let additional_selectors = &["latest", "lts"];
    let app = add_common_args(Command::new("ubuntu-distro-info"), additional_selectors)
        .arg(flag("latest", 'l', ""))
        .arg(
            Arg::new("lts")
                .action(ArgAction::SetTrue)
                .long("lts")
                .help("latest long term support (LTS) version"),
        );
    let matches = app.try_get_matches()?;
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    common_run(&matches, &ubuntu_distro_info)
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        writeln!(stderr, "ubuntu-distro-info: {}", e).unwrap();
        ::std::process::exit(1);
    }
}
