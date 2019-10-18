use std::collections::HashMap;
use std::net::SocketAddr;
use std::result::Result::{Err, Ok};
use std::sync::{Arc, Mutex};

use base64::decode;
use futures::sync::mpsc::UnboundedSender;

use crate::database::Users;
use crate::helper::connect_to_db;

pub fn authenticate(
    mut conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr
){
    // For each open connection except the sender, send the
    // string via the channel.
    let iter = (*(conns.lock().unwrap())).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            tx.unbounded_send(format!("+\r\n")).unwrap();
            tx.unbounded_send(format!("{} {}", identifier, "+\r\n")).unwrap();

            //Print to view for debug
            debug!("{} {}", identifier, "+\r\n");
        }
    }
}

pub fn parse_login_data(
    mut conns: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<String>>>>,
    args: Vec<&str>,
    addr: &std::net::SocketAddr
){
    let bytes = decode(args[0]).unwrap();
    let string = match String::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => format!("Invalid UTF-8 sequence: {}", e),
    };
    let string_str = &string;
    let up: Vec<&str> = string_str.split("\u{0000}").collect();

    let user = Users {
        name: String::from(up[1]),
        passwd: String::from(up[2]),
    };
    let pool = connect_to_db();

    // For each open connection except the sender, send the
    // string via the channel.
    let iter = (*(conns.lock().unwrap())).iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    if up[1].contains("@riot.nordgedanken.de") {
        for (y, tx) in iter {
            if y == addr {
                tx.unbounded_send(format!("+\r\n")).unwrap();
                tx.unbounded_send(format!(
                    "{} {}",
                    identifier,
                    "OK PLAIN authentication successful\r\n"
                )).unwrap();

                //Print to view for debug
                debug!(
                    "{} {}",
                    identifier,
                    "OK PLAIN authentication successful\r\n"
                );
            }
        }
    } else {
        for (y, tx) in iter {
            if y == addr {
                tx.unbounded_send(format!("+\r\n")).unwrap();
                tx.unbounded_send(format!("{} {}", identifier, "NO credentials rejected\r\n"))
                    .unwrap();

                //Print to view for debug
                debug!("{} {}", identifier, "NO credentials rejected\r\n");
            }
        }
    }
    println!("user: {} \r\n password: {}", up[1], up[2]);
}
