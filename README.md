# RsInetd

[![Crates.io](https://img.shields.io/crates/v/rsinetd)](https://crates.io/crates/rsinetd)
[![Build Status](https://travis-ci.org/SuperHacker-liuan/rsinetd.svg?branch=master)](https://travis-ci.org/SuperHacker-liuan/rsinetd)
[![GitHub license](https://img.shields.io/github/license/SuperHacker-liuan/rsinetd)](https://github.com/SuperHacker-liuan/rsinetd/blob/master/LICENSE)

A port proxy, replacement of rinetd. Because async-std use epoll rather than select, RsInetd may handle higher throughput than rinetd.

## Install

```bash
cargo install rsinetd
```

## How to use

Usage of RsInetd is similiar to rinetd. We'll try to open the default configuration
file in the following order.

### Default conf's open order on unix

1. `/etc/rsinetd.conf` 
2. `./rsinetd.conf`
3. `/etc/rinetd.conf`
4. `./rinetd.conf`

### Default conf's open order on non-unix

1. `./rsinetd.conf`
2. `./rinetd.conf`


## command line options

```bash
$ rsinetd -h

rsinetd 0.2.0

A port proxy, replacement of rinetd. Because async-std use epoll rather than select, RsInetd may handle higher
throughput than rinetd.

USAGE:
    rsinetd [FLAGS] [OPTIONS]

FLAGS:
    -f, --foreground    do not run in the background
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -c, --conf-file <FILE>    read configuration from FILE
```

### Example of `/etc/rsinetd.conf`

```
::          80      crates.io   80
0.0.0.0     443     crates.io   443
```

With this configuration file, rsinetd will listen on `[::]:80` and forward the
port access to `crates.io:80`, at the same time listenon `0.0.0.0:443` and forward the port access to `crates.io:443`
