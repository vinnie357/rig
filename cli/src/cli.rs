use clap::Parser;
use std::path::PathBuf;

use crate::commands::Commands;

#[derive(Parser)]
#[command(
    name = "rig",
    about = "Command-line interface for the Max platform",
    version,
    long_about = None
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct GlobalOpts {
    /// Output format
    #[arg(long, short = 'o', value_enum, default_value = "table")]
    pub output: OutputFormat,

    /// Enable verbose logging
    #[arg(long, short)]
    pub verbose: bool,

    /// Output JSON instead of human-readable format
    #[arg(long)]
    pub json: bool,

    /// Configuration file path
    #[arg(long, short)]
    pub config: Option<PathBuf>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

impl GlobalOpts {
    pub fn is_json_output(&self) -> bool {
        self.json || matches!(self.output, OutputFormat::Json)
    }
}
