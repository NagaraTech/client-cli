use crate::config::config::{Config,get_keypairs_path};
use std::fs;
use std::ops::Add;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct ConfigBundle {
    #[command(subcommand)]
    pub subcommand: ConfigBundleSubcommand,
}

impl ConfigBundle {
    /// Run this subcommand, passing off all the work to the sub-sub-command enum
    pub async fn run(self) -> anyhow::Result<()> {
        self.subcommand.run().await
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigBundleSubcommand {
    /// Create a new keypair, name is to mark the keppair name, optional path is the output keypair json file path, default output path is
    /// `/Users/UserName/Library/Application Support/dclient/keypairs`.
    SetKeypair {
        inputName: String
    },

    /// Print config info
    Print,
}


impl ConfigBundleSubcommand {
    /// Run this command
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::SetKeypair {inputName} => {
                let directory_path = get_keypairs_path;

                let entries = fs::read_dir(directory_path())
                    .expect("Failed to read directory");
                let mut files :Vec<String>= Vec::new();
                let mut is_find = false;
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        let name = file_name.into_string().unwrap();
                        let result = &name[0..name.len()-5];
                        if inputName.eq(result){
                            is_find = true;
                            let config = Config{
                                KeypairPath: directory_path().into_os_string().into_string().unwrap().add("/").add(&name)
                            };
                            config.save().unwrap();
                            println!("Set successfully");
                            break
                        }
                    }
                }
                if !is_find{
                    println!("Can not find the keypair");
                }
            }
            Self::Print {} => {
                let config = Config::load().unwrap();
                println!("{:?}", config);
            }
        }
        Ok(())
    }
}
