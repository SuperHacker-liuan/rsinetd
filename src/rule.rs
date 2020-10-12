use anyhow::anyhow;
use anyhow::Result;
use std::net::IpAddr;

#[derive(Clone)]
pub struct Rule {
    pub listen: IpAddr,
    pub lport: u16,
    pub target: String,
    pub tport: u16,
}

impl Rule {
    pub fn parse() -> Result<Vec<Rule>> {
        parse_rule()
    }
}

fn parse_rule() -> Result<Vec<Rule>> {
    let conf: Vec<String> = crate::config::open_conf_file()?
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
