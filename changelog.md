<a name="0.2.0"></a>
## 0.2.0 (2018-06-24)


#### Features

* **distro-info:**
  *  add UbuntuDistroInfo.unsupported ([f136a27b](https://github.com/OddBloke/distro-info-rs/commit/f136a27b91fcfbabb3e05431e01adaac9d130ae8))
  *  implement DistroRelease.{released_at,supported_at} ([f1d55202](https://github.com/OddBloke/distro-info-rs/commit/f1d552021222fd3a9939c5c2c93b42d9a8983d16))
* **ubuntu-distro-info:**
  *  initial release



<a name="0.1.3"></a>
## 0.1.3 (2018-06-23)



#### Bug Fixes

* **distro-info:**  Fix a whole host of boundary bugs ([93d47d5a](https://github.com/OddBloke/distro-info-rs/commit/93d47d5a2e12137c3df67c1b2dded54de580fddf))



<a name="0.1.2"></a>
## 0.1.2 (2018-06-23)


#### Bug Fixes

* **distro-info:**  Consider eol_server in UbuntuDistroInfo.supported() ([dda17940](https://github.com/OddBloke/distro-info-rs/commit/dda1794084bdaa7b2cdd651521a3b3f2c79f4ef8))

#### Features

* **distro-info:**
  *  Add UbuntuDistroInfo.released() ([03b2a729](https://github.com/OddBloke/distro-info-rs/commit/03b2a72998068ccf28641c4bc4dae939a2df0499))
  *  Add DistroRelease.is_lts() ([13a4473f](https://github.com/OddBloke/distro-info-rs/commit/13a4473fc5da9eec4cfe20effee02834a5c30415))
  *  Add UbuntuDistroInfo.latest ([641d70c1](https://github.com/OddBloke/distro-info-rs/commit/641d70c1ffe9d39b1ba75ec55d6bdf74d139436f))
  *  Add UbuntuDistroInfo.all_at ([88c14175](https://github.com/OddBloke/distro-info-rs/commit/88c14175a0958fcb84086d0ae57f991132f88e24))
  *  Add UbuntuDistroInfo.devel() to return the current devel releases ([1452c994](https://github.com/OddBloke/distro-info-rs/commit/1452c994f3302267106ccac1e1000122488e7259))



<a name="0.1.1"></a>
## 0.1.1 (2018-06-23)


#### Features

* **distro-info:**  Implement UbuntuDistroInfo.iter ([b22e27d8](https://github.com/OddBloke/distro-info-rs/commit/b22e27d86e5a3398569b8b46ed8e4de0dc969199))



<a name="0.1.0"></a>
## 0.1.0 (2018-06-23)


#### Features

* **distro-info:**
  *  Add UbuntuDistroInfo.supported ([a1e7165f](https://github.com/OddBloke/distro-info-rs/commit/a1e7165fed791b97ec1ab00342c6693ddad259c2))
  *  Implement IntoIterator for UbuntuDistroInfo ([5e835fdf](https://github.com/OddBloke/distro-info-rs/commit/5e835fdf766d838f4c2fc0dffcba4b069b22df17))
  *  Handle server-eol ([03ec0280](https://github.com/OddBloke/distro-info-rs/commit/03ec0280d80dfdaf57b1724a6164bf50582f4a52))
  *  Implement UbuntuDistroInfo::new() excluding server_eol ([83b0e121](https://github.com/OddBloke/distro-info-rs/commit/83b0e121bf429e66879ccf465ee725354321684b))
  *  Add DistroRelease::new() ([2601728a](https://github.com/OddBloke/distro-info-rs/commit/2601728ac5c97e4cf6643046fca23caaf0021e94))
  *  Add DistroRelease struct ([4837e3e3](https://github.com/OddBloke/distro-info-rs/commit/4837e3e36c9b0e9b77e3a97ed15f32b6d067686d))
  *  Initial commit ([21cbfd4e](https://github.com/OddBloke/distro-info-rs/commit/21cbfd4ef5d61a8ffb51f89f07875b2f84945ec7))

#### Bug Fixes

* **distro-info:**  Fix date parsing ([79dd6fe4](https://github.com/OddBloke/distro-info-rs/commit/79dd6fe47ec2882b6f268e72a65843530809732b))
