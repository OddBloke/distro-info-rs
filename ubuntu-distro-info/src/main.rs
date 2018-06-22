extern crate chrono;
extern crate clap;
extern crate distro_info;

use chrono::Datelike;
use chrono::Utc;
use chrono::naive::NaiveDate;
use clap::{Arg, ArgGroup, App};
use distro_info::UbuntuDistroInfo;

fn all() {
    let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
    for distro_release in ubuntu_distro_info {
        println!("{}", distro_release.series);
    }
}

fn supported() {
    let now = Utc::now();
    let today = NaiveDate::from_ymd(now.year(), now.month(), now.day());
    let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
    for distro_release in ubuntu_distro_info.supported(today) {
        println!("{}", distro_release.series);
    }
}

fn main() {
    let matches = App::new("ubuntu-distro-info")
        .version("0.1.2")
        .author("Daniel Watkins <daniel@daniel-watkins.co.uk>")
        .arg(Arg::with_name("all").short("a").long("all"))
        .arg(Arg::with_name("supported").long("supported"))
        .group(ArgGroup::with_name("selector").args(&["all", "supported"]).required(true))
        .get_matches();
    if matches.is_present("all") {
        all();
    } else if matches.is_present("supported") {
        supported();
    }
}
