# distro-info-rs

A Rust library to parse Debian/Ubuntu distro-info-data, aiming to
replicate the functionality of the C implementation in the distro-info
package in Debian/Ubuntu.

## Installation

With a working installation of `cargo`, run `cargo install
distro-info-binaries`, then:

```
ubuntu-distro-info --help
```

or

```
debian-distro-info --help
```

## Changelog Generation

Note that the clog-cli at https://github.com/OddBloke/clog-cli should
be used for generating changelogs, pending acceptance of some changes
upstream.
