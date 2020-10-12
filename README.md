# RsInetd

A port proxy, replacement of rinetd. Because async-std use epoll rather than select, RsInetd may handle higher throughput than rinetd.

## Install

```bash
cargo install rsinetd
```

## How to use

Usage of RsInetd is similiar to rinetd. The default configuration file is
`/etc/rsinetd.conf`, current version we have only implemented socket to socket
proxy, dns resolve haven't been implemented yet. For detail command line options,
execute `rsinetd -h`.

```
rsinetd 0.1.0
劉安 <liuan@sgcc.com.cn>
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
::      9999    192.168.1.1     80
```

With this configuration file, rsinetd will listen on `[::]:9999` and forward the
port access to 192.168.1.1:80.
