use clap::{Parser, Subcommand};
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Hello
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct AppConfig {
    list: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = Config::builder()
        // Add in the default configuration file
        .add_source(File::with_name("config.example").required(false))
        .add_source(File::with_name("config").required(false))
        // Add in settings from the environment
        .add_source(
            Environment::default()
                .try_parsing(true)
                .separator("_")
                .list_separator(" ")
        )
        .build()?;

    let app: AppConfig = config.try_deserialize()?;

    tracing::info!(?app);

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