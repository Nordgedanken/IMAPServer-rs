use crate::database::Database;
use crate::passwords;
use crate::server::parser::ParserResult;
use color_eyre::Report;
use futures::SinkExt;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::{Stream, StreamExt};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

mod parser;

pub async fn run(db: Database) -> color_eyre::Result<()> {
    let addr = "127.0.0.1:143";
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);
    loop {
        // Asynchronously wait for an inbound socket.
        let (socket, addr) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        let cloned_db = db.clone();
        tokio::spawn(async move {
            tracing::info!("accepted connection");
            if let Err(e) = process(cloned_db.clone(), socket, addr).await {
                tracing::error!("an error occurred; error = {:?}", e);
            }
        });
    }
}

/// The state for each connected client.
struct Peer {
    /// The TCP socket wrapped with the `Lines` codec, defined below.
    ///
    /// This handles sending and receiving data on the socket. When using
    /// `Lines`, we can work at the line level instead of having to manage the
    /// raw byte operations.
    lines: Framed<TcpStream, LinesCodec>,

    /// State of the peer
    state: State,
}

impl Peer {
    /// Create a new instance of `Peer`.
    async fn new(lines: Framed<TcpStream, LinesCodec>) -> color_eyre::Result<Peer> {
        Ok(Peer {
            lines,
            state: State::None,
        })
    }
}

#[derive(Debug)]
enum Message {
    /// A message that should be received by a client
    Received(String),
}

enum State {
    None,
    // Plain auth requires the tag
    PlainAuth(String),
    LoggedIn,
    MailboxSelected(String),
}

// Peer implements `Stream` in a way that polls both the `Rx`, and `Framed` types.
// A message is produced whenever an event is ready until the `Framed` stream returns `None`.
impl Stream for Peer {
    type Item = Result<Message, LinesCodecError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Secondly poll the `Framed` stream.
        let result: Option<_> = futures::ready!(Pin::new(&mut self.lines).poll_next(cx));

        Poll::Ready(match result {
            // We've received a message we should broadcast to others.
            Some(Ok(message)) => Some(Ok(Message::Received(message))),

            // An error occurred.
            Some(Err(e)) => Some(Err(e)),

            // The stream has been exhausted.
            None => None,
        })
    }
}

