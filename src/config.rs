use clap::App;
use clap::Arg;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub struct Config {
    pub conf_file: PathBuf,
    pub daemon: bool,
}

pub static CONFIG: Lazy<Config> = Lazy::new(parse_config);

fn command_config() -> App<'static, 'static> {
    App::new("RsInetd: A replacement of rinetd")
        .name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("conf-file")
                .short("c")
                .long("conf-file")
                .value_name("FILE")
                .help("read configuration from FILE")
                .takes_value(true)
                .multiple(false)
                .required(false),
        )
        .arg(
            Arg::with_name("foreground")
                .short("f")
                .long("foreground")
                .help("do not run in the background")
                .takes_value(false)
                .multiple(false)
                .required(false),
        )
}

fn parse_config() -> Config {
    let matches = command_config().get_matches();
    let conf_file = matches
        .value_of("conf-file")
        .unwrap_or("/etc/rsinetd.conf")
        .into();
    let daemon = !matches.is_present("foreground");
    Config { conf_file, daemon }
}
