//! Parse Debian and Ubuntu distro-info-data files and provide them as easy-to-consume Rust data
//! structures.
//!
//! Use [``UbuntuDistroInfo``](struct.UbuntuDistroInfo.html) to access the Ubuntu data.  (The
//! Debian implementation has yet to happen.)
extern crate chrono;
extern crate csv;
#[macro_use]
extern crate anyhow;

mod distro_release;

use std::env;

use anyhow::Error;
use chrono::naive::NaiveDate;
use csv::ReaderBuilder;

pub use crate::distro_release::DistroRelease;

pub enum Distro {
    Debian,
    Ubuntu,
}

impl Distro {
    pub fn to_string(&self) -> &'static str {
        match self {
            Distro::Ubuntu => "Ubuntu",
            Distro::Debian => "Debian",
        }
    }
}

fn parse_date(field: String) -> Result<NaiveDate, Error> {
    Ok(NaiveDate::parse_from_str(field.as_str(), "%Y-%m-%d")?)
}

#[derive(PartialEq)]
pub enum Milestone {
    Eol,
    EolELTS,
    EolESM,
    EolLTS,
    EolServer,
}

pub trait DistroInfo: Sized {
    const DEFAULT_CSV_PATH: &'static str;
    fn distro(&self) -> &Distro;
    fn releases(&self) -> &Vec<DistroRelease>;
    fn from_vec(releases: Vec<DistroRelease>) -> Self;
    /// The full path to the CSV file to read from for this distro
    fn csv_path() -> String {
        env::var("DISTRO_INFO_CSV").unwrap_or(Self::DEFAULT_CSV_PATH.to_string())
    }
    /// Read records from the given CSV reader to create a Debian/UbuntuDistroInfo object
    ///
    /// (These records must be in the format used in debian.csv/ubuntu.csv as provided by the
    /// distro-info-data package in Debian/Ubuntu.)
    fn from_csv_reader<T: std::io::Read>(mut rdr: csv::Reader<T>) -> Result<Self, Error> {
        let columns = rdr.headers()?.clone();
        let parse_required_str = |field: Option<String>| -> Result<String, Error> {
            field.ok_or(format_err!("failed to read required option"))
        };
        let getfield = |r: &csv::StringRecord, n: &str| -> Option<String> {
            columns
                .iter()
                .position(|header| header == n)
                .and_then(|i| r.get(i))
                .map(|s| s.to_string())
        };
        let mut releases = vec![];
        for record in rdr.records() {
            let record = record?;
            releases.push(DistroRelease::new(
                parse_required_str(getfield(&record, "version"))?,
                parse_required_str(getfield(&record, "codename"))?,
                parse_required_str(getfield(&record, "series"))?,
                getfield(&record, "created").map(parse_date).transpose()?,
                getfield(&record, "release").map(parse_date).transpose()?,
                getfield(&record, "eol").map(parse_date).transpose()?,
                getfield(&record, "eol-lts").map(parse_date).transpose()?,
                getfield(&record, "eol-elts").map(parse_date).transpose()?,
                getfield(&record, "eol-esm").map(parse_date).transpose()?,
                getfield(&record, "eol-server")
                    .map(parse_date)
                    .transpose()?,
            ))
        }
        Ok(Self::from_vec(releases))
    }

    /// Open this distro's CSV file and parse the release data contained therein
    fn new() -> Result<Self, Error> {
        Self::from_csv_reader(
            ReaderBuilder::new()
                .flexible(true)
                .has_headers(true)
                .from_path(Self::csv_path())?,
        )
    }

    /// Returns a vector of `DistroRelease`s for releases that had been created at the given date
    fn all_at(&self, date: NaiveDate) -> Vec<&DistroRelease> {
        self.releases()
            .iter()
            .filter(|distro_release| match distro_release.created() {
                Some(created) => date >= *created,
                None => false,
            })
            .collect()
    }

    /// Returns a vector of `DistroRelease`s for releases that were released at the given date
    fn released(&self, date: NaiveDate) -> Vec<&DistroRelease> {
        self.releases()
            .iter()
            .filter(|distro_release| distro_release.released_at(date))
            .collect()
    }

    /// Returns a vector of `DistroRelease`s for releases that were released and supported at the
    /// given date, per the given Milestone
    fn supported(&self, date: NaiveDate, milestone: Milestone) -> Vec<&DistroRelease> {
        self.releases()
            .iter()
            .filter(|distro_release| distro_release.supported_at(date, &milestone))
            .collect()
    }

