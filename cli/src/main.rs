#[macro_use]
extern crate clap;

use std::error::Error;

use clap::{App, Arg, SubCommand};
use log::{error, info};

use IMAPServer_database::mailbox::Mailbox;
use IMAPServer_database::setup;

mod log_helper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log_helper::setup_logger().expect("Unable to start logger.");
    let matches = App::new("mailbox-cli")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Allows you to manage, create and remove mailboxes")
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a new Mailbox")
                .arg(
                    Arg::with_name("username")
                        .help("the email address of the user to add")
                        .takes_value(true)
                        .short("u")
                        .multiple(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("password")
                        .help("the password of the user to add")
                        .takes_value(true)
                        .multiple(false)
                        .short("p")
                        .required(true),
                ),
        )
        .get_matches();

    setup();

    if matches.is_present("add") {
        if let Some(ref matches) = matches.subcommand_matches("add") {
            let result = Mailbox::new(
                matches.value_of("username").unwrap().parse()?,
                matches.value_of("password").unwrap().parse()?,
            );
            match result {
                Some(_) => info!("Added User {}", matches.value_of("username").unwrap()),
                None => error!(
                    "Failed to add User {}",
                    matches.value_of("username").unwrap()
                ),
            }
        }
    }

    Ok(())
}
