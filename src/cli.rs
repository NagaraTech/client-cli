#![warn(missing_docs)]

//! A library and CLI to help create, run, and interact with Holochain conductor setups.
//! **Warning this is still WIP and subject to change**
//! There's probably a few bugs. If you find one please open an [issue](https://github.com/holochain/holochain/issues)
//! or make a PR.
//!
//! ## CLI
//!
//! The `hc` CLI makes it easy to create, modify, and run hApps that
//! you are working on or someone has sent you.
//! It has been designed to use sensible defaults but still give you
//! the configurability when that's required.
//!
//! Setups are stored in tmp directories by default and the paths are
//! persisted in a `.hc` file which is created wherever you are using
//! the CLI.

use std::process::Command;

// Useful to have this public when using this as a library.
use clap::{crate_version, Parser, Subcommand};
use crate::keystore;
use crate::config;
use crate::messager;

use lazy_static::lazy_static;



/// The main entry-point for the command.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Parser)]
#[command(infer_subcommands = true, allow_external_subcommands = true, version = crate_version!())]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: CliSubcommand,
}

/// Describes all the possible CLI arguments for `hc`, including external subcommands like `hc-scaffold`.
#[derive(Debug, Subcommand)]
#[warn(clippy::large_enum_variant)]
pub enum CliSubcommand {
    Keystore(keystore::sub_command::KeystoreBundle),
    Config(config::sub_command::ConfigBundle),
    Messager(messager::sub_command::MessagerBundle),
    // /// Work with DNA bundles.
    // Dna(hc_bundle::HcDnaBundle),
    // /// Work with hApp bundles.
    // App(hc_bundle::HcAppBundle),
    // /// Work with web-hApp bundles.
    // WebApp(hc_bundle::HcWebAppBundle),
    // /// Work with sandboxed environments for testing and development.
    // Sandbox(hc_sandbox::HcSandbox),
    // /// Run a local bootstrap and WebRTC signalling server.
    // RunLocalServices(hc_run_local_services::HcRunLocalServices),
    // /// Allow redirect of external subcommands (like `hc-scaffold` and `hc-launch`).
    // #[command(external_subcommand)]
    // External(Vec<String>),
}

impl CliSubcommand {
    /// Run this command.
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            // CliSubcommand::App(cmd) => cmd.run().await?,
            // CliSubcommand::Dna(cmd) => cmd.run().await?,
            // CliSubcommand::WebApp(cmd) => cmd.run().await?,
            // CliSubcommand::Sandbox(cmd) => cmd.run().await?,
            // CliSubcommand::RunLocalServices(cmd) => cmd.run().await,
            CliSubcommand::Config(cmd) => cmd.run().await?,
            CliSubcommand::Keystore(cmd) => cmd.run().await?,
            CliSubcommand::Messager(cmd) => cmd.run().await?,
            // CliSubcommand::External(args) => {
            //     let command_suffix = args.first().expect("Missing subcommand name");
            //     Command::new(format!("hc-{}", command_suffix))
            //         .args(&args[1..])
            //         .status()
            //         .expect("Failed to run external subcommand");
            // }
        }
        Ok(())
    }
}
