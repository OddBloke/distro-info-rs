extern crate chrono;
extern crate clap;
extern crate distro_info;
#[macro_use]
extern crate failure;

use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono::Utc;
use clap::{App, Arg, ArgGroup};
use distro_info::{DistroRelease, UbuntuDistroInfo};
use failure::{Error, ResultExt};

enum DaysMode {
    Created,
    Release,
}

enum OutputMode {
    Codename,
    FullName,
    Release,
    Suppress,
}

fn determine_day_delta(current_date: NaiveDate, target_date: &NaiveDate) -> i64 {
    target_date.signed_duration_since(current_date).num_days()
}

fn output(
    distro_releases: Vec<&DistroRelease>,
    output_mode: OutputMode,
    days_mode: Option<DaysMode>,
    date: NaiveDate,
) -> Result<(), Error> {
    for distro_release in distro_releases {
        let mut output_parts = vec![];
        match output_mode {
            OutputMode::Codename => output_parts.push(format!("{}", &distro_release.series)),
            OutputMode::Release => output_parts.push(format!("{}", &distro_release.version)),
            OutputMode::FullName => output_parts.push(format!(
                "Ubuntu {} \"{}\"",
                &distro_release.version, &distro_release.codename
            )),
            OutputMode::Suppress => (),
        }
        let target_date = match days_mode {
            Some(DaysMode::Created) => Some(distro_release.created.ok_or(format_err!(
                "No creation date found for {}",
                &distro_release.series
            ))?),
            Some(DaysMode::Release) => Some(distro_release.release.ok_or(format_err!(
                "No release date found for {}",
                &distro_release.series
            ))?),
            None => None,
        };
        target_date.map(|target_date| {
            output_parts.push(format!("{}", determine_day_delta(date, &target_date)))
        });
        if !output_parts.is_empty() {
            println!("{}", output_parts.join(" "));
        }
    }
    Ok(())
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
        .arg(Arg::with_name("lts").long("lts"))
        .arg(Arg::with_name("series").long("series").takes_value(true))
        .arg(Arg::with_name("stable").short("s").long("stable"))
        .arg(Arg::with_name("supported").long("supported"))
        .arg(Arg::with_name("codename").short("c").long("codename"))
        .arg(Arg::with_name("fullname").short("f").long("fullname"))
        .arg(Arg::with_name("release").short("r").long("release"))
        .arg(Arg::with_name("date").long("date").takes_value(true))
        .arg(
            Arg::with_name("days")
                .long("days")
                .takes_value(true)
                .default_value("release")
                .possible_values(&["created", "release"]),
        )
        .group(
            ArgGroup::with_name("selector")
                .args(&[
                    "all",
                    "devel",
                    "latest",
                    "lts",
                    "series",
                    "stable",
                    "supported",
                ])
                .required(true),
        )
        .group(ArgGroup::with_name("output").args(&["codename", "fullname", "release"]))
        .get_matches();
    let ubuntu_distro_info = UbuntuDistroInfo::new()?;
    let date = match matches.value_of("date") {
        Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").context(format!(
            "Failed to parse date '{}'; must be YYYY-MM-DD format",
            date_str
        ))?,
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
    } else if matches.is_present("lts") {
        let mut lts_releases = vec![];
        for distro_release in ubuntu_distro_info.all_at(date) {
            if distro_release.is_lts() {
                lts_releases.push(distro_release);
            }
        }
        vec![lts_releases.last().unwrap().clone()]
    } else if matches.is_present("stable") {
        vec![ubuntu_distro_info.supported(date).last().unwrap().clone()]
    } else if matches.is_present("series") {
        match matches.value_of("series") {
            Some(needle_series) => {
                let candidates: Vec<&DistroRelease> = ubuntu_distro_info
                    .iter()
                    .filter(|distro_release| distro_release.series == needle_series)
                    .collect();
                if candidates.len() == 0 {
                    bail!("unknown distribution series `{}'", needle_series);
                };
                Ok(candidates)
            }
            None => Err(format_err!(
                "--series requires an argument; please report a bug about this \
                 error"
            )),
        }?
    } else {
        panic!("clap prevent us from reaching here; report a bug if you see this")
    };
    let days_mode = if matches.occurrences_of("days") == 0 {
        None
    } else {
        matches.value_of("days").map(|value| match value {
            "created" => DaysMode::Created,
            "release" => DaysMode::Release,
            _ => panic!("unknown days mode found; please report a bug"),
        })
    };
    if matches.is_present("fullname") {
        output(distro_releases_iter, OutputMode::FullName, days_mode, date)?;
    } else if matches.is_present("release") {
        output(distro_releases_iter, OutputMode::Release, days_mode, date)?;
    } else if matches.is_present("codename") || days_mode.is_none() {
        // This should be the default output _unless_ --days is specified
        output(distro_releases_iter, OutputMode::Codename, days_mode, date)?;
    } else {
        output(distro_releases_iter, OutputMode::Suppress, days_mode, date)?;
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
