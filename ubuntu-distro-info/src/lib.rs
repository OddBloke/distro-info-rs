use clap::{App, Arg, ArgGroup};

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
