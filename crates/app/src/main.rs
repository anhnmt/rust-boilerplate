use clap::{Parser, Subcommand};
use common::config::Config;
use common::database::{database_connection, migrate};
use tracing_subscriber::fmt::format;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Migrate,
    Hello,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .event_format(format().compact())
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = Config::new()?;
    tracing::info!(?config);

    let pool = database_connection(config.database).await?;
    if pool.is_closed() {
        tracing::error!("Failed to connect to database");
        return Ok(());
    }

    let args = Args::parse();
    match args.cmd {
        Some(Command::Migrate) => {
            tracing::info!("Migrations running...");
            migrate(&pool).await?;
            tracing::info!("Migrations ran successfully!");
        }
        Some(Command::Hello) => {
            tracing::info!("Hello, world!")
        }
        None => {
            tracing::info!("No command provided")
        }
    }

    Ok(())
}