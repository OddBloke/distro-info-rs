use std::collections::HashMap;

use anyhow::Error;
use distro_info::{DebianDistroInfo, DistroInfo};
use distro_info_binaries::{add_common_args, common_run};

fn run() -> Result<(), Error> {
    let command = add_common_args(
        "debian-distro-info",
        HashMap::from([("testing", (Some('t'), "current testing version"))]),
    );
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
