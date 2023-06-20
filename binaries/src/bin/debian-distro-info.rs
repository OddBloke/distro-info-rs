use std::collections::HashMap;

use anyhow::Error;
use distro_info::{DebianDistroInfo, DistroInfo};
use distro_info_binaries::{common_run, DistroInfoCommand};

fn run() -> Result<(), Error> {
    let command = DistroInfoCommand {
        command_name: "debian-distro-info",
        additional_selectors: HashMap::from([("testing", (Some('t'), "current testing version"))]),
    }
    .create_command();
    let debian_distro_info = DebianDistroInfo::new()?;
    common_run(command, &debian_distro_info)
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        writeln!(stderr, "debian-distro-info: {}", e).unwrap();
        ::std::process::exit(1);
    }
}
