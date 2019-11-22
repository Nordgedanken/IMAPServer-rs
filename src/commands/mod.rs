use std::net::SocketAddr;
use std::sync::Arc;

use log::debug;
use tokio::sync::{mpsc, Mutex};

use IMAPServer_shared::mailbox::Mailbox;

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
                "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT NAMESPACE ID LIST-EXTENDED ENABLE LOGINDISABLED\r",
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

        let one = "* BYE IMAP4rev1 Server logging out\r\n";

        let response = format!("{}{}", identifier, " OK LOGOUT completed\r");

        let complete = [one, &response].concat();

        state.respond(addr, &complete).await?;

        state.peers.remove(&addr);

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

    // TODO actually implement
    pub async fn enable(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        let response = format!("{} {}", identifier, "OK enabled\r");

        state.respond(addr, &response).await?;

        //Print to view for debug
        debug!("Responded: {} {}", identifier, "OK enabled");
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
                let email = &state.peers.get(&addr).expect("unable to find peer").user;

                let mailbox = Mailbox::load(email.to_string())
                    .await
                    .expect("failed to get mailbox");

                let mut folders: Vec<String> =
                    mailbox.get_list().await.expect("unable to get folders");
                debug!("Responded: {:?}", folders);

                let response = format!("{} {}", identifier, "OK LIST Completed\r");
                folders.push(response);

                let complete = folders.concat();

                state.respond(addr, &complete).await?;

                //Print to view for debug

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

    pub async fn lsub(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                let email = &state.peers.get(&addr).expect("unable to find peer").user;

                let mailbox = Mailbox::load(email.to_string())
                    .await
                    .expect("failed to get mailbox");

                let mut folders: Vec<String> =
                    mailbox.get_lsub().await.expect("unable to get folders");
                debug!("Responded: {:?}", folders);

                let response = format!("{} {}", identifier, "OK LSUB Completed\r");
                folders.push(response);

                let complete = folders.concat();

                state.respond(addr, &complete).await?;

                //Print to view for debug
                debug!("Responded: {} {}", identifier, "OK LSUB Completed");
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

    pub async fn status(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];
        let path = args[2];

        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                let response = format!(
                    "* STATUS {} (MESSAGES 2 UIDNEXT 44292 UNSEEN 1 RECENT 1)\r\n",
                    path.replace("\"", "")
                );

                let response_completed = format!("{} {}", identifier, "OK STATUS Completed\r");

                let complete = [response, response_completed].concat();

                state.respond(addr, &complete).await?;

                //Print to view for debug
                debug!(
                    "Responded: * STATUS {} (MESSAGES 2 UIDNEXT 44292 UNSEEN 1 RECENT 1)",
                    path
                );
                debug!("Responded: {} {}", identifier, "OK STATUS Completed");
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

    pub async fn namespace(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                let one = "* NAMESPACE ((\"\" \".\")) NIL  NIL\r\n";

                let response = format!("{} {}", identifier, "OK NAMESPACE Completed\r");

                let complete = [one, &response].concat();

                state.respond(addr, &complete).await?;

                //Print to view for debug
                debug!("Responded: {}", "* NAMESPACE ((\"\" \".\")) NIL  NIL");
                debug!("Responded: {} {}", identifier, "OK NAMESPACE Completed");
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

    pub async fn id(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];

        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                let one = "* ID (\"name\" \"IMAPServer-rs\" \"version\" \"0.1.0\")\r\n";

                let response = format!("{} {}", identifier, "OK ID Completed\r");

                let complete = [one, &response].concat();

                state.respond(addr, &complete).await?;

                //Print to view for debug
                debug!(
                    "Responded: {}",
                    "* ID (\"name\" \"IMAPServer-rs\" \"version\" \"0.1.0\")"
                );
                debug!("Responded: {} {}", identifier, "OK ID Completed");
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
        let command = args[1];
        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                let one = "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)\r\n";
                let two = "* OK [PERMANENTFLAGS (\\Deleted \\Seen \\*)] Limited\r\n";
                let three = "* 2 EXISTS\r\n";
                let four = "* 1 RECENT\r\n";
                let five = "* OK [UNSEEN 1] First unseen\r\n";

                /*state
                    .respond(addr, "* OK [UIDVALIDITY 3857529045] UIDs valid\r")
                    .await?;

                state
                    .respond(addr, "* OK [UIDNEXT 44292] Predicted next UID\r")
                    .await?;*/

                if command == "select" {
                    let response =
                        format!("{} {}", identifier, "OK [READ-WRITE] SELECT completed\r");

                    let complete = [one, two, three, four, five, &response].concat();

                    state.respond(addr, &complete).await?;
                } else {
                    let response =
                        format!("{} {}", identifier, "OK [READ-ONLY] SELECT completed\r");

                    let complete = [one, two, three, four, five, &response].concat();

                    state.respond(addr, &complete).await?;
                }

                //Print to view for debug
                debug!(
                    "Responded (truncated): {} {}",
                    identifier, "OK [READ-WRITE] SELECT completed"
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

    pub async fn create(
        args: Vec<&str>,
        addr: SocketAddr,
        state: Arc<Mutex<Shared>>,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        let identifier = args[0];
        let path = args[2];

        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                let email = &state.peers.get(&addr).expect("unable to find peer").user;

                let mailbox = Mailbox::load(email.to_string())
                    .await
                    .expect("failed to get mailbox");

                let path = path.replace("\"", "").replace(".", "/");
                debug!("{}", path);
                mailbox
                    .create_folder(path)
                    .await
                    .expect("failed to create folder");

                // TODO handle error and respond that one to the client

                let response = format!("{} {}", identifier, "OK CREATE Completed\r");

                state.respond(addr, &response).await?;

                //Print to view for debug
                debug!("Responded: {} {}", identifier, "OK CREATE Completed");
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
