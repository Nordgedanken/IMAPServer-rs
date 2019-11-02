#![warn(missing_debug_implementations)]

use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use futures::io::ErrorKind::{ConnectionAborted, ConnectionReset};
use futures::Poll;
use futures::StreamExt;
use futures::task::Context;
use log::{debug, error, info};
use tokio::codec::{Framed, LinesCodec, LinesCodecError};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::{mpsc, Mutex};

mod commands;
mod log_helper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log_helper::setup_logger().expect("Unable to start logger.");

    // Create the shared state. This is how all the peers communicate.
    //
    // The server task will hold a handle to this. For every new client, the
    // `state` handle is cloned and passed into the task that processes the
    // client connection.
    let state = Arc::new(Mutex::new(Shared::new()));

    let addr: SocketAddr = "0.0.0.0:143".parse()?;
    let mut listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener.accept().await?;

        // Clone a handle to the `Shared` state for the new connection.
        let state = Arc::clone(&state);

        // Spawn our handler to be run asynchronously.
        tokio::spawn(async move {
            info!("{} connected", addr);
            if let Err(e) = process(state, stream, addr).await {
                error!("an error occurred; error = {:?}", e);
            }
        });
    }
}

/// Shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<String>;

/// Shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<String>;

enum State {
    LoggedOut,
    LoggedIn,
}

struct Connection {
    state: State,
    identifier: String,
    tx: Tx,
}

/// Data that is shared between all peers in the chat server.
///
/// This is the set of `Tx` handles for all connected clients. Whenever a
/// message is received from a client, it is broadcasted to all peers by
/// iterating over the `peers` entries and sending a copy of the message on each
/// `Tx`.
struct Shared {
    peers: HashMap<SocketAddr, Connection>,
}

/// The state for each connected client.
struct Peer {
    /// The TCP socket wrapped with the `Lines` codec, defined below.
    ///
    /// This handles sending and receiving data on the socket. When using
    /// `Lines`, we can work at the line level instead of having to manage the
    /// raw byte operations.
    lines: Framed<TcpStream, LinesCodec>,

    /// Receive half of the message channel.
    ///
    /// This is used to receive messages from peers. When a message is received
    /// off of this `Rx`, it will be written to the socket.
    rx: Rx,
}

impl Shared {
    /// Create a new, empty, instance of `Shared`.
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    /// Send a `LineCodec` encoded message to every peer, except
    /// for the sender.
    async fn respond(
        &mut self,
        sender: SocketAddr,
        message: &str,
    ) -> Result<(), mpsc::error::UnboundedSendError> {
        for peer in self.peers.iter_mut() {
            if *peer.0 == sender {
                peer.1.tx.send(message.into()).await?;
                break;
            }
        }

        Ok(())
    }
}

impl Peer {
    /// Create a new instance of `Peer`.
    async fn new(
        state: Arc<Mutex<Shared>>,
        lines: Framed<TcpStream, LinesCodec>,
    ) -> io::Result<Peer> {
        // Get the client socket address
        let addr = lines.get_ref().peer_addr()?;

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded_channel();

        // Add an entry for this `Peer` in the shared state map.
        let connection = Connection {
            identifier: "".to_string(),
            state: State::LoggedOut,
            tx,
        };
        state.lock().await.peers.insert(addr, connection);

        Ok(Peer { lines, rx })
    }
}

#[derive(Debug)]
enum Message {
    // Response from the server
    Response(String),

    /// A message that contains a command
    Command(String),
}

// Peer implements `Stream` in a way that polls both the `Rx`, and `Framed` types.
// A message is produced whenever an event is ready until the `Framed` stream returns `None`.
impl Stream for Peer {
    type Item = Result<Message, LinesCodecError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // First poll the `UnboundedReceiver`.

        if let Poll::Ready(Some(v)) = self.rx.poll_next_unpin(cx) {
            return Poll::Ready(Some(Ok(Message::Response(v))));
        }

        // Secondly poll the `Framed` stream.
        let result: Option<_> = futures::ready!(self.lines.poll_next_unpin(cx));

        Poll::Ready(match result {
            // We've received a message we should broadcast to others.
            Some(Ok(message)) => Some(Ok(Message::Command(message))),

            // An error occurred.
            Some(Err(e)) => Some(Err(e)),

            // The stream has been exhausted.
            None => None,
        })
    }
}

/// Process an individual imap connection
async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    // Send Capabilities
    lines
        .send(String::from("* OK [CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED] IMAP4rev1 Service Ready\r"))
        .await?;

    // Register our peer with state which internally sets up some channels.
    let mut peer = Peer::new(state.clone(), lines).await?;

    // Process incoming messages until our stream is exhausted by a disconnect.
    while let Some(result) = peer.next().await {
        match result {
            // A message was received from the current user, we should
            // treat it as a command
            Ok(Message::Command(msg)) => {
                debug!("Message raw: {}", msg);
                let args: Vec<&str> = msg.split_whitespace().collect();
                if args.len() > 1 {
                    let command = args[1].to_lowercase();

                    if command == "capability" {
                        commands::Commands::capability(args, addr, state.clone()).await?;
                    } else if command == "login" {
                        commands::Commands::login(args, addr, state.clone()).await?;
                    } else if command == "logout" {
                        commands::Commands::logout(args, addr, state.clone()).await?;
                    } else if command == "select" {
                        commands::Commands::select(args, addr, state.clone()).await?;
                    } else if command == "list" {
                        commands::Commands::list(args, addr, state.clone()).await?;
                    } else if command == "noop" {
                        commands::Commands::noop(args, addr, state.clone()).await?;
                    } else if command == "authenticate" {
                        commands::authenticate::Authentication::authenticate(
                            args,
                            addr,
                            state.clone(),
                        )
                            .await?;
                    } else {
                        error!("Command {} by {} is not known. dropping it.", command, addr);

                        let mut state = state.lock().await;
                        state
                            .respond(addr, "* BAD Command not known\r")
                            .await
                            .expect("Unable to write");
                    }
                } else if args.len() == 1 {
                    commands::authenticate::Authentication::parse_login_data(
                        args,
                        addr,
                        state.clone(),
                    )
                        .await?;
                }
            }

            Err(e) => {
                error!(
                    "an error occured while processing messages for {}; error = {:?}",
                    addr, e
                );

                // Handle some extra errors specially
                match e {
                    LinesCodecError::MaxLineLengthExceeded => {}
                    LinesCodecError::Io(e) => {
                        match e.kind() {
                            ConnectionReset => {
                                error!("connection reset");
                                return Ok(());
                            }
                            ConnectionAborted => {
                                error!("connection aborted");
                                return Ok(());
                            }
                            _ => {}
                        }
                        let mut state = state.lock().await;
                        state.peers.remove(&addr);
                    }
                }
            }
            Ok(Message::Response(msg)) => {
                peer.lines.send(msg).await?;
            }
        }
    }

    // If this section is reached it means that the client was disconnected!
    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);

        info!("{} disconnected", addr);
    }

    Ok(())
}
