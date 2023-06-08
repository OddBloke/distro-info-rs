extern crate chrono;
extern crate clap;
extern crate distro_info;
extern crate failure;

use clap::{App, Arg};
use distro_info::{DistroInfo, UbuntuDistroInfo};
use distro_info_binaries::{add_common_args, common_run};
use failure::Error;

fn run() -> Result<(), Error> {
    let additional_selectors = &["latest", "lts"];
    let app = add_common_args(App::new("ubuntu-distro-info"), additional_selectors)
        .arg(Arg::with_name("latest").short("l").long("latest"))
        .arg(
            Arg::with_name("lts")
                .long("lts")
                .help("latest long term support (LTS) version"),
        );
    let matches = app.get_matches();
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
