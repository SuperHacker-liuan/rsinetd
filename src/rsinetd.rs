use crate::rule::Rule;
use anyhow::Result;
use async_signals::Signals;
use async_std::io;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::net::ToSocketAddrs;
use async_std::stream::StreamExt;
use async_std::sync::Condvar;
use async_std::sync::Mutex;
use async_std::task;
use futures::future::select_ok;
use futures::FutureExt;
use log::error;
use log::warn;
use std::net::SocketAddr;
use std::sync::Arc;

type ReloadLock = Arc<(Mutex<Option<Vec<Rule>>>, Condvar)>;

pub struct RsInetd {
    lock: ReloadLock,
}

impl RsInetd {
    pub fn new() -> RsInetd {
        let lock = Arc::new((Mutex::new(None), Condvar::new()));
        RsInetd { lock }
    }

    pub async fn run(self, mut rules: Vec<Rule>) {
        task::spawn(sig_handler(self.lock.clone()));
        loop {
            let mut handlers = vec![];
            for rule in rules.iter() {
                let handler = task::spawn(listen(rule.clone()));
                handlers.push(handler);
            }
            let (lock, cond) = &*self.lock;
            let mut lock = lock.lock().await;
            *lock = None;
            let r = cond.wait_until(lock, |r| r.is_some()).await;

            for handler in handlers {
                task::JoinHandle::cancel(handler).await;
            }
            match &*r {
                Some(r) => rules = r.clone(),
                None => continue,
            }
        }
    }
}

async fn sig_handler(lock: ReloadLock) {
    let mut signals = match Signals::new(vec![libc::SIGHUP]) {
        Ok(s) => s,
        Err(e) => {
            error!(target: "rsinetd", "{}", e);
            return;
        }
    };

    loop {
        if let Some(libc::SIGHUP) = signals.next().await {
            let rules = match Rule::parse() {
                Ok(rules) => rules,
                Err(e) => {
                    warn!(target: "rsinetd", "Unable to reload configuration: {}", e);
                    continue;
                }
            };
            reload(&lock, rules).await;
        }
    }
}

async fn reload(lock: &ReloadLock, rules: Vec<Rule>) {
    let (lock, cond) = &**lock;
    *lock.lock().await = Some(rules);
    cond.notify_one();
}

async fn listen(rule: Rule) {
    if let Err(e) = listen_impl(rule).await {
        warn!(target: "rsinetd", "Failed to listen: {}", e);
    }
}

async fn listen_impl(rule: Rule) -> Result<()> {
    let listen = SocketAddr::new(rule.listen, rule.lport);
    let listen = TcpListener::bind(listen).await?;
    let mut listen = listen.incoming();
    while let Some(ls) = listen.next().await {
        let ls = match ls {
            Ok(stream) => stream,
            Err(_) => continue,
        };
        task::spawn(tcp_stream(ls, rule.target.clone(), rule.tport));
    }
    Ok(())
}

async fn tcp_stream(ls: TcpStream, target: String, tport: u16) {
    let _ = tcp_stream_impl(ls, target, tport).await;
}

async fn tcp_stream_impl(ls: TcpStream, target: String, tport: u16) -> Result<()> {
    let targets = format!("{}:{}", target, tport)
        .to_socket_addrs()
        .await?
        .map(|target| Box::pin(TcpStream::connect(target)));
    let (ts, _) = select_ok(targets).await?;

    // Sync
    let (lr, lw) = &mut (&ls, &ls);
    let (tr, tw) = &mut (&ts, &ts);
    futures::select! {
        _ = io::copy(lr, tw).fuse() => {},
        _ = io::copy(tr, lw).fuse() => {},
    }
    Ok(())
}
