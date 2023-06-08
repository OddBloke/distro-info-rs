use chrono::NaiveDate;
use clap::{App, Arg, ArgGroup, ArgMatches};
use distro_info::{DistroInfo, DistroRelease};
use failure::{bail, format_err, Error};

pub const OUTDATED_MSG: &str = "Distribution data outdated.
Please check for an update for distro-info-data. See /usr/share/doc/distro-info-data/README.Debian for details.";

/// Add arguments common to both ubuntu- and debian-distro-info to `app`
pub fn add_common_args<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app.version("0.1.0")
        .author("Daniel Watkins <daniel@daniel-watkins.co.uk>")
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("list all known versions"),
        )
        .arg(
            Arg::with_name("devel")
                .short("d")
                .long("devel")
                .help("latest development version"),
        )
        .arg(
            Arg::with_name("series")
                .long("series")
                .takes_value(true)
                .help("series to calculate the version for"),
        )
        .arg(
            Arg::with_name("stable")
                .short("s")
                .long("stable")
                .help("latest stable version"),
        )
        .arg(
            Arg::with_name("supported")
                .long("supported")
                .help("list of all supported stable versions"),
        )
        .arg(
            Arg::with_name("unsupported")
                .long("unsupported")
                .help("list of all unsupported stable versions"),
        )
        .arg(
            Arg::with_name("codename")
                .short("c")
                .long("codename")
                .help("print the codename (default)"),
        )
        .arg(
            Arg::with_name("fullname")
                .short("f")
                .long("fullname")
                .help("print the full name"),
        )
        .arg(
            Arg::with_name("release")
                .short("r")
                .long("release")
                .help("print the release version"),
        )
        .arg(
            Arg::with_name("date")
                .long("date")
                .takes_value(true)
                .help("date for calculating the version (default: today)"),
        )
        .arg(
            Arg::with_name("days")
                .short("y")
                .long("days")
                .takes_value(true)
                .default_value("release")
                .possible_values(&["created", "eol", "eol-server", "release"])
                .value_name("milestone")
                .help("additionally, display days until milestone"),
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
                    "unsupported",
                ])
                .required(true),
        )
        .group(ArgGroup::with_name("output").args(&["codename", "fullname", "release"]))
}

pub fn select_distro_releases<'a>(
    matches: &ArgMatches,
    date: NaiveDate,
    distro_info: &'a impl DistroInfo,
) -> Result<Vec<&'a DistroRelease>, Error> {
    Ok(if matches.is_present("all") {
        distro_info.iter().collect()
    } else if matches.is_present("supported") {
        distro_info.supported(date)
    } else if matches.is_present("unsupported") {
        distro_info.unsupported(date)
    } else if matches.is_present("devel") {
        distro_info.devel(date)
    } else if matches.is_present("latest") {
        let devel_result = distro_info.devel(date);
        if devel_result.len() > 0 {
            vec![*devel_result.last().unwrap()]
        } else {
            distro_info
                .latest(date)
                .map(|distro_release| vec![distro_release])
                .unwrap_or_else(|| vec![])
        }
    } else if matches.is_present("lts") {
        let mut lts_releases = vec![];
        for distro_release in distro_info.all_at(date) {
            if distro_release.is_lts() {
                lts_releases.push(distro_release);
            }
        }
        match lts_releases.last() {
            Some(release) => vec![*release],
            None => bail!(OUTDATED_MSG),
        }
    } else if matches.is_present("stable") {
        distro_info
            .latest(date)
            .map(|distro_release| vec![distro_release])
            .unwrap_or_else(|| vec![])
    } else if matches.is_present("series") {
        match matches.value_of("series") {
            Some(needle_series) => {
                if !needle_series.chars().all(|c| c.is_lowercase()) {
                    bail!("invalid distribution series `{}'", needle_series);
                };
                let candidates: Vec<&DistroRelease> = distro_info
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
    })
}
