extern crate chrono;
extern crate clap;
extern crate distro_info;
#[macro_use]
extern crate failure;

use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono::Utc;
use clap::{App, Arg};
use distro_info::{DistroInfo, DistroRelease, UbuntuDistroInfo};
use failure::{Error, ResultExt};
use ubuntu_distro_info::add_common_args;

enum DaysMode {
    Created,
    Eol,
    EolServer,
    Release,
}

enum OutputMode {
    Codename,
    FullName,
    Release,
    Suppress,
}

const OUTDATED_MSG: &str = "Distribution data outdated.
Please check for an update for distro-info-data. See /usr/share/doc/distro-info-data/README.Debian for details.";

fn determine_day_delta(current_date: NaiveDate, target_date: NaiveDate) -> i64 {
    target_date.signed_duration_since(current_date).num_days()
}

fn output(
    distro_releases: Vec<&DistroRelease>,
    output_mode: &OutputMode,
    days_mode: &Option<DaysMode>,
    date: NaiveDate,
) -> Result<(), Error> {
    if distro_releases.len() == 0 {
        bail!(OUTDATED_MSG);
    }
    for distro_release in distro_releases {
        let mut output_parts = vec![];
        match output_mode {
            OutputMode::Codename => output_parts.push(distro_release.series().to_string()),
            OutputMode::Release => output_parts.push(distro_release.version().to_string()),
            OutputMode::FullName => output_parts.push(format!(
                "Ubuntu {} \"{}\"",
                &distro_release.version(),
                &distro_release.codename()
            )),
            OutputMode::Suppress => (),
        }
        let target_date = match days_mode {
            Some(DaysMode::Created) => Some(distro_release.created().ok_or(format_err!(
                "No creation date found for {}",
                &distro_release.series()
            ))?),
            Some(DaysMode::Eol) => Some(distro_release.eol().ok_or(format_err!(
                "No EOL date found for {}",
                &distro_release.series()
            ))?),
            Some(DaysMode::EolServer) => *distro_release.eol_server(),
            Some(DaysMode::Release) => Some(distro_release.release().ok_or(format_err!(
                "No release date found for {}",
                &distro_release.series()
            ))?),
            None => None,
        };
        match target_date {
            Some(target_date) => {
                output_parts.push(format!("{}", determine_day_delta(date, target_date)));
            }
            None => {
                if let Some(DaysMode::EolServer) = days_mode {
                    output_parts.push("(unknown)".to_string())
                }
            }
        };
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
    let distro_releases_iter = if matches.is_present("all") {
        ubuntu_distro_info.iter().collect()
    } else if matches.is_present("supported") {
        ubuntu_distro_info.supported(date)
    } else if matches.is_present("unsupported") {
        ubuntu_distro_info.unsupported(date)
    } else if matches.is_present("devel") {
        ubuntu_distro_info.devel(date)
    } else if matches.is_present("latest") {
        let devel_result = ubuntu_distro_info.devel(date);
        if devel_result.len() > 0 {
            vec![*devel_result.last().unwrap()]
        } else {
            ubuntu_distro_info
                .latest(date)
                .map(|distro_release| vec![distro_release])
                .unwrap_or_else(|| vec![])
        }
    } else if matches.is_present("lts") {
        let mut lts_releases = vec![];
        for distro_release in ubuntu_distro_info.all_at(date) {
            if distro_release.is_lts() {
                lts_releases.push(distro_release);
            }
        }
        match lts_releases.last() {
            Some(release) => vec![*release],
            None => bail!(OUTDATED_MSG),
        }
    } else if matches.is_present("stable") {
        ubuntu_distro_info
            .latest(date)
            .map(|distro_release| vec![distro_release])
            .unwrap_or_else(|| vec![])
    } else if matches.is_present("series") {
        match matches.value_of("series") {
            Some(needle_series) => {
                if !needle_series.chars().all(|c| c.is_lowercase()) {
                    bail!("invalid distribution series `{}'", needle_series);
                };
                let candidates: Vec<&DistroRelease> = ubuntu_distro_info
                    .iter()
                    .filter(|distro_release| distro_release.series() == needle_series)
                    .collect();
                if candidates.is_empty() {
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
