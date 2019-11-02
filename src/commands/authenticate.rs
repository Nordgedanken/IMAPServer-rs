use std::net::SocketAddr;
use std::result::Result::{Err, Ok};
use std::sync::Arc;

use base64::decode;
use log::debug;
use tokio::sync::{mpsc, Mutex};

use crate::{Shared, State};

pub(crate) struct Authentication;

impl Authentication {
    pub async fn parse_login_data(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let bytes = decode(args[0]).expect("unable to decode");
        let string = match String::from_utf8(bytes) {
            Ok(v) => v,
            Err(e) => format!("Invalid UTF-8 sequence: {}", e),
        };
        let string_str = &string;
        let up: Vec<&str> = string_str.split("\u{0000}").collect();

        let identifier = args[0];

        let mut state = state.lock().await;

        if up.len() < 2 { return Ok(()); }

        // TODO make this dynamic and use real accounts
        if up[1].contains("@riot.nordgedanken.de") {
            state.respond(addr, "+\r\n").await?;
            // DO NOT INLINE!
            let response = format!(
                "{} {}",
                identifier, "OK PLAIN authentication successful\r\n"
            );
            state.respond(addr, &response).await?;

            state.peers.get_mut(&addr).expect("unable to find peer").state = State::LoggedIn;

            //Print to view for debug
            debug!(
                "Responded: {} {}",
                identifier, "OK PLAIN authentication successful\r\n"
            );
        } else {
            state.respond(addr, "+\r\n").await?;
            // DO NOT INLINE!
            let response = format!("{} {}", identifier, "NO credentials rejected\r\n");
            state.respond(addr, &response).await?;

            //Print to view for debug
            debug!(
                "Responded: {} {}",
                identifier, "NO credentials rejected\r\n"
            );
        }
        debug!("user: {} \r\n password: {}", up[1], up[2]);
        Ok(())
    }

    pub async fn authenticate(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        // This space is important!
        state.respond(addr, "+\r\n").await?;

        // DO NOT INLINE!
        // This space is important!
        let response = format!("{} {}", identifier, "+\r\n");

        //state.respond(addr, &response).await?;

        //Print to view for debug
        debug!("Responded: +");
        Ok(())
    }
}
