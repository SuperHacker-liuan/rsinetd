# RsInetd

A replacement of rinetd. Because async-std use epoll rather than select, RsInetd may handle higher throughput than rinetd.