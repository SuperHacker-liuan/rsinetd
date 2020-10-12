use self::config::CONFIG;
use self::rsinetd::RsInetd;
use self::rsinetd::Rule;
use anyhow::anyhow;
use anyhow::Result;
use async_std::task;
use daemonize::Daemonize;
use std::fs::File;
use std::io::Read;

mod config;
mod log;
mod rsinetd;

fn main() -> Result<()> {
    log::init_logger();
    let rules = parse_rule()?;
    daemonize();
    let rsinetd = RsInetd::new();
    task::block_on(rsinetd.run(rules));
    Ok(())
}

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

fn parse_rule() -> Result<Vec<Rule>> {
    let mut conf = File::open(&CONFIG.conf_file)?;
    let mut content = String::new();
    conf.read_to_string(&mut content)?;
    let conf: Vec<String> = content
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.starts_with("#") && s.len() > 0)
        .map(|s| s.into())
        .collect();
    let mut rules = vec![];
    for line in conf {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() != 4 {
            return Err(anyhow!("Syntax error in config file: {}", line));
        }
        let rule = Rule {
            listen: cols[0].parse()?,
            lport: cols[1].parse()?,
            target: cols[2].into(),
            tport: cols[3].parse()?,
        };
        rules.push(rule);
    }
    Ok(rules)
}
