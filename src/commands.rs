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
            tx.send(format!("{}", "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED\r\n")).unwrap();
            tx.send(format!("{}{}", identifier, " OK CAPABILITY completed")).unwrap();

            //Print to view for debug
            println!("{}", "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED\r\n");
            println!("{}{}", identifier, " OK CAPABILITY completed");
        } else {
            tx.send(format!("{}", "* CAPABILITY IMAP4rev1 UTF8=ACCEPT AUTH=PLAIN LOGINDISABLED\r\n")).unwrap();

            //Print to view for debug
            println!("{}", "* CAPABILITY IMAP4rev1 UTF8=ACCEPT AUTH=PLAIN LOGINDISABLED\r\n");
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

            //Print to view for debug
            println!("{}", "* BYE IMAP4rev1 Server logging out\r\n");
            println!("{}{}", identifier, " OK LOGOUT completed");
        } else {
            tx.send(format!("{}", "* BYE IMAP4rev1 Server logging out\r\n")).unwrap();

            //Print to view for debug
            println!("{}", "* BYE IMAP4rev1 Server logging out\r\n");
        }
    }
}

pub fn authenticate <'a>(mut conns: std::cell::RefMut<'a, std::collections::HashMap<std::net::SocketAddr, futures::sync::mpsc::UnboundedSender<std::string::String>>>, msg: std::string::String, addr: &'a std::net::SocketAddr) {
    // For each open connection except the sender, send the
    // string via the channel.
    let iter = conns
        .iter_mut()
        .map(|(y, v)| (y, v));

    let mut identifier_iter = msg.split_whitespace();
    let identifier = identifier_iter.nth(0).unwrap();
    for (y, tx) in iter {
        if y == addr {
            tx.send(format!("{} {}", identifier, "OK PLAIN authentication successful")).unwrap();

            //Print to view for debug
            println!("{} {}", identifier, "OK LOGIN completed");
        }
    }
}

pub fn noop<'a>(mut conns: std::cell::RefMut<'a, std::collections::HashMap<std::net::SocketAddr, futures::sync::mpsc::UnboundedSender<std::string::String>>>, msg: std::string::String, addr: &'a std::net::SocketAddr) {
    // For each open connection except the sender, send the
    // string via the channel.
    let iter = conns
        .iter_mut()
        .map(|(y, v)| (y, v));

    let mut identifier_iter = msg.split_whitespace();
    let identifier = identifier_iter.nth(0).unwrap();
    for (y, tx) in iter {
        if y == addr {
            tx.send(format!("{} {}", identifier, "OK NOOP completed")).unwrap();

            //Print to view for debug
            println!("{} {}", identifier, "OK NOOP completed");
        }
    }
}

pub fn select<'a>(mut conns: std::cell::RefMut<'a, std::collections::HashMap<std::net::SocketAddr, futures::sync::mpsc::UnboundedSender<std::string::String>>>, msg: std::string::String, addr: &'a std::net::SocketAddr) {
    // For each open connection except the sender, send the
    // string via the channel.
    let iter = conns
        .iter_mut()
        .map(|(y, v)| (y, v));

    let mut identifier_iter = msg.split_whitespace();
    let identifier = identifier_iter.nth(0).unwrap();
    for (y, tx) in iter {
        if y == addr {
            tx.send(format!("{}", "* 1 EXISTS")).unwrap();
            tx.send(format!("{}", "* 1 RECENT")).unwrap();
            tx.send(format!("{}", "* OK [UNSEEN 1] Message 1 is first unseen")).unwrap();
            tx.send(format!("{}", "* OK [UIDVALIDITY 3857529045] UIDs valid")).unwrap();
            tx.send(format!("{}", "* OK [UIDNEXT 4392] Predicted next UID")).unwrap();
            tx.send(format!("{}", "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)")).unwrap();
            tx.send(format!("{}", "* OK [PERMANENTFLAGS (\\Deleted \\Seen \\*)] Limited")).unwrap();
            tx.send(format!("{} {}", identifier, "OK [READ-WRITE] SELECT completed")).unwrap();

            //Print to view for debug
            println!("{} {}", identifier, "OK [READ-WRITE] SELECT completed");
        }
    }
}