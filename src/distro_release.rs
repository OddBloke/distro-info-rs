use chrono::naive::NaiveDate;

use crate::Milestone;

#[derive(Default, Clone, Debug)]
pub struct DistroRelease {
    version: Option<String>,
    codename: String,
    series: String,
    created: Option<NaiveDate>,
    release: Option<NaiveDate>,
    eol: Option<NaiveDate>,
    eol_lts: Option<NaiveDate>,
    eol_elts: Option<NaiveDate>,
    eol_esm: Option<NaiveDate>,
    eol_server: Option<NaiveDate>,
}

impl DistroRelease {
    pub fn new(
        version: String,
        codename: String,
        series: String,
        created: Option<NaiveDate>,
        release: Option<NaiveDate>,
        eol: Option<NaiveDate>,
        eol_lts: Option<NaiveDate>,
        eol_elts: Option<NaiveDate>,
        eol_esm: Option<NaiveDate>,
        eol_server: Option<NaiveDate>,
    ) -> Self {
        Self {
            version: if version.is_empty() {
                None
            } else {
                Some(version)
            },
            codename,
            series,
            created,
            release,
            eol,
            eol_lts,
            eol_elts,
            eol_esm,
            eol_server,
        }
    }

    // Getters
    pub fn version(&self) -> &Option<String> {
        &self.version
    }
    pub fn codename(&self) -> &String {
        &self.codename
    }
    pub fn series(&self) -> &String {
        &self.series
    }
    pub fn created(&self) -> &Option<NaiveDate> {
        &self.created
    }
    pub fn release(&self) -> &Option<NaiveDate> {
        &self.release
    }
    pub fn eol(&self) -> &Option<NaiveDate> {
        &self.eol
    }
    pub fn eol_server(&self) -> &Option<NaiveDate> {
        &self.eol_server
    }
    pub fn eol_esm(&self) -> &Option<NaiveDate> {
        &self.eol_esm
    }
    pub fn eol_elts(&self) -> &Option<NaiveDate> {
        &self.eol_elts
    }
    pub fn eol_lts(&self) -> &Option<NaiveDate> {
        &self.eol_lts
    }

    // Non-getters
    pub fn ubuntu_is_lts(&self) -> bool {
        self.version
            .as_ref()
            .map(|version| version.contains("LTS"))
            .unwrap_or(false)
    }

    pub fn created_at(&self, date: NaiveDate) -> bool {
        match self.created {
            Some(created) => date >= created,
            None => false,
        }
    }

    pub fn released_at(&self, date: NaiveDate) -> bool {
        match self.release {
            Some(release) => date >= release,
            None => false,
        }
    }

    pub fn milestone_date(&self, milestone: &Milestone) -> Option<NaiveDate> {
        *match milestone {
            Milestone::Eol => self.eol(),
            Milestone::EolELTS => self.eol_elts(),
            Milestone::EolESM => self.eol_esm(),
            Milestone::EolLTS => self.eol_lts(),
            Milestone::EolServer => self.eol_server(),
        }
    }

    pub fn supported_at(&self, date: NaiveDate, milestone: &Milestone) -> bool {
        self.created_at(date)
            && match self.milestone_date(milestone) {
                Some(eol) => date <= eol,
                // Missing eol means supported, otherwise unsupported
                None => milestone == &Milestone::Eol,
            }
    }

