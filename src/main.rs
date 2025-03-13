use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
mod commands;
mod logging;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    quiet: u8,

    #[command(subcommand)]
    command: cli::Commands,
}

fn main() -> anyhow::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .build();
    tracing::subscriber::set_global_default(subscriber)?;

    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the command
    cli.command.execute()?;

    Ok(())
}
