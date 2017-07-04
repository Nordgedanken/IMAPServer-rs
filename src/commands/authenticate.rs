use std;
use futures;
pub fn authenticate <'a>(
    mut conns: std::cell::RefMut<'a,
        std::collections::HashMap<std::net::SocketAddr,
            futures::sync::mpsc::UnboundedSender<std::string::String>>>,
    args: Vec<&str>,
    addr: &'a std::net::SocketAddr
){
    // For each open connection except the sender, send the
    // string via the channel.
    let iter = conns.iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    for (y, tx) in iter {
        if y == addr {
            println!("{}", args[2]);
            let bytes = base64::decode(args[2]).unwrap();
            let s = match str::from_utf8(bytes) {
                Ok(v) => v,
                Err(e) => error!("Invalid UTF-8 sequence: {}", e),
            };

            println!("{}", s);
            tx.send(format!("+\r\n")).unwrap();
            tx.send(format!(
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
}

pub fn parse_login_data() {}
