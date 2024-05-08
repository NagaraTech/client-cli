
use std::ops::Add;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::path::PathBuf;
use ed25519_dalek::Keypair;
use rand_core::{RngCore, OsRng};
use crate::config::config;
use crate::keystore::keystore::*;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct KeystoreBundle {
    #[command(subcommand)]
    pub subcommand: KeystoreBundleSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum KeystoreBundleSubcommand {
    /// Create a new keypair, name is to mark the keppair name, optional path is the output keypair json file path, default output path is
    /// `/Users/UserName/Library/Application Support/dclient/keypairs`.
    New {
        /// Keypair account name
        name: String,
        /// The path to create the working directory.
        path: Option<PathBuf>,
    },

    /// Import the `[name].json` to the default keypair directionary
    /// e.g.:
    ///
    /// $ dclient import path/xxx.json
    ///
    /// creates a file `[name].dna`, in the
    /// `/Users/UserName/Library/Application Support/dclient/keypairs`.
    Import {
        /// The path to the working directory containing a `dna.yaml` manifest.
        path: String,
    },
    /// Print the names of default path's keypairs name
    List,
}


impl KeystoreBundle {
    /// Run this subcommand, passing off all the work to the sub-sub-command enum
    pub async fn run(self) -> anyhow::Result<()> {
        self.subcommand.run().await
    }
}

impl KeystoreBundleSubcommand {
    /// Run this command
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::New { name, path } => {
                let mut csprng = OsRng {};
                let keypair = Keypair::generate(&mut csprng);
                let save_keystore = Keystore {
                    name: name.clone(),
                    keypair: keypair.to_bytes(),
                };
                let ap = get_file_path(&name);
                if is_empty() {
                    let configS = config::Config{
                        KeypairPath: ap.clone().into_os_string().into_string().unwrap()
                    };
                    configS.save().unwrap();
                }
                save_keystore.save(&ap);
            }
            Self::Import {
                path,
            } => {
                Keystore::load(path);
            }
            Self::List => {
                get_keypairs_list();
            }
        }
        Ok(())
    }
}
