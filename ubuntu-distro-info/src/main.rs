extern crate distro_info;

use distro_info::UbuntuDistroInfo;

fn main() {
    let ubuntu_distro_info = UbuntuDistroInfo::new().unwrap();
    for distro_release in ubuntu_distro_info {
        println!("{}", distro_release.series);
    }
}
