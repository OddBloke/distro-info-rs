extern crate anyhow;
extern crate chrono;
extern crate clap;
extern crate distro_info;

use std::collections::HashMap;

use anyhow::Error;
use distro_info::{DistroInfo, UbuntuDistroInfo};
use distro_info_binaries::DistroInfoCommand;

fn run(command: DistroInfoCommand) -> Result<(), Error> {
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    command.run(&ubuntu_distro_info)
}

fn main() {
    let command = DistroInfoCommand {
        command_name: "ubuntu-distro-info",
        additional_selectors: HashMap::from([
            ("latest", (Some('l'), "", None)),
            (
                "lts",
                (None, "latest long term support (LTS) version", None),
            ),
        ]),
        additional_args: HashMap::new(),
    };
    command.main(&run)
}
