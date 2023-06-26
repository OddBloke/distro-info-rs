use std::collections::HashMap;

use anyhow::Error;
use clap::Arg;
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
            (
                "elts",
                (
                    Some('e'),
                    "list of all Extended LTS supported versions",
                    None,
                ),
            ),
            (
                "lts",
                (Some('l'), "list of all LTS supported versions", None),
            ),
            (
                "oldstable",
                (Some('o'), "latest oldstable version", Some("old")),
            ),
            ("testing", (Some('t'), "current testing version", None)),
        ]),
        additional_args: HashMap::from([
            ("alias", Arg::new("alias").long("alias").help("print the alias (oldstable, stable, testing, unstable) relative to the given distribution codename"))
        ]),
    };
    command.main(&run)
}
