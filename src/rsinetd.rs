use anyhow::Result;
use async_std::io;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use async_std::sync::Condvar;
use async_std::sync::Mutex;
use async_std::task;
use futures::FutureExt;
use log::warn;
use std::net::IpAddr;
use std::net::SocketAddr;

type ReloadLock = (Mutex<Option<Vec<Rule>>>, Condvar);

pub struct RsInetd {
    lock: ReloadLock,
}

impl RsInetd {
    pub fn new() -> RsInetd {
        let lock = (Mutex::new(None), Condvar::new());
        RsInetd { lock }
    }

    pub async fn run(&self, mut rules: Vec<Rule>) {
        loop {
            let mut handlers = vec![];
            for rule in rules.iter() {
                let handler = task::spawn(listen(rule.clone()));
                handlers.push(handler);
            }
            let (lock, cond) = &self.lock;
            let r = cond.wait_until(lock.lock().await, |r| r.is_some()).await;

            for handler in handlers {
                task::JoinHandle::cancel(handler).await;
            }
            match &*r {
                Some(r) => rules = r.clone(),
                None => continue,
            }
        }
    }

    pub async fn _reload(&self, rules: Vec<Rule>) {
        let (lock, cond) = &self.lock;
        *lock.lock().await = Some(rules);
        cond.notify_one();
    }
}

#[derive(Clone)]
pub struct Rule {
    pub listen: IpAddr,
    pub lport: u16,
    pub target: String,
    pub tport: u16,
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
    let target = SocketAddr::new(target.parse()?, tport);
    let ts = TcpStream::connect(target).await?;

    // Sync
    let (lr, lw) = &mut (&ls, &ls);
    let (tr, tw) = &mut (&ts, &ts);
    futures::select! {
        _ = io::copy(lr, tw).fuse() => {},
        _ = io::copy(tr, lw).fuse() => {},
    }
    Ok(())
}
