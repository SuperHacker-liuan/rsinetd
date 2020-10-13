use self::config::CONFIG;
use self::rsinetd::RsInetd;
use anyhow::Result;
use async_std::task;
use daemonize::Daemonize;

mod config;
mod log;
mod rsinetd;
mod rule;

fn main() -> Result<()> {
    // Init
    log::init_logger();
    let rules = rule::Rule::parse()?;

    daemonize(); // to store init value, daemon should start after Init
    let rsinetd = RsInetd::new();
    task::block_on(rsinetd.run(rules));
    Ok(())
}

#[cfg(unix)]
fn daemonize() {
    if !CONFIG.daemon {
        return;
    }
    Daemonize::new()
        .pid_file(format!("/tmp/rsinetd.pid"))
        .working_directory("/tmp")
        .umask(0o777)
        .start()
        .expect("Failed to start as daemon");
}

#[cfg(not(unix))]
fn daemonize() {}