    /// Returns a vector of `DistroRelease`s for releases that were released but no longer
    /// supported at the given date, per the given Milestone
    fn unsupported(&self, date: NaiveDate, milestone: Milestone) -> Vec<&DistroRelease> {
        self.released(date)
            .into_iter()
            .filter(|distro_release| !distro_release.supported_at(date, &milestone))
            .collect()
    }

    /// Returns a vector of `DistroRelease`s for releases that were released and supported at the
    /// given date, using Ubuntu's rules
    fn ubuntu_supported(&self, date: NaiveDate) -> Vec<&DistroRelease> {
        self.releases()
            .iter()
            .filter(|distro_release| distro_release.ubuntu_supported_at(date))
            .collect()
    }

    /// Returns a vector of `DistroRelease`s for releases that were released but no longer
    /// supported at the given date, using Ubuntu's rules
    fn ubuntu_unsupported(&self, date: NaiveDate) -> Vec<&DistroRelease> {
        self.released(date)
            .into_iter()
            .filter(|distro_release| !distro_release.ubuntu_supported_at(date))
            .collect()
    }

    /// Returns a vector of `DistroRelease`s for releases that were in development at the given
    /// date
    fn ubuntu_devel(&self, date: NaiveDate) -> Vec<&DistroRelease> {
        self.all_at(date)
            .into_iter()
            .filter(|distro_release| match distro_release.release() {
                Some(release) => date < *release,
                None => false,
            })
            .collect()
    }

    /// Returns a vector of `DistroRelease`s for releases that were in development at the given
    /// date
    fn debian_devel(&self, date: NaiveDate) -> Vec<&DistroRelease> {
        self.all_at(date)
            .into_iter()
            .filter(|distro_release| match distro_release.release() {
                Some(release) => date < *release,
                None => true,
            })
            .filter(|distro_release| distro_release.version().is_none())
            .collect::<Vec<_>>()
            .first()
            .copied()
            .map(|dr| vec![dr])
            .unwrap_or_else(std::vec::Vec::new)
    }

    /// Returns a `DistroRelease` for the latest supported, non-EOL release at the given date
    fn latest(&self, date: NaiveDate) -> Option<&DistroRelease> {
        self.ubuntu_supported(date)
            .into_iter()
            .filter(|distro_release| distro_release.released_at(date))
            .collect::<Vec<_>>()
            .last()
            .copied()
    }

    /// Returns a `DistroRelease` for the stable release prior to the current one (if one exists)
    fn oldstable(&self, date: NaiveDate) -> Option<&DistroRelease> {
        let candidates = self
            .released(date)
            .into_iter()
            .filter(|distro_release| distro_release.released_at(date))
            .collect::<Vec<_>>();
        let candidate_idx = candidates.len().checked_sub(2);
        candidate_idx.and_then(|idx| candidates.get(idx).copied())
    }

    fn iter(&self) -> ::std::slice::Iter<DistroRelease> {
        self.releases().iter()
    }
}

pub struct UbuntuDistroInfo {
    releases: Vec<DistroRelease>,
}

impl DistroInfo for UbuntuDistroInfo {
    const DEFAULT_CSV_PATH: &'static str = "/usr/share/distro-info/ubuntu.csv";
    fn distro(&self) -> &Distro {
        &Distro::Ubuntu
    }
    fn releases(&self) -> &Vec<DistroRelease> {
        &self.releases
    }
    /// Initialise an UbuntuDistroInfo struct from a vector of DistroReleases
    fn from_vec(releases: Vec<DistroRelease>) -> Self {
        Self { releases }
    }
}

impl IntoIterator for UbuntuDistroInfo {
    type Item = DistroRelease;
    type IntoIter = ::std::vec::IntoIter<DistroRelease>;

    fn into_iter(self) -> Self::IntoIter {
        self.releases.into_iter()
    }
}

pub struct DebianDistroInfo {
    releases: Vec<DistroRelease>,
}

impl DebianDistroInfo {
    pub fn stable(&self, date: NaiveDate) -> Option<&DistroRelease> {
        self.released(date).into_iter().rev().next()
    }

    pub fn oldstable(&self, date: NaiveDate) -> Option<&DistroRelease> {
        self.released(date).into_iter().rev().nth(1)
    }

