extern crate chrono;
extern crate clap;
extern crate distro_info;
extern crate failure;

use chrono::Datelike;
use chrono::Utc;
use chrono::naive::NaiveDate;
use clap::{Arg, ArgGroup, App};
use distro_info::{DistroRelease, UbuntuDistroInfo};
use failure::{Error, ResultExt};

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

fn today() -> NaiveDate {
    let now = Utc::now();
    NaiveDate::from_ymd(now.year(), now.month(), now.day())
}

fn run() -> Result<(), Error> {
    let matches = App::new("ubuntu-distro-info")
        .version("0.1.0")
        .author("Daniel Watkins <daniel@daniel-watkins.co.uk>")
        .arg(Arg::with_name("all").short("a").long("all"))
        .arg(Arg::with_name("devel").short("d").long("devel"))
        .arg(Arg::with_name("latest").short("l").long("latest"))
        .arg(Arg::with_name("supported").long("supported"))
        .arg(Arg::with_name("codename").short("c").long("codename"))
        .arg(Arg::with_name("fullname").short("f").long("fullname"))
        .arg(Arg::with_name("release").short("r").long("release"))
        .arg(Arg::with_name("date").long("date").takes_value(true))
        .group(ArgGroup::with_name("selector")
            .args(&["all", "devel", "latest", "supported"])
            .required(true))
        .group(ArgGroup::with_name("output").args(&["codename", "fullname", "release"]))
        .get_matches();
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    let date = match matches.value_of("date") {
        Some(date_str) =>
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").context(
                format!("Failed to parse date '{}'; must be YYYY-MM-DD format", date_str))?,
        None => today(),
    };
    let distro_releases_iter = if matches.is_present("all") {
        ubuntu_distro_info.iter().collect()
    } else if matches.is_present("supported") {
        ubuntu_distro_info.supported(date)
    } else if matches.is_present("devel") {
        ubuntu_distro_info.devel(date)
    } else if matches.is_present("latest") {
        vec![ubuntu_distro_info.latest(date)]
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
