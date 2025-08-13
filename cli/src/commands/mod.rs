use anyhow::Result;
use clap::Subcommand;

use crate::cli::GlobalOpts;

pub mod auth;
pub mod status;

#[derive(Subcommand)]
pub enum Commands {
    /// Authentication commands
    Login {
        /// API endpoint URL
        #[arg(long)]
        endpoint: Option<String>,
    },

    /// Show status information
    Status {
        #[command(subcommand)]
        target: Option<StatusTarget>,
    },

    /// Show version information
    Version,
}

#[derive(Subcommand)]
pub enum StatusTarget {
    /// Show dashboard overview
    Dashboard,
    /// Show network status
    Network {
        /// Network name
        name: Option<String>,
    },
    /// Show application status
    App {
        /// Application name
        name: Option<String>,
    },
}

impl Commands {
    pub async fn execute(&self, global_opts: &GlobalOpts) -> Result<()> {
        match self {
            Commands::Login { endpoint } => {
                auth::login_command(endpoint.clone(), global_opts).await
            }
            Commands::Status { target } => {
                status::status_command(target.as_ref(), global_opts).await
            }
            Commands::Version => {
                println!("rig {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
        }
    }
}
