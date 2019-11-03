use std::net::SocketAddr;
use std::sync::Arc;

use log::debug;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::split::WriteHalf;
use tokio::sync::{mpsc, Mutex};

use crate::{Shared, State};

pub mod authenticate;

pub(crate) struct Commands;

impl Commands {
    pub async fn capability(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        state
            .respond(
                addr,
                "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED\r",
            )
            .await?;

        let response = format!("{}{}", identifier, " OK CAPABILITY completed\r");
        state.respond(addr, &response).await?;

        //Print to view for debug
        debug!(
            "Responded: {}",
            "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED"
        );
        debug!("Responded: {}{}", identifier, " OK CAPABILITY completed");
        Ok(())
    }

    pub async fn logout(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        state
            .respond(addr, "* BYE IMAP4rev1 Server logging out\r")
            .await?;

        state.peers.remove(&addr);

        let response = format!("{}{}", identifier, " OK LOGOUT completed");
        state.respond(addr, &response).await?;

        //Print to view for debug
        debug!("Responded: {}", "* BYE IMAP4rev1 Server logging out\r");
        debug!("Responded: {}{}", identifier, " OK LOGOUT completed\r");
        Ok(())
    }

    pub async fn noop(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        let response = format!("{} {}", identifier, "OK NOOP completed\r");

        state.respond(addr, &response).await?;

        //Print to view for debug
        debug!("Responded: {} {}", identifier, "OK NOOP completed");
        Ok(())
    }

    pub async fn list(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                state.respond(addr, "* LIST  () \"/\" \"\"\r").await?;

                let response = format!("{} {}", identifier, "OK LIST Completed\r");

                state.respond(addr, &response).await?;

                //Print to view for debug
                debug!("Responded: {}", "* LIST () \"/\" \"INBOX\"");
                debug!("Responded: {} {}", identifier, "OK LIST Completed");
            }
            _ => {
                let response = format!("{} {}", identifier, "NO Please Login first!\r");

                state.respond(addr, &response).await?;

                //Print to view for debug
                debug!("Responded: {} {}", identifier, "NO Please Login first!");
            }
        }

        Ok(())
    }

    pub async fn select(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];
        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                state.respond(addr, "* 1 EXISTS\r").await?;
                state.respond(addr, "* 1 RECENT\r").await?;
                state
                    .respond(addr, "* OK [UNSEEN 1] Message 1 is first unseen\r")
                    .await?;
                state
                    .respond(addr, "* OK [UIDNEXT 1] Predicted next UID\r")
                    .await?;
                state
                    .respond(
                        addr,
                        "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)\r",
                    )
                    .await?;
                state
                    .respond(
                        addr,
                        "* OK [PERMANENTFLAGS (\\Deleted \\Seen \\*)] Limited\r",
                    )
                    .await?;

                let response = format!("{} {}", identifier, "OK [READ-ONLY] SELECT completed\r");

                state.respond(addr, &response).await?;

                //Print to view for debug
                debug!(
                    "Responded (truncated): {} {}",
                    identifier, "OK [READ-ONLY] SELECT completed"
                );
            }
            _ => {
                let response = format!("{} {}", identifier, "NO Please Login first!\r");

                state.respond(addr, &response).await?;

                //Print to view for debug
                debug!("Responded: {} {}", identifier, "NO Please Login first!");
            }
        }

        Ok(())
    }
}
