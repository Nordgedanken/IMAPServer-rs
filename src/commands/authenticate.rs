use std;
use futures;
use base64;

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

pub fn parse_login_data <'a>(
    mut conns: std::cell::RefMut<'a,
        std::collections::HashMap<std::net::SocketAddr,
            futures::sync::mpsc::UnboundedSender<std::string::String>>>,
    args: Vec<&str>,
    addr: &'a std::net::SocketAddr
){
    use base64::decode;
    let bytes = decode(args[0]).unwrap();
    let string = match String::from_utf8(bytes){
        Ok(v) => v,
        Err(e) => format!("Invalid UTF-8 sequence: {}", e),
    };
    let string_str = &string;
    let up: Vec<&str> = string_str.split("\u{0000}").collect();
    println!("user: {} \r\n password: {}", up[0], up[1]);
}