    pub fn testing(&self, date: NaiveDate) -> Option<&DistroRelease> {
        self.iter().find(|release| {
            if let Some(created) = release.created() {
                date > *created
                    && (release.release().is_none() || date < release.release().unwrap())
                    && release.series() != "sid"
                    && release.series() != "experimental"
            } else {
                false
            }
        })
    }

    pub fn unstable(&self) -> &DistroRelease {
        self.releases()
            .iter()
            .find(|release| release.series() == "sid")
            .unwrap()
    }

    pub fn experimental(&self) -> &DistroRelease {
        self.releases()
            .iter()
            .find(|release| release.series() == "experimental")
            .unwrap()
    }
}

impl DistroInfo for DebianDistroInfo {
    const DEFAULT_CSV_PATH: &'static str = "/usr/share/distro-info/debian.csv";
    fn distro(&self) -> &Distro {
        &Distro::Debian
    }
    fn releases(&self) -> &Vec<DistroRelease> {
        &self.releases
    }
    /// Initialise an DebianDistroInfo struct from a vector of DistroReleases
    fn from_vec(releases: Vec<DistroRelease>) -> Self {
        Self { releases }
    }
}

impl IntoIterator for DebianDistroInfo {
    type Item = DistroRelease;
    type IntoIter = ::std::vec::IntoIter<DistroRelease>;

