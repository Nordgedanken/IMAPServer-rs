use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::UnboundedSender;

pub fn capability(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!(
                "{}",
                "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED\r\n"
            )).unwrap();
            tx.unbounded_send(format!("{}{}", identifier, " OK CAPABILITY completed\r\n"))
                .unwrap();

            //Print to view for debug
            debug!(
                "{}",
                "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED\r\n"
            );
            debug!("{}{}", identifier, " OK CAPABILITY completed");
        }
    }
}

pub fn list(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("{}", "* LIST () \"/\" INBOX\r\n")).unwrap();
            tx.unbounded_send(format!("{}{}", identifier, " OK LIST Completed\r\n"))
                .unwrap();

            //Print to view for debug
            debug!("{}", "* LIST () \"/\" \"INBOX\"\r\n");
            debug!("{}{}", identifier, " OK LIST Completed\r\n");
        }
    }
}

pub fn uid(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("{}", "* 1 FETCH (FLAGS (\\Seen) UID 1)\r\n"))
                .unwrap();
            tx.unbounded_send(format!("{}{}", identifier, " OK UID FETCH completed\r\n"))
                .unwrap();

            //Print to view for debug
            debug!("{}", "* 1 FETCH (FLAGS (\\Seen) UID 1)\r\n");
            debug!("{}{}", identifier, " OK UID FETCH completed\r\n");
        }
    }
}

pub fn logout(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("{}", "* BYE IMAP4rev1 Server logging out\r\n"))
                .unwrap();
            tx.unbounded_send(format!("{}{}", identifier, " OK LOGOUT completed"))
                .unwrap();

            //Print to view for debug
            debug!("{}", "* BYE IMAP4rev1 Server logging out\r\n");
            debug!("{}{}", identifier, " OK LOGOUT completed\r\n");
        }
    }
}

pub mod authenticate;

#[deprecated(since = "0.0.1", note = "please use `commands::authenticate::authenticate` instead")]
pub fn login(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("{} {}", identifier, "OK LOGIN completed\r\n"))
                .unwrap();

            //Print to view for debug
            debug!("{} {}", identifier, "OK LOGIN completed\r\n");
        }
    }
}

pub fn noop(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("{} {}", identifier, "OK NOOP completed\r\n"))
                .unwrap();

            //Print to view for debug
            debug!("{} {}", identifier, "OK NOOP completed");
        }
    }
}

pub fn select(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("{}", "* 1 EXISTS\r\n")).unwrap();
            tx.unbounded_send(format!("{}", "* 0 RECENT\r\n")).unwrap();
            tx.unbounded_send(format!(
                "{}",
                "* OK [UNSEEN 1] Message 1 is first unseen\r\n"
            )).unwrap();
            tx.unbounded_send(format!("{}", "* OK [UIDNEXT 1] Predicted next UID\r\n"))
                .unwrap();
            tx.unbounded_send(format!(
                "{}",
                "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)\r\n"
            )).unwrap();
            tx.unbounded_send(format!(
                "{}",
                "* OK [PERMANENTFLAGS (\\Deleted \\Seen \\*)] Limited\r\n"
            )).unwrap();
            tx.unbounded_send(format!(
                "{} {}",
                identifier,
                "OK [READ-WRITE] SELECT completed\r\n"
            )).unwrap();

            //Print to view for debug
            debug!("{} {}", identifier, "OK [READ-WRITE] SELECT completed");
        }
    }
}

pub fn check(
    conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr,
) {
    // For each open connection except the sender, send the
    // string via the channel.
    let mut conns = conns.lock().unwrap();
    let iter = (*conns).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("{} {}", identifier, "OK CHECK Completed\r\n"))
                .unwrap();

            //Print to view for debug
            debug!("{} {}", identifier, "OK CHECK Completed");
        }
    }
}
