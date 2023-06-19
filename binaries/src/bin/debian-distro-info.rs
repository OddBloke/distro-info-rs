use anyhow::Error;
use clap::{Arg, ArgAction, Command};
use distro_info::{DebianDistroInfo, DistroInfo};
use distro_info_binaries::{add_common_args, common_run};

fn run() -> Result<(), Error> {
    let app = add_common_args(Command::new("debian-distro-info"), &["testing"]).arg(
        Arg::new("testing")
            .action(ArgAction::SetTrue)
            .short('t')
            .long("testing")
            .help("current testing version"),
    );
    let matches = app.try_get_matches()?;
    let debian_distro_info = DebianDistroInfo::new()?;
    common_run(&matches, &debian_distro_info)
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        writeln!(stderr, "debian-distro-info: {}", e).unwrap();
        ::std::process::exit(1);
    }
}
