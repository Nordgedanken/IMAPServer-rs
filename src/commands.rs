use std;
use futures;

pub fn capability<'a>(mut conns: std::cell::RefMut<'a, std::collections::HashMap<std::net::SocketAddr, futures::sync::mpsc::UnboundedSender<std::string::String>>>, msg: std::string::String, addr: &'a std::net::SocketAddr) {
    // For each open connection except the sender, send the
    // string via the channel.
    let iter = conns
        .iter_mut()
        .map(|(y, v)| (y, v));

    let mut identifier_iter = msg.split_whitespace();
    let identifier = identifier_iter.nth(0).unwrap();
    for (y, tx) in iter {
        if y == addr {
            tx.send(format!("{}", "* CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN  LOGINDISABLED\r\n")).unwrap();
            tx.send(format!("{}{}", identifier, " OK CAPABILITY completed")).unwrap();
        } else {
            tx.send(format!("{}", "* CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN  LOGINDISABLED\r\n")).unwrap();
        }
    }
}

pub fn logout<'a>(mut conns: std::cell::RefMut<'a, std::collections::HashMap<std::net::SocketAddr, futures::sync::mpsc::UnboundedSender<std::string::String>>>, msg: std::string::String, addr: &'a std::net::SocketAddr) {
    // For each open connection except the sender, send the
    // string via the channel.
    let iter = conns
        .iter_mut()
        .map(|(y, v)| (y, v));

    let mut identifier_iter = msg.split_whitespace();
    let identifier = identifier_iter.nth(0).unwrap();
    for (y, tx) in iter {
        if y == addr {
            tx.send(format!("{}", "* BYE IMAP4rev1 Server logging out\r\n")).unwrap();
            tx.send(format!("{}{}", identifier, " OK LOGOUT completed")).unwrap();
        } else {
            tx.send(format!("{}", "* BYE IMAP4rev1 Server logging out\r\n")).unwrap();
        }
    }
}

pub fn login<'a>(mut conns: std::cell::RefMut<'a, std::collections::HashMap<std::net::SocketAddr, futures::sync::mpsc::UnboundedSender<std::string::String>>>, msg: std::string::String, addr: &'a std::net::SocketAddr) {
    unimplemented!();
}
