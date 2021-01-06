use clap::{clap_app, ArgMatches};
use color_eyre::eyre::Result;

mod cli;
mod database;
mod passwords;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    let opts = clap_app!("imap-server" =>
        (about: "A Rust imap server")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg verbose: -v --verbose "Print test information verbosely")
        (@subcommand "user" =>
            (@subcommand "add" =>
                (about: "adds a user")
                (@arg USERNAME: -u --username +takes_value +required "The username of the new user")
                (@arg PASSWORD: -p --password +takes_value +required "The password of the new user")
            )
            (@subcommand "change-password" =>
                (about: "changes the password of the given user")
                (@arg USERNAME: -u --username +takes_value +required "The username of the new user")
                (@arg PASSWORD: -p --password +takes_value +required "The password of the new user")
            )
            (@subcommand "remove" =>
                (@arg USERNAME: -u --username +takes_value +required "The username of the new user")
                (about: "removes a user")
            )
            (about: "user operations")
        )
    )
    .get_matches();

    let database = database::Database::open().await?;
    match opts.subcommand() {
        Some(("user", user)) => match user.subcommand() {
            Some(("add", add_user)) => {
                if let Some(username) = add_user.value_of("USERNAME") {
                    if let Some(password) = add_user.value_of("PASSWORD") {
                        cli::add(database, username.to_string(), password.to_string()).await?;
                    }
                }
            }
            Some(("change-password", add_user)) => {
                if let Some(username) = add_user.value_of("USERNAME") {
                    if let Some(password) = add_user.value_of("PASSWORD") {
                        cli::change_password(database, username.to_string(), password.to_string())
                            .await?;
                    }
                }
            }
            Some(("remove", remove_user)) => {
                if let Some(username) = remove_user.value_of("USERNAME") {
                    cli::remove(database, username.to_string()).await?;
                }
            }
            _ => {}
        },
        _ => {
            tracing::info!("Starting...");
            server::run(database).await?
        }
    }
    Ok(())
}
