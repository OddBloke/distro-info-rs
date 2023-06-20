use std::collections::HashMap;

use anyhow::Error;
use distro_info::{DebianDistroInfo, DistroInfo};
use distro_info_binaries::{common_run, DistroInfoCommand};

fn run(command: DistroInfoCommand) -> Result<(), Error> {
    let debian_distro_info = DebianDistroInfo::new()?;
    common_run(command, &debian_distro_info)
}

fn main() {
    let command = DistroInfoCommand {
        command_name: "debian-distro-info",
        additional_selectors: HashMap::from([("testing", (Some('t'), "current testing version"))]),
    };
    command.main(&run)
}
