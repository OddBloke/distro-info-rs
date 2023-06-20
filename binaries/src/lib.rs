use std::collections::HashMap;

use anyhow::{bail, format_err, Context, Error};
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Utc;
use clap::{crate_version, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use distro_info::Distro;
use distro_info::{DistroInfo, DistroRelease};

pub const OUTDATED_MSG: &str = "Distribution data outdated.
Please check for an update for distro-info-data. See /usr/share/doc/distro-info-data/README.Debian for details.";

#[derive(Clone)]
pub enum DaysMode {
    Created,
    Eol,
    EolServer,
    Release,
}

impl DaysMode {
    pub fn from_cli(value: &str) -> Result<Self, Error> {
        match value {
            "created" => Ok(Self::Created),
            "eol" => Ok(Self::Eol),
            "eol-server" => Ok(Self::EolServer),
            "release" => Ok(Self::Release),
            _ => bail!("unknown days mode found; please report a bug"),
        }
    }
}

pub enum OutputMode {
    Codename,
    FullName,
    Release,
    Suppress,
}

pub fn flag(name: &'static str, short: Option<char>, help: &'static str) -> Arg {
    Arg::new(name)
        .action(ArgAction::SetTrue)
        .short(short)
        .long(name)
        .help(help)
}

pub struct DistroInfoCommand {
    pub command_name: &'static str,
    pub additional_selectors: HashMap<&'static str, (Option<char>, &'static str)>,
}

impl DistroInfoCommand {
    /// Add arguments common to both ubuntu- and debian-distro-info to `app`
    pub fn create_command(self) -> Command {
        let mut selectors = vec![
            "all",
            "devel",
            "series",
            "stable",
            "supported",
            "unsupported",
        ];
        selectors.extend(self.additional_selectors.keys());
        let mut command = Command::new(self.command_name)
            .version(crate_version!())
            .author("Daniel Watkins <daniel@daniel-watkins.co.uk>")
            .arg(flag("all", Some('a'), "list all known versions"))
            .arg(flag("devel", Some('d'), "latest development version"))
            .arg(
                Arg::new("series")
                    .long("series")
                    .help("series to calculate the version for"),
            )
            .arg(flag("stable", Some('s'), "latest stable version"))
            .arg(flag(
                "supported",
                None,
                "list of all supported stable versions",
            ))
            .arg(flag(
                "unsupported",
                None,
                "list of all unsupported stable versions",
            ))
            .arg(flag("codename", Some('c'), "print the codename (default)"))
            .arg(flag("fullname", Some('f'), "print the full name"))
            .arg(flag("release", Some('r'), "print the release version"))
            .arg(
                Arg::new("date")
                    .long("date")
                    .help("date for calculating the version (default: today)"),
            )
            .arg(
                Arg::new("days")
                    .short('y')
                    .long("days")
                    .default_missing_value("release")
                    .num_args(0..=1)
                    .value_parser(DaysMode::from_cli)
                    .value_name("milestone")
                    .help("additionally, display days until milestone"),
            )
            .group(ArgGroup::new("selector").args(&selectors).required(true))
            .group(ArgGroup::new("output").args(["codename", "fullname", "release"]));
        for (long, (short, help)) in self.additional_selectors {
            command = command.arg(flag(long, short, help));
        }
        command
    }

    pub fn main(self, run: &dyn Fn(DistroInfoCommand) -> Result<(), Error>) {
        let command_name = self.command_name;
        if let Err(ref e) = run(self) {
            use std::io::Write;
            let stderr = &mut ::std::io::stderr();
            writeln!(stderr, "{}: {}", command_name, e).unwrap();
            ::std::process::exit(1);
        }
    }

