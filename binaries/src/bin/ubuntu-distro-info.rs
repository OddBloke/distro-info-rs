extern crate anyhow;
extern crate chrono;
extern crate clap;
extern crate distro_info;

use anyhow::Error;
use distro_info::{DistroInfo, UbuntuDistroInfo};
use distro_info_binaries::{flag, DistroInfoCommand};

fn run(command: DistroInfoCommand) -> Result<(), Error> {
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    command.run(&ubuntu_distro_info)
}

fn main() {
    let command = DistroInfoCommand {
        command_name: "ubuntu-distro-info",
        additional_selectors: vec![
            flag("latest", Some('l'), "", None),
            flag("lts", None, "latest long term support (LTS) version", None),
        ],
    };
    command.main(&run)
}
