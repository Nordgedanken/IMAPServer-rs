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

        let mut state = state.lock().await;

        if up.len() < 2 {
            return Ok(());
        }

        // TODO make this dynamic and use real accounts
        if up[1].contains("@riot.nordgedanken.de") || up[1].contains("@localhost") {
            state.respond(addr, "+\r").await?;
            debug!("Responded: +");
            // DO NOT INLINE!
            let response = format!(
                "{} {}",
                &state
                    .peers
                    .get(&addr)
                    .expect("unable to find peer")
                    .identifier,
                "OK PLAIN authentication successful"
            );
            state.respond(addr, &response).await?;

            state
                .peers
                .get_mut(&addr)
                .expect("unable to find peer")
                .state = State::LoggedIn;

            //Print to view for debug
            debug!(
                "Responded: {} {}",
                &state
                    .peers
                    .get(&addr)
                    .expect("unable to find peer")
                    .identifier,
                "OK PLAIN authentication successful"
            );
        } else {
            state.respond(addr, "+\r").await?;
            // DO NOT INLINE!
            let response = format!(
                "{} {}",
                &state
                    .peers
                    .get(&addr)
                    .expect("unable to find peer")
                    .identifier,
                "NO credentials rejected\r"
            );
            state.respond(addr, &response).await?;

            //Print to view for debug
            debug!(
                "Responded: {} {}",
                &state
                    .peers
                    .get(&addr)
                    .expect("unable to find peer")
                    .identifier,
                "NO credentials rejected\r"
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
        let mut peer = state.peers.get_mut(&addr).expect("unable to find peer");

        peer.identifier = identifier.to_string();

        // This space is important!
        state.respond(addr, "+\r").await?;

        // DO NOT INLINE!
        // This space is important!
        let response = format!("{} {}", identifier, "+\r");

        //state.respond(addr, &response).await?;

        //Print to view for debug
        debug!("Responded: +");
        Ok(())
    }
}
