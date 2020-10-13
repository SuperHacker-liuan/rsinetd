use anyhow::anyhow;
use anyhow::Result;
use clap::App;
use clap::Arg;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub struct Config {
    pub conf_file: Option<PathBuf>,
    pub daemon: bool,
}

pub static CONFIG: Lazy<Config> = Lazy::new(parse_config);

fn command_config() -> App<'static, 'static> {
    let mut app = App::new("RsInetd: A replacement of rinetd")
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
        );
    if cfg!(unix) {
        app = app.arg(
            Arg::with_name("foreground")
                .short("f")
                .long("foreground")
                .help("do not run in the background")
                .takes_value(false)
                .multiple(false)
                .required(false),
        );
    }
    app
}

fn parse_config() -> Config {
    let matches = command_config().get_matches();
    let conf_file = matches.value_of("conf-file").map(|r| r.into());
    let daemon = !matches.is_present("foreground");
    Config { conf_file, daemon }
}

#[cfg(unix)]
const DEFAULT_CONF: &[&str] = &[
    "/etc/rsinetd.conf",
    "rsinetd.conf",
    "/etc/rinetd.conf",
    "rinetd.conf",
];

#[cfg(not(unix))]
const DEFAULT_CONF: &[&str] = &["rsinetd.conf", "rinetd.conf"];
static USE_CONF: OnceCell<PathBuf> = OnceCell::new();

pub fn open_conf_file() -> Result<String> {
    if let Some(file) = USE_CONF.get() {
        return read_file(file);
    }
    if let Some(file) = &CONFIG.conf_file {
        return read_file(&file);
    }
    for file in DEFAULT_CONF {
        if let Ok(s) = read_file(file.as_ref()) {
            return Ok(s);
        }
    }
    Err(anyhow!("Default conf file not found: {:?}.", DEFAULT_CONF))
}

fn read_file(file: &Path) -> Result<String> {
    let mut conf = File::open(file)?;
    let mut content = String::new();
    conf.read_to_string(&mut content)?;

    let file = file.canonicalize()?;
    USE_CONF.get_or_init(|| file);
    Ok(content)
}
