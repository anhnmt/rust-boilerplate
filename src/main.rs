use clap::{Parser, Subcommand};

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
        .with_max_level(tracing::Level::INFO)
        .init();

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