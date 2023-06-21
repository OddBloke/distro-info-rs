use std::collections::HashMap;

use anyhow::Error;
use distro_info::{DebianDistroInfo, DistroInfo};
use distro_info_binaries::DistroInfoCommand;

fn run(command: DistroInfoCommand) -> Result<(), Error> {
    let debian_distro_info = DebianDistroInfo::new()?;
    command.run(&debian_distro_info)
}

fn main() {
    let command = DistroInfoCommand {
        command_name: "debian-distro-info",
        additional_selectors: HashMap::from([
            ("oldstable", (Some('o'), "latest oldstable version")),
            ("testing", (Some('t'), "current testing version")),
        ]),
    };
    command.main(&run)
}
