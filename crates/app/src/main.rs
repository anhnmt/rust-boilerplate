use clap::{Parser, Subcommand};
use tracing_subscriber::fmt::format;
use common::config::Config;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Hello
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .event_format(format().compact())
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = Config::new()?;
    tracing::info!(?config);

    let args = Args::parse();
    match args.cmd {
        Some(Command::Hello) => {
            tracing::info!("Hello, world!")
        }
        None => {
            tracing::info!("No command provided")
        }
    }

    Ok(())
}