    pub fn run(self, distro_info: &impl DistroInfo) -> Result<(), Error> {
        let command = self.create_command();
        let matches = command.try_get_matches()?;
        let date = match matches.get_one::<String>("date") {
            Some(date_str) => {
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d").with_context(|| {
                    format!(
                        "Failed to parse date '{}'; must be YYYY-MM-DD format",
                        date_str
                    )
                })?
            }
            None => today(),
        };
        let distro_releases_iter = select_distro_releases(&matches, date, distro_info)?;
        let days_mode = matches.get_one::<DaysMode>("days");
        let distro_name = distro_info.distro().to_string();
        if matches.get_flag("fullname") {
            output(
                distro_name,
                distro_releases_iter,
                &OutputMode::FullName,
                &days_mode,
                date,
            )?;
        } else if matches.get_flag("release") {
            output(
                distro_name,
                distro_releases_iter,
                &OutputMode::Release,
                &days_mode,
                date,
            )?;
        } else if matches.get_flag("codename") || days_mode.is_none() {
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
}

fn determine_day_delta(current_date: NaiveDate, target_date: NaiveDate) -> i64 {
    target_date.signed_duration_since(current_date).num_days()
}

pub fn output(
    distro_name: &str,
    distro_releases: Vec<&DistroRelease>,
    output_mode: &OutputMode,
    days_mode: &Option<&DaysMode>,
    date: NaiveDate,
) -> Result<(), Error> {
    if distro_releases.is_empty() {
        bail!(OUTDATED_MSG);
    }
    for distro_release in distro_releases {
        let mut output_parts = vec![];
        match output_mode {
            OutputMode::Codename => output_parts.push(distro_release.series().to_string()),
            OutputMode::Release => output_parts.push(
                distro_release
                    .version()
                    .as_ref()
                    .unwrap_or_else(|| distro_release.series())
                    .to_string(),
            ),
            OutputMode::FullName => output_parts.push(format!(
                "{} {} \"{}\"",
                distro_name,
                match distro_release.version() {
                    Some(version) => version,
                    None => "",
                },
                &distro_release.codename()
            )),
            OutputMode::Suppress => (),
        }
        let target_date = match days_mode {
            Some(DaysMode::Created) => Some(distro_release.created().ok_or(format_err!(
                "No creation date found for {}",
                &distro_release.series()
            ))?),
            Some(DaysMode::Eol) => *distro_release.eol(),
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
            None => match days_mode {
                Some(DaysMode::EolServer) | Some(DaysMode::Eol) => {
                    output_parts.push("(unknown)".to_string())
                }
                _ => (),
            },
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
    let get_maybe_missing_flag = |name: &str| -> bool {
        match matches.try_get_one::<bool>(name) {
            Ok(flag) => *flag.unwrap_or(&false),
            Err(_) => false,
        }
    };
    Ok(if matches.get_flag("all") {
        distro_info.iter().collect()
    } else if matches.get_flag("supported") {
        distro_info.supported(date)
    } else if matches.get_flag("unsupported") {
        distro_info.unsupported(date)
    } else if matches.get_flag("devel") {
        match distro_info.distro() {
            Distro::Ubuntu => distro_info.ubuntu_devel(date),
            Distro::Debian => distro_info.debian_devel(date),
        }
    } else if get_maybe_missing_flag("testing") {
        // d-d-i --testing selection matches u-d-i --devel
        distro_info.ubuntu_devel(date)
    } else if get_maybe_missing_flag("latest") {
        let devel_result = distro_info.ubuntu_devel(date);
        if !devel_result.is_empty() {
            vec![*devel_result.last().unwrap()]
        } else {
            distro_info
                .latest(date)
                .map(|distro_release| vec![distro_release])
                .unwrap_or_else(Vec::new)
        }
    } else if get_maybe_missing_flag("lts") {
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
    } else if matches.get_flag("stable") {
        distro_info
            .latest(date)
            .map(|distro_release| vec![distro_release])
            .unwrap_or_else(Vec::new)
    } else if matches.contains_id("series") {
        match matches.get_one::<String>("series") {
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
                candidates
            }
            None => bail!("--series requires an argument; please report a bug about this error"),
        }
    } else {
        panic!("clap prevent us from reaching here; report a bug if you see this")
    })
}

fn today() -> NaiveDate {
    let now = Utc::now();
    NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap()
}