/// Process an individual mail client
async fn process(db: Database, stream: TcpStream, addr: SocketAddr) -> color_eyre::Result<()> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    // Tell the client what we can do
    lines
        .send("* OK [CAPABILITY IMAP4rev1 LITERAL+ AUTH=PLAIN] Rust Imap-Server ready.")
        .await?;

    let mut peer = Peer::new(lines).await?;

    // Process incoming messages until our stream is exhausted by a disconnect.
    while let Some(result) = peer.next().await {
        match result {
            Ok(Message::Received(msg)) => {
                tracing::info!("Message received: {}", msg);
                match peer.state {
                    State::None => {
                        match parser::commands(&msg) {
                            Ok((_, result)) => {
                                match result {
                                    ParserResult::CapabilityRequest(_) => {
                                        tracing::info!("Responding capabilities");
                                        peer.lines.send("* OK [CAPABILITY IMAP4rev1 LITERAL+ AUTH=PLAIN] Rust Imap-Server ready.").await?;
                                    }
                                    ParserResult::AuthPlainContinueRequest(tag) => {
                                        // New state: Auth
                                        peer.state = State::PlainAuth(tag.to_string());
                                        // Tell client to continue
                                        peer.lines.send("+").await?;
                                    }
                                    ParserResult::LoginRequest(tag, user, pass) => {
                                        if let Ok(hash) = db.get_user_hash(&user).await {
                                            if passwords::verify(hash, &pass) {
                                                peer.lines
                                                    .send(format!("{} OK [CAPABILITY IMAP4rev1 LITERAL+ AUTH=PLAIN] AUTHENTICATE completed", tag))
                                                    .await?;
                                                peer.state = State::LoggedIn;
                                                continue;
                                            }
                                        }

                                        peer.lines
                                            .send(format!("{} NO credentials invalid", tag))
                                            .await?;
                                        tracing::warn!("user: {:?} tried to login with invalid password or is unknown",user);
                                        peer.state = State::None;
                                    }
                                    _ => {
                                        tracing::error!("Whoops");
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Unable to parse command: {}", e);
                                peer.lines.send("* BAD unknown command").await?;
                            }
                        }
                    }
                    State::PlainAuth(ref tag) => {
                        let decoded = base64::decode(msg.to_string())?;

                        let possible_user_pass = std::str::from_utf8(&decoded);
                        match possible_user_pass {
                            Ok(user_pass) => {
                                let without_leading_null = user_pass.strip_prefix("\u{0}").unwrap();
                                let split: Vec<&str> =
                                    without_leading_null.split('\u{0}').collect();
                                let user = split[0];
                                let pass = split[1];

                                if let Ok(hash) = db.get_user_hash(user).await {
                                    if passwords::verify(hash, pass) {
                                        peer.lines
                                            .send(format!("{} OK [CAPABILITY IMAP4rev1 LITERAL+ AUTH=PLAIN] AUTHENTICATE completed", tag))
                                            .await?;
                                        peer.state = State::LoggedIn;
                                        continue;
                                    }
                                }

                                peer.lines
                                    .send(format!("{} NO credentials invalid", tag))
                                    .await?;
                                tracing::warn!(
                                    "user: {:?} tried to login with invalid password or is unknown",
                                    user
                                );
                                peer.state = State::None;
                            }
                            Err(_) => peer.lines.send("* BAD invalid passwords").await?,
                        }
                    }
                    State::LoggedIn => {
                        match parser::commands(&msg) {
                            Ok((_, result)) => {
                                match result {
                                    ParserResult::LogoutRequest(tag) => {
                                        peer.lines
                                            .send("* BYE Rust Imap-Server logging out")
                                            .await?;
                                        peer.lines
                                            .send(format!("{} OK LOGOUT completed", tag))
                                            .await?;
                                    }
                                    ParserResult::ListRequest(tag, _, _) => {
                                        //TODO real logic
                                        peer.lines
                                            .send(format!("{} OK LIST Completed", tag))
                                            .await?;
                                    }
                                    ParserResult::LSUBRequest(tag, _, _) => {
                                        //TODO real logic
                                        peer.lines
                                            .send(format!("{} OK LSUB Completed", tag))
                                            .await?;
                                    }
                                    ParserResult::CreateRequest(tag, _) => {
                                        //TODO real logic
                                        peer.lines
                                            .send(format!("{} OK CREATE Completed", tag))
                                            .await?;
                                    }
                                    ParserResult::SubscribeRequest(tag, _) => {
                                        //TODO real logic
                                        peer.lines
                                            .send(format!("{} OK SUBSCRIBE Completed", tag))
                                            .await?;
                                    }
                                    ParserResult::SelectRequest(tag, mailbox) => {
                                        //TODO real logic
                                        peer.state = State::MailboxSelected(mailbox);
                                        peer.lines
                                            .send(format!(
                                                "{} OK [READ-ONLY] SELECT Completed",
                                                tag
                                            ))
                                            .await?;
                                    }
                                    _ => {
                                        tracing::error!("Whoops");
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Unable to parse command: {}", e);
                                peer.lines.send("* BAD unknown command").await?;
                            }
                        }
                    }
                    State::MailboxSelected(ref mailbox) => match parser::commands(&msg) {
                        Ok((_, result)) => match result {
                            ParserResult::CloseRequest(tag) => {
                                peer.lines
                                    .send(format!("{} OK CLOSE completed", tag))
                                    .await?;
                                peer.state = State::LoggedIn;
                            }
                            _ => {
                                tracing::error!("Whoops");
                            }
                        },
                        Err(e) => {
                            tracing::error!("Unable to parse command: {}", e);
                            peer.lines.send("* BAD unknown command").await?;
                        }
                    },
                }
            }
            Err(e) => {
                tracing::error!(
                    "an error occurred while processing messages for {}; error = {:?}",
                    addr,
                    e
                );
            }
        }
    }

    Ok(())
}
