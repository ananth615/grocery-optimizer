//! Binary entry point for the grocery-optimizer scraper CLI.
//!
//! Phase one: prove ingestion against one real, login-gated store site.
//! The CLI exists to drive the scraper during development and testing. The
//! future API server will invoke the scraper directly; the CLI is not a
//! shipped surface.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use grocery_optimizer_scraper::Config;
use grocery_optimizer_scraper::read_config;

#[derive(Debug, Parser)]
#[command(
    name = "grocery-optimizer-scraper",
    version,
    about = "Scrape past receipts from a grocery store login."
)]
struct Cli {
    /// Path to the local secrets TOML file.
    #[arg(short, long, default_value = "./secrets.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Log in and pull past receipts from the configured store.
    Scrape,
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    let cfg: Config = read_config(&cli.config)?;
    tracing::info!(store_url = %cfg.store_url, "config loaded");

    match cli.command {
        Command::Scrape => {
            tracing::info!(store_url = %cfg.store_url, "scrape subcommand invoked (not implemented yet)");
            Ok(())
        }
    }
}
