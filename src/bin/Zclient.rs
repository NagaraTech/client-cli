use clap::Parser;
use client_cli::cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_some() {
        //log module
    }
    let cli = cli::Cli::parse();
    cli.subcommand.run().await
}
