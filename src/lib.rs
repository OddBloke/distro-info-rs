extern crate chrono;

use chrono::naive::NaiveDate;

pub struct DistroRelease {
    pub version: String,
    pub codename: String,
    pub series: String,
    pub created: Option<NaiveDate>,
    pub release: Option<NaiveDate>,
    pub eol: Option<NaiveDate>,
    pub eol_server: Option<NaiveDate>,
}

#[cfg(test)]
mod tests {
    use chrono::naive::NaiveDate;
    use DistroRelease;

    #[test]
    fn create_struct() {
        DistroRelease {
            version: "version".to_string(),
            codename: "codename".to_string(),
            series: "series".to_string(),
            created: Some(NaiveDate::from_ymd(2018, 6, 14)),
            release: Some(NaiveDate::from_ymd(2018, 6, 14)),
            eol: Some(NaiveDate::from_ymd(2018, 6, 14)),
            eol_server: Some(NaiveDate::from_ymd(2018, 6, 14)),
        };
        ()
    }
}
