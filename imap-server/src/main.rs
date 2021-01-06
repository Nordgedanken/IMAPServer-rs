use clap::Clap;
use color_eyre::eyre::Result;

mod cli;
mod server;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "MTRNord <mtrnord1@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "config.yml")]
    config: String,
    /// Print debug info
    #[clap(short)]
    debug: bool,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Server(Server),
    Cli(Cli),
}

#[derive(Clap)]
struct Server {}

#[derive(Clap)]
struct Cli {}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("Starting...");
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Server(_) => server::run().await?,
        SubCommand::Cli(_) => {}
    }
    Ok(())
}
