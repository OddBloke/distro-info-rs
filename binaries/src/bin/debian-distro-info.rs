use anyhow::Error;
use clap::Arg;
use distro_info::{DebianDistroInfo, DistroInfo};
use distro_info_binaries::{flag, DistroInfoCommand};

fn run(command: DistroInfoCommand) -> Result<(), Error> {
    let debian_distro_info = DebianDistroInfo::new()?;
    command.run(&debian_distro_info)
}

fn main() {
    let command = DistroInfoCommand {
        command_name: "debian-distro-info",
        additional_selectors: vec![
            flag("elts", Some('e'), "list of all Extended LTS supported versions", None),
            flag("lts", Some('l'), "list of all LTS supported versions", None),
            flag("oldstable", Some('o'), "latest oldstable version", Some("old")),
            flag("testing", Some('t'), "current testing version", None),
            Arg::new("alias").long("alias").help("print the alias (oldstable, stable, testing, unstable) relative to the given distribution codename"),
        ],
    };
    command.main(&run)
}
