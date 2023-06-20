extern crate anyhow;
extern crate chrono;
extern crate clap;
extern crate distro_info;

use std::collections::HashMap;

use anyhow::Error;
use distro_info::{DistroInfo, UbuntuDistroInfo};
use distro_info_binaries::{common_run, DistroInfoCommand};

fn run(command: DistroInfoCommand) -> Result<(), Error> {
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    common_run(command, &ubuntu_distro_info)
}

fn main() {
    let command = DistroInfoCommand {
        command_name: "ubuntu-distro-info",
        additional_selectors: HashMap::from([
            ("latest", (Some('l'), "")),
            ("lts", (None, "latest long term support (LTS) version")),
        ]),
    };
    command.main(&run)
}