    pub fn ubuntu_supported_at(&self, date: NaiveDate) -> bool {
        self.created_at(date)
            && match self.eol {
                Some(eol) => match self.eol_server {
                    Some(eol_server) => date <= ::std::cmp::max(eol, eol_server),
                    None => date <= eol,
                },
                None => true,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::DistroRelease;

    use crate::{tests::naive_date, Milestone};

    #[test]
    fn create_struct() {
        DistroRelease {
            version: Some("version".to_string()),
            codename: "codename".to_string(),
            series: "series".to_string(),
            created: Some(naive_date(2018, 6, 14)),
            release: Some(naive_date(2018, 6, 14)),
            eol: Some(naive_date(2018, 6, 14)),
            eol_server: Some(naive_date(2018, 6, 14)),
            ..Default::default()
        };
    }

    #[test]
    fn distro_release_new() {
        let get_date = |mut n| {
            let mut date = naive_date(2018, 6, 14);
            while n > 0 {
                date = date.succ_opt().unwrap();
                n -= 1;
            }
            date
        };
        let distro_release = DistroRelease::new(
            "version".to_string(),
            "codename".to_string(),
            "series".to_string(),
            Some(get_date(0)),
            Some(get_date(1)),
            Some(get_date(2)),
            Some(get_date(3)),
            Some(get_date(4)),
            Some(get_date(5)),
            Some(get_date(6)),
        );
        assert_eq!(Some("version".to_string()), distro_release.version);
        assert_eq!("codename", distro_release.codename);
        assert_eq!("series", distro_release.series);
        assert_eq!(Some(get_date(0)), distro_release.created);
        assert_eq!(Some(get_date(1)), distro_release.release);
        assert_eq!(Some(get_date(2)), distro_release.eol);
        assert_eq!(Some(get_date(3)), distro_release.eol_lts);
        assert_eq!(Some(get_date(4)), distro_release.eol_elts);
        assert_eq!(Some(get_date(5)), distro_release.eol_esm);
        assert_eq!(Some(get_date(6)), distro_release.eol_server);

        assert_eq!(&Some("version".to_string()), distro_release.version());
        assert_eq!(&"codename", distro_release.codename());
        assert_eq!(&"series", distro_release.series());
        assert_eq!(&Some(get_date(0)), distro_release.created());
        assert_eq!(&Some(get_date(1)), distro_release.release());
        assert_eq!(&Some(get_date(2)), distro_release.eol());
        assert_eq!(&Some(get_date(3)), distro_release.eol_lts());
        assert_eq!(&Some(get_date(4)), distro_release.eol_elts());
        assert_eq!(&Some(get_date(5)), distro_release.eol_esm());
        assert_eq!(&Some(get_date(6)), distro_release.eol_server());
    }

    #[test]
    fn distro_release_ubuntu_is_lts() {
        let distro_release = DistroRelease::new(
            "98.04 LTS".to_string(),
            "codename".to_string(),
            "series".to_string(),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
        );
        assert!(distro_release.ubuntu_is_lts());

        let distro_release = DistroRelease::new(
            "98.04".to_string(),
            "codename".to_string(),
            "series".to_string(),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
        );
        assert!(!distro_release.ubuntu_is_lts());
    }

    #[test]
    fn distro_release_released_at() {
        let distro_release = DistroRelease::new(
            "98.04 LTS".to_string(),
            "codename".to_string(),
            "series".to_string(),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 16)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
        );
        // not released before release day
        assert!(!distro_release.released_at(naive_date(2018, 6, 13)));
        // released on release day
        assert!(distro_release.released_at(naive_date(2018, 6, 14)));
        // still released after EOL
        assert!(distro_release.released_at(naive_date(2018, 6, 17)));
    }

    #[test]
    fn distro_release_ubuntu_supported_at() {
        let distro_release = DistroRelease::new(
            "98.04 LTS".to_string(),
            "codename".to_string(),
            "series".to_string(),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 16)),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 14)),
            None,
            None,
        );
        // not supported before release day
        assert!(!distro_release.ubuntu_supported_at(naive_date(2018, 6, 13)));
        // supported on release day
        assert!(distro_release.ubuntu_supported_at(naive_date(2018, 6, 14)));
        // not supported after EOL
        assert!(!distro_release.ubuntu_supported_at(naive_date(2018, 6, 17)));
    }

    #[test]
    fn distro_release_milestone_date() {
        let distro_release = DistroRelease::new(
            "98.04 LTS".to_string(),
            "codename".to_string(),
            "series".to_string(),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 15)),
            Some(naive_date(2018, 6, 16)),
            Some(naive_date(2018, 6, 17)),
            Some(naive_date(2018, 6, 18)),
            Some(naive_date(2018, 6, 19)),
            Some(naive_date(2018, 6, 20)),
        );
        let assert = |milestone: Milestone, day: u32| {
            assert_eq!(
                distro_release.milestone_date(&milestone),
                Some(naive_date(2018, 6, day))
            )
        };
        assert(Milestone::Eol, 16);
        assert(Milestone::EolLTS, 17);
        assert(Milestone::EolELTS, 18);
        assert(Milestone::EolESM, 19);
        assert(Milestone::EolServer, 20);
    }

    #[test]
    fn distro_release_milestone_date_none() {
        let distro_release = DistroRelease::new(
            "98.04 LTS".to_string(),
            "codename".to_string(),
            "series".to_string(),
            Some(naive_date(2018, 6, 14)),
            Some(naive_date(2018, 6, 15)),
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(distro_release.milestone_date(&Milestone::Eol), None);
        assert_eq!(distro_release.milestone_date(&Milestone::EolLTS), None);
        assert_eq!(distro_release.milestone_date(&Milestone::EolELTS), None);
        assert_eq!(distro_release.milestone_date(&Milestone::EolESM), None);
        assert_eq!(distro_release.milestone_date(&Milestone::EolServer), None);
    }
}
