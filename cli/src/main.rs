use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

mod cli;
mod commands;

use cli::{Cli, GlobalOpts};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(&cli.global)?;

    info!("Starting Rig CLI");

    // Execute the command
    cli.command.execute(&cli.global).await
}

fn init_logging(opts: &GlobalOpts) -> Result<()> {
    let filter = if opts.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false);

    if opts.json {
        subscriber.json().init();
    } else {
        subscriber.init();
    }

    Ok(())
}
