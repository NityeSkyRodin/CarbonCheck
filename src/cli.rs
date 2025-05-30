use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "carboncheck", version = "0.1.0", about = "Laravel Carbon Footprint Analyzer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Analyze {
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    Report {
        #[arg(short, long)]
        output: Option<String>,
    },
}
