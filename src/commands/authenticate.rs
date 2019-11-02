use std::result::Result::{Err, Ok};

use base64::decode;
use log::debug;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::split::WriteHalf;

pub async fn authenticate(args: Vec<&str>, write: &mut WriteHalf<'_>) {
    let identifier = args[0];

    write
        .write_all(b"+\r\n")
        .await
        .expect("failed to write data to socket");
    write
        .write_all(format!("{} {}", identifier, "+\r\n").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{} {}", identifier, "+\r\n");
}

pub async fn parse_login_data(args: Vec<&str>, write: &mut WriteHalf<'_>) {
    let bytes = decode(args[0]).unwrap();
    let string = match String::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => format!("Invalid UTF-8 sequence: {}", e),
    };
    let string_str = &string;
    let up: Vec<&str> = string_str.split("\u{0000}").collect();

    let identifier = args[0];
    if up[1].contains("@riot.nordgedanken.de") {
        write
            .write_all(b"+\r\n")
            .await
            .expect("failed to write data to socket");
        write
            .write_all(
                format!(
                    "{} {}",
                    identifier, "OK PLAIN authentication successful\r\n"
                )
                .as_ref(),
            )
            .await
            .expect("failed to write data to socket");

        //Print to view for debug
        debug!(
            "{} {}",
            identifier, "OK PLAIN authentication successful\r\n"
        );
    } else {
        write
            .write_all(b"+\r\n")
            .await
            .expect("failed to write data to socket");
        write
            .write_all(format!("{} {}", identifier, "NO credentials rejected\r\n").as_ref())
            .await
            .expect("failed to write data to socket");

        //Print to view for debug
        debug!("{} {}", identifier, "NO credentials rejected\r\n");
    }
    println!("user: {} \r\n password: {}", up[1], up[2]);
}
