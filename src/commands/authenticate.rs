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
            tx.send(format!("+\r\n")).unwrap();
            tx.send(format!(
                "{} {}",
                identifier,
                "+\r\n"
            )).unwrap();

            //Print to view for debug
            debug!(
            "{} {}",
            identifier,
            "+\r\n"
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
    use database::User;
    use helper::connect_to_db;

    let bytes = decode(args[0]).unwrap();
    let string = match String::from_utf8(bytes){
        Ok(v) => v,
        Err(e) => format!("Invalid UTF-8 sequence: {}", e),
    };
    let string_str = &string;
    let up: Vec<&str> = string_str.split("\u{0000}").collect();

    let user = User { name: String::from(up[1]), passwd: String::from(up[2]) };
    let pool = connect_to_db();

    // For each open connection except the sender, send the
    // string via the channel.
    let iter = conns.iter_mut().map(|(y, v)| (y, v));

    let identifier = args[0];
    if up[1].contains("@riot.nordgedanken.de") {
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
    }else {
        for (y, tx) in iter {
            if y == addr {
                tx.send(format!("+\r\n")).unwrap();
                tx.send(format!(
                    "{} {}",
                    identifier,
                    "NO credentials rejected\r\n"
                )).unwrap();

                //Print to view for debug
                debug!(
                "{} {}",
                identifier,
                "NO credentials rejected\r\n"
                );
            }
        }
    }
    println!("user: {} \r\n password: {}", up[1], up[2]);
}
