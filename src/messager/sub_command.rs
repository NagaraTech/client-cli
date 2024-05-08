use crate::config::config::{Config, get_keypairs_path};
use std::fs;
use std::ops::Add;
use clap::{Parser, Subcommand};
use clap::builder::Str;
use crate::messager::messager::{account_read_message, account_send_message};
use crate::messager::websocketConnection;
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct MessagerBundle {
    #[command(subcommand)]
    pub subcommand: MessagerBundleSubcommand,
}

impl MessagerBundle {
    /// Run this subcommand, passing off all the work to the sub-sub-command enum
    pub async fn run(self) -> anyhow::Result<()> {
        self.subcommand.run().await
    }
}

#[derive(Debug, Subcommand)]
pub enum MessagerBundleSubcommand {
    /// Create a new keypair, name is to mark the keppair name, optional path is the output keypair json file path, default output path is
    /// `/Users/UserName/Library/Application Support/dclient/keypairs`.
    // SetKeypair {
    //     inputName: String
    // },

    /// Print config info
    Test,
    ReceiveMessage,
    SendMsg {
        from: String,
        to: String,
        msg: String,
    },
    SetWSConnection {
        url: String
    },
}


impl MessagerBundleSubcommand {
    /// Run this command
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::SetWSConnection { url } => {
                websocketConnection::init_ws_conn(url).await;
            }
            Self::SendMsg { from, to, msg } => {
                account_send_message(from, to, msg).await;
            }
            Self::Test {} => {
                let config = Config::load().unwrap();
                println!("{:?}", config);
                account_send_message("406b4c9bb2117df0505a58c6c44a99c8817b7639d9c877bdbea5a8e4e0412740".parse()?, "406b4c9bb2117df0505a58c6c44a99c8817b7639d9c877bdbea5a8e4e0412740".parse()?, "hello".parse()?).await;
            }
            Self::ReceiveMessage {} => {
                account_read_message().await;
            }
        }
        Ok(())
    }
}
