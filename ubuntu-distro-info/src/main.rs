extern crate chrono;
extern crate clap;
extern crate distro_info;
extern crate failure;

use chrono::Datelike;
use chrono::Utc;
use chrono::naive::NaiveDate;
use clap::{Arg, ArgGroup, App};
use distro_info::{DistroRelease, UbuntuDistroInfo};
use failure::Error;

fn all<'a>(ubuntu_distro_info: &'a UbuntuDistroInfo) -> Vec<&'a DistroRelease> {
    ubuntu_distro_info.iter().collect()
}

fn supported<'a>(ubuntu_distro_info: &'a UbuntuDistroInfo) -> Vec<&'a DistroRelease> {
    let now = Utc::now();
    let today = NaiveDate::from_ymd(now.year(), now.month(), now.day());
    ubuntu_distro_info.supported(today)
}

enum OutputMode {
    Codename,
    FullName,
    Release,
}

fn output(distro_releases: Vec<&DistroRelease>, output_mode: OutputMode) {
    for distro_release in distro_releases {
        println!("{}",
                 match output_mode {
                     OutputMode::Codename => &distro_release.series,
                     OutputMode::FullName => &distro_release.codename,
                     OutputMode::Release => &distro_release.version,
                 });
    }
}

fn run() -> Result<(), Error> {
    let matches = App::new("ubuntu-distro-info")
        .version("0.1.0")
        .author("Daniel Watkins <daniel@daniel-watkins.co.uk>")
        .arg(Arg::with_name("all").short("a").long("all"))
        .arg(Arg::with_name("supported").long("supported"))
        .arg(Arg::with_name("codename").short("c").long("codename"))
        .arg(Arg::with_name("fullname").short("f").long("fullname"))
        .arg(Arg::with_name("release").short("r").long("release"))
        .group(ArgGroup::with_name("selector").args(&["all", "supported"]).required(true))
        .group(ArgGroup::with_name("output").args(&["codename", "fullname", "release"]))
        .get_matches();
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    let distro_releases_iter = if matches.is_present("all") {
        all(&ubuntu_distro_info)
    } else if matches.is_present("supported") {
        supported(&ubuntu_distro_info)
    } else {
        panic!("clap prevent us from reaching here; report a bug if you see this")
    };
    if matches.is_present("fullname") {
        output(distro_releases_iter, OutputMode::FullName);
    } else if matches.is_present("release") {
        output(distro_releases_iter, OutputMode::Release);
    } else {
        output(distro_releases_iter, OutputMode::Codename);
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        writeln!(stderr, "error: {:?}", e).unwrap();
        ::std::process::exit(1);
    }
}
