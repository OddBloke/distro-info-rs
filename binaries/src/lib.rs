use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Utc;
use clap::{App, Arg, ArgGroup, ArgMatches};
use distro_info::{DistroInfo, DistroRelease};
use failure::{bail, format_err, Error, ResultExt};

pub const OUTDATED_MSG: &str = "Distribution data outdated.
Please check for an update for distro-info-data. See /usr/share/doc/distro-info-data/README.Debian for details.";

pub enum DaysMode {
    Created,
    Eol,
    EolServer,
    Release,
}

pub enum OutputMode {
    Codename,
    FullName,
    Release,
    Suppress,
}

/// Add arguments common to both ubuntu- and debian-distro-info to `app`
pub fn add_common_args<'a>(app: App<'a, 'a>, additional_selectors: &'a [&str]) -> App<'a, 'a> {
    let mut selectors = vec![
        "all",
        "devel",
        "series",
        "stable",
        "supported",
        "unsupported",
    ];
    selectors.extend(additional_selectors);
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
                .args(&selectors)
                .required(true),
        )
        .group(ArgGroup::with_name("output").args(&["codename", "fullname", "release"]))
}

pub fn common_run(matches: &ArgMatches, distro_info: &impl DistroInfo) -> Result<(), Error> {
    let date = match matches.value_of("date") {
        Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").context(format!(
            "Failed to parse date '{}'; must be YYYY-MM-DD format",
            date_str
        ))?,
        None => today(),
    };
    let distro_releases_iter = select_distro_releases(&matches, date, distro_info)?;
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
    let distro_name = distro_info.distro_name();
    if matches.is_present("fullname") {
        output(
            distro_name,
            distro_releases_iter,
            &OutputMode::FullName,
            &days_mode,
            date,
        )?;
    } else if matches.is_present("release") {
        output(
            distro_name,
            distro_releases_iter,
            &OutputMode::Release,
            &days_mode,
            date,
        )?;
    } else if matches.is_present("codename") || days_mode.is_none() {
        // This should be the default output _unless_ --days is specified
        output(
            distro_name,
            distro_releases_iter,
            &OutputMode::Codename,
            &days_mode,
            date,
        )?;
    } else {
        output(
            distro_name,
            distro_releases_iter,
            &OutputMode::Suppress,
            &days_mode,
            date,
        )?;
    }
    Ok(())
}

fn determine_day_delta(current_date: NaiveDate, target_date: NaiveDate) -> i64 {
    target_date.signed_duration_since(current_date).num_days()
}

pub fn output(
    distro_name: &str,
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
                "{} {} \"{}\"",
                distro_name,
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
    } else if matches.is_present("devel") || matches.is_present("testing") {
        // u-d-i --devel and d-d-i --testing share selection logic
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

fn today() -> NaiveDate {
    let now = Utc::now();
    NaiveDate::from_ymd(now.year(), now.month(), now.day())
}
