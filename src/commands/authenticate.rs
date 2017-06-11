use std;
use futures;
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
            tx.send(format!("+\r\n")).unwrap();
            tx.send(format!("{} {}", identifier, "OK PLAIN authentication successful\r\n")).unwrap();

            //Print to view for debug
            println!("{} {}", identifier, "OK PLAIN authentication successful\r\n");
        }
    }
}

pub fn parse_login_data (){

}