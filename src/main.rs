extern crate app_dirs;
extern crate config;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter;
use std::io::{Error, ErrorKind, BufReader};

use futures::Future;
use futures::stream::{self, Stream};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::io;
use tokio_io::AsyncRead;

fn main() {
    let mut config = helper::get_config();
    config.set_default("RFC", "3501").unwrap();
    config.set_default("address", "0.0.0.0:143").unwrap();

    let addr = config.get_str("address").unwrap().parse().unwrap();

    // Create the event loop and TCP listener we'll accept connections on.
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle).unwrap();
    println!("Listening on: {}", addr);

    // This is a single-threaded server, so we can just use Rc and RefCell to
    // store the map of all connections we know about.
    let connections = Rc::new(RefCell::new(HashMap::new()));

    let srv = socket.incoming().for_each(move |(stream, addr)| {
        println!("New Connection: {}", addr);
        let (reader, writer) = stream.split();

        // Create a channel for our stream, which other sockets will use to
        // send us messages. Then register our address with the stream to send
        // data to us.
        let (tx, rx) = futures::sync::mpsc::unbounded();
        tx.send(format!("{}: {}", addr, "* CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN  LOGINDISABLED")).unwrap();
        connections.borrow_mut().insert(addr, tx);

        // Define here what we do for the actual I/O. That is, read a bunch of
        // lines from the socket and dispatch them while we also write any lines
        // from other sockets.
        let connections_inner = connections.clone();
        let reader = BufReader::new(reader);

        // Model the read portion of this socket by mapping an infinite
        // iterator to each line off the socket. This "loop" is then
        // terminated with an error once we hit EOF on the socket.
        let iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
        let socket_reader = iter.fold(reader, move |reader, _| {
            // Read a line off the socket, failing if we're at EOF
            let line = io::read_until(reader, b'\n', Vec::new());
            let line = line.and_then(|(reader, vec)| {
                if vec.len() == 0 {
                    Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
                } else {
                    Ok((reader, vec))
                }
            });

            // Convert the bytes we read into a string, and then send that
            // string to all other connected clients.
            let line = line.map(|(reader, vec)| {
                (reader, String::from_utf8(vec))
            });
            let connections = connections_inner.clone();
            line.map(move |(reader, message)| {
                println!("{}: {:?}", addr, message);
                let mut conns = connections.borrow_mut();
                if let Ok(msg) = message {
                    if msg.contains("CAPABILITY") {
                        // For each open connection except the sender, send the
                        // string via the channel.
                        let iter = conns
                            .iter_mut()
                            .map(|(y, v)| (y, v));

                        let mut identifier_iter = msg.split_whitespace();
                        let identifier = identifier_iter.nth(0).unwrap();
                        for (y, tx) in iter {
                            if y == &addr {
                                tx.send(format!("{}: {}", addr, "* CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN  LOGINDISABLED")).unwrap();
                                tx.send(format!  ("{}: {}{}", addr, identifier, " OK CAPABILITY completed")).unwrap();
                            }else {
                                tx.send(format!("{}: {}", addr, "* CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN  LOGINDISABLED")).unwrap();
                            }
                        }
                    } else {
                        // For each open connection except the sender, send the
                        // string via the channel.
                        let iter = conns
                            .iter_mut()
                            .map(|(y, v)| (y, v));

                        let mut identifier_iter = msg.split_whitespace();
                        let identifier = identifier_iter.nth(0).unwrap();
                        for (y, tx) in iter {
                            if y == &addr {
                                tx.send(format!("{}: {}", addr, "* CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN  LOGINDISABLED")).unwrap();
                                tx.send(format!  ("{}: {}{}", addr, identifier, " OK CAPABILITY completed")).unwrap();
                            }else {
                                tx.send(format!("{}: {}", addr, "* CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN  LOGINDISABLED")).unwrap();
                            }
                        }
                    }
                } else {
                    let tx = conns.get_mut(&addr).unwrap();
                    println!("Message by {} is not valid. dropping it.", addr);
                    // tx.send("You didn't send valid UTF-8.".to_string()).unwrap();
                }
                reader
            })
        });

        // Whenever we receive a string on the Receiver, we write it to
        // `WriteHalf<TcpStream>`.
        let socket_writer = rx.fold(writer, |writer, msg| {
            let amt = io::write_all(writer, msg.into_bytes());
            let amt = amt.map(|(writer, _)| writer);
            amt.map_err(|_| ())
        });

        // Now that we've got futures representing each half of the socket, we
        // use the `select` combinator to wait for either half to be done to
        // tear down the other. Then we spawn off the result.
        let connections = connections.clone();
        let socket_reader = socket_reader.map_err(|_| ());
        let connection = socket_reader.map(|_| ()).select(socket_writer.map(|_| ()));
        handle.spawn(connection.then(move |_| {
            connections.borrow_mut().remove(&addr);
            println!("Connection {} closed.", addr);
            Ok(())
        }));

        Ok(())
    });

    // execute server
    core.run(srv).unwrap();
}

mod helper;