    fn into_iter(self) -> Self::IntoIter {
        self.releases.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use chrono::naive::NaiveDate;
    use {super::DebianDistroInfo, super::DistroInfo, super::UbuntuDistroInfo};

    pub fn naive_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn debian_distro_info_new() {
        DebianDistroInfo::new().unwrap();
    }

    #[test]
    fn ubuntu_distro_info_new() {
        UbuntuDistroInfo::new().unwrap();
    }

    #[test]
    fn debian_distro_info_item() {
        let distro_release = DebianDistroInfo::new().unwrap().into_iter().next().unwrap();
        assert_eq!(&Some("1.1".to_string()), distro_release.version());
        assert_eq!("Buzz", distro_release.codename());
        assert_eq!("buzz", distro_release.series());
        assert_eq!(&Some(naive_date(1993, 8, 16)), distro_release.created());
        assert_eq!(&Some(naive_date(1996, 6, 17)), distro_release.release());
        assert_eq!(&Some(naive_date(1997, 6, 5)), distro_release.eol());
        assert_eq!(&None, distro_release.eol_server());
    }

    #[test]
    fn ubuntu_distro_info_item() {
        let distro_release = UbuntuDistroInfo::new().unwrap().into_iter().next().unwrap();
        assert_eq!(&Some("4.10".to_string()), distro_release.version());
        assert_eq!("Warty Warthog", distro_release.codename());
        assert_eq!("warty", distro_release.series());
        assert_eq!(&Some(naive_date(2004, 3, 5)), distro_release.created());
        assert_eq!(&Some(naive_date(2004, 10, 20)), distro_release.release());
        assert_eq!(&Some(naive_date(2006, 4, 30)), distro_release.eol());
        assert_eq!(&None, distro_release.eol_server());
    }

    #[test]
    fn ubuntu_distro_info_eol_server() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        for distro_release in ubuntu_distro_info {
            match distro_release.series().as_ref() {
                "breezy" => assert_eq!(&None, distro_release.eol_server()),
                "dapper" => {
                    assert_eq!(&Some(naive_date(2011, 6, 1)), distro_release.eol_server());
                    break;
                }
                _ => {}
            }
        }
    }
    #[test]
    fn ubuntu_distro_info_released() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        // Use dapper's release date to confirm we don't have a boundary issue
        let date = naive_date(2006, 6, 1);
        let released_series: Vec<_> = ubuntu_distro_info
            .released(date)
            .iter()
            .map(|distro_release| distro_release.series())
            .collect();
        assert_eq!(vec!["warty", "hoary", "breezy", "dapper"], released_series);
    }

    #[test]
    fn ubuntu_distro_info_supported() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        // Use bionic's release date to confirm we don't have a boundary issue
        let date = naive_date(2018, 4, 26);
        let supported_series: Vec<_> = ubuntu_distro_info
            .ubuntu_supported(date)
            .iter()
            .map(|distro_release| distro_release.series())
            .collect();
        assert_eq!(
            vec!["trusty", "xenial", "artful", "bionic", "cosmic"],
            supported_series
        );
    }

    #[test]
    fn ubuntu_distro_info_unsupported() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        // Use bionic's release date to confirm we don't have a boundary issue
        let date = naive_date(2006, 11, 1);
        let unsupported_series: Vec<_> = ubuntu_distro_info
            .ubuntu_unsupported(date)
            .iter()
            .map(|distro_release| distro_release.series())
            .collect();
        assert_eq!(vec!["warty", "hoary"], unsupported_series);
    }

    #[test]
    fn ubuntu_distro_info_supported_on_eol_day() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        // Use artful's EOL date to confirm we don't have a boundary issue
        let date = naive_date(2018, 7, 19);
        let supported_series: Vec<_> = ubuntu_distro_info
            .ubuntu_supported(date)
            .iter()
            .map(|distro_release| distro_release.series())
            .collect();
        assert_eq!(
            vec!["trusty", "xenial", "artful", "bionic", "cosmic"],
            supported_series
        );
    }

    #[test]
    fn ubuntu_distro_info_supported_with_server_eol() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        let date = naive_date(2011, 5, 14);
        let supported_series: Vec<_> = ubuntu_distro_info
            .ubuntu_supported(date)
            .iter()
            .map(|distro_release| distro_release.series())
            .collect();
        assert!(supported_series.contains(&&"dapper".to_string()));
    }

    #[test]
    fn ubuntu_distro_info_devel() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        let date = naive_date(2018, 4, 26);
        let devel_series: Vec<_> = ubuntu_distro_info
            .ubuntu_devel(date)
            .iter()
            .map(|distro_release| distro_release.series())
            .collect();
        assert_eq!(vec!["cosmic"], devel_series);
    }

    #[test]
    fn ubuntu_distro_info_all_at() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        let date = naive_date(2005, 4, 8);
        let all_series: Vec<_> = ubuntu_distro_info
            .all_at(date)
            .iter()
            .map(|distro_release| distro_release.series())
            .collect();
        assert_eq!(vec!["warty", "hoary", "breezy"], all_series);
    }

    #[test]
    fn ubuntu_distro_info_latest() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        let date = naive_date(2005, 4, 8);
        let latest_series = ubuntu_distro_info.latest(date).unwrap().series();
        assert_eq!("hoary", latest_series);
    }

    #[test]
    fn ubuntu_distro_info_iter() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        let iter_suites: Vec<String> = ubuntu_distro_info
            .iter()
            .map(|distro_release| distro_release.series().clone())
            .collect();
        let mut for_loop_suites = vec![];
        for distro_release in ubuntu_distro_info {
            for_loop_suites.push(distro_release.series().clone());
        }
        assert_eq!(for_loop_suites, iter_suites);
    }

    #[test]
    fn ubuntu_distro_info_iters_are_separate() {
        let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
        let mut iter1 = ubuntu_distro_info.iter();
        let mut iter2 = ubuntu_distro_info.iter();
        assert_eq!(
            iter1.next().unwrap().series(),
            iter2.next().unwrap().series()
        );
    }

    #[test]
    fn debian_stable() {
        let debian_distro_info = DebianDistroInfo::new().unwrap();
        let stable = debian_distro_info.stable(naive_date(2018, 4, 26)).unwrap();
        assert_eq!(stable.series(), "stretch");
    }

    #[test]
    fn debian_oldstable() {
        let debian_distro_info = DebianDistroInfo::new().unwrap();
        let oldstable = debian_distro_info
            .oldstable(naive_date(2018, 4, 26))
            .unwrap();
        assert_eq!(oldstable.series(), "jessie");
    }

    #[test]
    fn debian_testing() {
        let debian_distro_info = DebianDistroInfo::new().unwrap();
        let testing = debian_distro_info.testing(naive_date(2021, 7, 26)).unwrap();
        assert_eq!(testing.series(), "bullseye");
    }

    #[test]
    fn debian_unstable() {
        let debian_distro_info = DebianDistroInfo::new().unwrap();
        let unstable = debian_distro_info.unstable();
        assert_eq!(unstable.series(), "sid");
    }

    #[test]
    fn debian_experimental() {
        let debian_distro_info = DebianDistroInfo::new().unwrap();
        let experimental = debian_distro_info.experimental();
        assert_eq!(experimental.series(), "experimental");
    }
}
