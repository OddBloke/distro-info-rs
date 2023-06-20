extern crate anyhow;
extern crate chrono;
extern crate clap;
extern crate distro_info;

use std::collections::HashMap;

use anyhow::Error;
use distro_info::{DistroInfo, UbuntuDistroInfo};
use distro_info_binaries::{common_run, DistroInfoCommand};

fn run() -> Result<(), Error> {
    let command = DistroInfoCommand {
        command_name: "ubuntu-distro-info",
        additional_selectors: HashMap::from([
            ("latest", (Some('l'), "")),
            ("lts", (None, "latest long term support (LTS) version")),
        ]),
    }
    .create_command();
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    common_run(command, &ubuntu_distro_info)
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        writeln!(stderr, "ubuntu-distro-info: {}", e).unwrap();
        ::std::process::exit(1);
    }
}
