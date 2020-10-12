use std::net::IpAddr;

#[derive(Debug)]
pub struct Config {
    Vec<Rule>
}

#[derive(Debug)]
pub struct Rule {
    listen: IpAddr,
    lport: u16,
    target: String,
    tport: u16,
}
