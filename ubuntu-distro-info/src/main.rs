extern crate chrono;
extern crate clap;
extern crate distro_info;
extern crate failure;

use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono::Utc;
use clap::{App, Arg};
use distro_info::{DistroInfo, UbuntuDistroInfo};
use failure::{Error, ResultExt};
use ubuntu_distro_info::{add_common_args, output, select_distro_releases, DaysMode, OutputMode};

fn today() -> NaiveDate {
    let now = Utc::now();
    NaiveDate::from_ymd(now.year(), now.month(), now.day())
}

fn run() -> Result<(), Error> {
    let app = add_common_args(App::new("ubuntu-distro-info"))
        .arg(Arg::with_name("latest").short("l").long("latest"))
        .arg(
            Arg::with_name("lts")
                .long("lts")
                .help("latest long term support (LTS) version"),
        );
    let matches = app.get_matches();
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    let date = match matches.value_of("date") {
        Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").context(format!(
            "Failed to parse date '{}'; must be YYYY-MM-DD format",
            date_str
        ))?,
        None => today(),
    };
    let distro_releases_iter = select_distro_releases(&matches, date, &ubuntu_distro_info)?;
    let days_mode = if matches.occurrences_of("days") == 0 {
        None
    } else {
        matches.value_of("days").map(|value| match value {
            "created" => DaysMode::Created,
            "eol" => DaysMode::Eol,
            "eol-server" => DaysMode::EolServer,
            "release" => DaysMode::Release,
            _ => panic!("unknown days mode found; please report a bug"),
        })
    };
    if matches.is_present("fullname") {
        output(
            distro_releases_iter,
            &OutputMode::FullName,
            &days_mode,
            date,
        )?;
    } else if matches.is_present("release") {
        output(distro_releases_iter, &OutputMode::Release, &days_mode, date)?;
    } else if matches.is_present("codename") || days_mode.is_none() {
        // This should be the default output _unless_ --days is specified
        output(
            distro_releases_iter,
            &OutputMode::Codename,
            &days_mode,
            date,
        )?;
    } else {
        output(
            distro_releases_iter,
            &OutputMode::Suppress,
            &days_mode,
            date,
        )?;
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        writeln!(stderr, "ubuntu-distro-info: {}", e).unwrap();
        ::std::process::exit(1);
    }
}
