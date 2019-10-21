#![feature(async_await)]
#![feature(async_closure)]
#![warn(missing_debug_implementations, missing_docs)]

//#[cfg(target_os = "linux")]
//extern crate dbus;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::net::SocketAddr;

use futures_util::StreamExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = helper::get_config().expect("Unable to access config");
    let ip = config.ip;
    let ip_port = format!("{}:143", ip);
    let addr: SocketAddr = ip_port.parse().expect("your ip isn't valid");

    helper::init_log();
    // Create the event loop and TCP listener we'll accept connections on.
    let mut listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;


        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let (read, mut write) = socket.split();
            let mut reader = BufReader::new(read);
            // In a loop, read data from the socket and write the data back.
            loop {
                // Imap Logic

                // Tell the client that the server is ready to listen
                write.write_all(b"* OK [CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED] IMAP4rev1 Service Ready\r\n")
                    .await
                    .expect("failed to write data to socket");
                debug!("* OK [CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED] IMAP4rev1 Service Ready\r\n");

                let mut full_req = String::new();
                reader.get_mut().read_to_string(&mut full_req).await.expect("failes to read");

                let lines: Vec<&str> = full_req.lines().collect();

                for line in lines.iter() {
                    println!("{}", line);
                    // let msg_clone = &line.clone();
                    let args: Vec<&str> = line.split_whitespace().collect();
                    if args.len() > 1 {
                        let command = args[1].to_lowercase();
                        if command == "capability" {
                            commands::capability(args, &mut write);
                        } else if command == "login" {
                            commands::login(args, &mut write);
                        } else if command == "logout" {
                            commands::logout(args, &mut write);;
                        } else if command == "noop" {
                            commands::noop(args, &mut write);
                        } else if command == "select" {
                            commands::select(args, &mut write);
                        } else if command == "authenticate" {
                            commands::authenticate::authenticate(args, &mut write);
                        } else if command == "list" {
                            commands::list(args, &mut write);
                        } else if command == "uid" {
                            commands::uid(args, &mut write);
                        } else if command == "check" {
                            commands::check(args, &mut write);
                        } else {
                            error!("Command {} by {} is not known. dropping it.", command, addr);

                            write.write_all(b"* BAD Command not known\r\n").await.expect("Unable to write");
                        }
                    } else if args.len() == 1 {
                       commands::authenticate::parse_login_data(args, &mut write);
                    }
                    //future::ready(())
                }
            }
            // future::ready(())
        });
    }


    /*// This is a single-threaded server, so we can just use Rc and RefCell to
    // store the map of all connections we know about.
    let connections = Arc::new(Mutex::new(HashMap::new()));
    let connections2 = Arc::clone(&connections);

    // Listen for incoming connections.
    // This is similar to the iterator of incoming connections that
    // .incoming() from std::net::TcpListener, produces, except that
    // it is an asynchronous Stream of tokio::net::TcpStream instead
    // of an Iterator of std::net::TcpStream.
    let incoming = socket.incoming();

    let srv = incoming.map_err(|e| eprintln!("failed to accept socket; error = {:?}", e)).for_each(|stream: io::Result<TcpStream>| {
        info!("New Connection: {}", addr);
        let (reader, writer) = stream.unwrap().split();

        // Create a channel for our stream, which other sockets will use to
        // send us messages. Then register our address with the stream to send
        // data to us.
        let (mut tx, rx) = mpsc::unbounded_channel();
        tx.try_send(format!("{}", "* OK [CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED] IMAP4rev1 Service Ready\r\n")).unwrap();
        debug!("* OK [CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED] IMAP4rev1 Service Ready\r\n");
        (*(connections2.lock().unwrap())).insert(addr, tx);

        // Define here what we do for the actual I/O. That is, read a bunch of
        // lines from the socket and dispatch them while we also write any lines
        // from other sockets.
        //let reader = BufReader::new(reader);

        // Model the read portion of this socket by mapping an infinite
        // iterator to each line off the socket. This "loop" is then
        // terminated with an error once we hit EOF on the socket.
        // Fixme maybe?
        let iter = futures_util::stream::iter(iter::repeat(()).map(Ok::<(), io::Error>));

        let connections3 = Arc::clone(&connections2);
        let socket_reader = iter.fold(reader, move |mut reader: ReadHalf<TcpStream>, _| {
            // Read a line off the socket, failing if we're at EOF
            let line = reader.read_until(b'\n', Vec::new());
            let line = line.and_then(|(reader, vec)| if vec.len() == 0 {
                Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
            } else {
                Ok((reader, vec))
            });

            // Convert the bytes we read into a string, and then send that
            // string to all other connected clients.
            let line = line.map(|(reader, vec)| (reader, String::from_utf8(vec)));
            let connections4 = Arc::clone(&connections3);
            line.map(move |(reader, message)| {
                let conns = Arc::clone(&connections4);
                if let Ok(msg) = message {
                    println!("{}", msg);
                    let msg_clone = &msg.clone();
                    let args: Vec<&str> = msg_clone.split_whitespace().collect();
                    if args.len() > 1 {
                        let command = args[1].to_lowercase();
                        if command == "capability" {
                            commands::capability(conns, args, &addr);
                        } else if command == "login" {
                            commands::login(conns, args, &addr);
                        } else if command == "logout" {
                            commands::logout(conns, args, &addr);
                        } else if command == "noop" {
                            commands::noop(conns, args, &addr);
                        } else if command == "select" {
                            commands::select(conns, args, &addr);
                        } else if command == "authenticate" {
                            commands::authenticate::authenticate(conns, args, &addr);
                        } else if command == "list" {
                            commands::list(conns, args, &addr);
                        } else if command == "uid" {
                            commands::uid(conns, args, &addr);
                        } else if command == "check" {
                            commands::check(conns, args, &addr);
                        } else {
                            error!("Command {} by {} is not known. dropping it.", command, addr);

                            let mut conns_locked = conns.lock().unwrap();
                            let tx = (*conns_locked).get_mut(&addr).unwrap();
                            tx.try_send(format!("{}", "* BAD Command not known\r\n"))
                                .unwrap();
                        }
                    } else if args.len() == 1 {
                        commands::authenticate::parse_login_data(conns, args, &addr);
                    }
                } else {
                    error!("{:?}", message);
                    error!("Message by {} is not valid. dropping it.", addr);
                }
                reader
            })
        });

        // Whenever we receive a string on the Receiver, we write it to
        // `WriteHalf<TcpStream>`.
        let socket_writer = rx.fold(writer, |mut writer: WriteHalf<TcpStream>, msg| {
            // FIXME probably need to get a full future now
            let amt = writer.write_all(msg.into_bytes());
            let amt = amt.map(|(writer, _)| writer);
            amt.map_err(|_| ())
        });

        // Now that we've got futures representing each half of the socket, we
        // use the `select` combinator to wait for either half to be done to
        // tear down the other. Then we spawn off the result.
        let socket_reader = socket_reader.map_err(|e: io::Error| { println!("{}", e); });
        let connection = socket_reader.map(|_| ()).select(socket_writer.map(|_| ()));
        let connections3 = Arc::clone(&connections2);
        tokio::spawn(connection.then(move |_| {
            let connections4 = Arc::clone(&connections3);
            let mut connections = connections4.lock().unwrap();
            (*connections).remove(&addr);
            info!("Connection {} closed.", addr);
            Ok(())
        }));
    });

    // execute server
    srv.await;*/

    //Ok(())
}

pub mod config;
mod helper;
mod commands;
//mod server;
//mod ssl_server;
mod database;
//#[cfg(target_os = "linux")]
//mod linux_low;
