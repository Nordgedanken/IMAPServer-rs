use std::net::SocketAddr;
use std::sync::Arc;

use log::debug;
use tokio::sync::{mpsc, Mutex};

use crate::mailbox::Mailbox;
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
                state
                    .respond(addr, "* LIST (\\Marked \\HasNoChildren \\Noinferiors \\Subscribed) \".\" INBOX\r")
                    .await?;
                state
                    .respond(addr, "* LIST  (\\Subscribed) \".\" \"test\"\r")
                    .await?;

                state
                    .respond(
                        addr,
                        "* LIST  (\\Subscribed \\Noinferiors) \".\" \"Trash\"\r",
                    )
                    .await?;

                let response = format!("{} {}", identifier, "OK LIST Completed\r");

                state.respond(addr, &response).await?;

                //Print to view for debug
                debug!("Responded: {}", "* LIST (\\Marked \\HasNoChildren \\Noinferiors \\Subscribed) \".\" INBOX");
                debug!("Responded: {}", "* LIST (\\Subscribed) \".\" \"test\"");
                debug!(
                    "Responded: {}",
                    "* LIST (\\Subscribed \\Noinferiors) \".\" \"Trash\""
                );
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
                state
                    .respond(addr, "* LSUB (\\HasNoChildren) \".\" INBOX\r")
                    .await?;
                state
                    .respond(addr, "* LSUB  (\\Subscribed) \".\" \"test\"\r")
                    .await?;
                state
                    .respond(
                        addr,
                        "* LSUB  (\\Subscribed \\Noinferiors) \".\" \"Trash\"\r",
                    )
                    .await?;

                let response = format!("{} {}", identifier, "OK LSUB Completed\r");

                state.respond(addr, &response).await?;

                //Print to view for debug
                debug!("Responded: {}", "* LSUB (\\HasNoChildren) \".\" INBOX");
                debug!("Responded: {}", "* LSUB (\\Subscribed) \".\" \"test\"");
                debug!(
                    "Responded: {}",
                    "* LSUB (\\Subscribed \\Noinferiors) \".\" \"Trash\""
                );
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
                    "* STATUS {} (MESSAGES 1 UIDNEXT 44292 UNSEEN 1 RECENT 1)\r",
                    path
                );
                state.respond(addr, &response).await?;
                let response_completed = format!("{} {}", identifier, "OK STATUS Completed\r");

                state.respond(addr, &response_completed).await?;

                //Print to view for debug
                debug!(
                    "Responded: * STATUS {} (MESSAGES 1 UIDNEXT 44292 UNSEEN 1 RECENT 1)",
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
                state
                    .respond(addr, "* NAMESPACE ((\"\" \".\")) NIL  NIL\r")
                    .await?;

                let response = format!("{} {}", identifier, "OK NAMESPACE Completed\r");

                state.respond(addr, &response).await?;

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
                state
                    .respond(
                        addr,
                        "* ID (\"name\" \"IMAPServer-rs\" \"version\" \"0.1.0\")\r",
                    )
                    .await?;

                let response = format!("{} {}", identifier, "OK ID Completed\r");

                state.respond(addr, &response).await?;

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
        let mut state = state.lock().await;

        match state.peers.get(&addr).expect("unable to find peer").state {
            State::LoggedIn => {
                state.respond(addr, "* 1 EXISTS\r").await?;
                state
                    .respond(
                        addr,
                        "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)\r",
                    )
                    .await?;
                state.respond(addr, "* 1 RECENT\r").await?;
                state
                    .respond(addr, "* OK [UNSEEN 1] Message 1 is first unseen\r")
                    .await?;

                state
                    .respond(
                        addr,
                        "* OK [PERMANENTFLAGS (\\Deleted \\Seen \\*)] Limited\r",
                    )
                    .await?;

                state
                    .respond(addr, "* OK [UIDVALIDITY 3857529045] UIDs valid\r")
                    .await?;

                state
                    .respond(addr, "* OK [UIDNEXT 44292] Predicted next UID\r")
                    .await?;

                let response = format!("{} {}", identifier, "OK [READ-WRITE] SELECT completed\r");

                state.respond(addr, &response).await?;

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
                let mailboxdummy = Mailbox::new();

                let path = format!("{}/{}", mailboxdummy.mailbox_root, path.replace("\"", "").replace(".", "/"));
                debug!("{}", path);
                mailboxdummy
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
