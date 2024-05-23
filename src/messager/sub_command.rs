use crate::config::config::{Config, get_keypairs_path};
use std::fs;
use std::ops::Add;
use clap::{Parser, Subcommand};
use clap::builder::Str;
use crate::messager::messager::{account_read_message, account_send_message, save_state, load_state, AppState};
use crate::messager::websocketConnection;
use std::io::{self, Write};

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
    ReceiveMessage {
        url: String
    },
    SendMsg {
        ws_url: String,
        to: String,
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
                save_state(url);
            }
            Self::SendMsg { ws_url, to } => {
                let mut app_state = AppState::init_state(ws_url, to).await;

                let mut input = String::new();

                loop {
                    // Clear the previous input
                    input.clear();
                    // Prompt the user
                    print!("Enter message sent (or 'exit' to quit): ");
                    io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
                    // Read input from the user
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    // Trim the newline character and check if the input is "exit"
                    if input.trim() == "exit" {
                        println!("Exiting...");
                        break;
                    }
                    let data = app_state.buildMsg(input.trim());
                    app_state.send_message(data).await;
                }


                // account_send_message("406b4c9bb2117df0505a58c6c44a99c8817b7639d9c877bdbea5a8e4e0412740".parse()?, to, msg).await;
            }
            Self::Test {} => {
                let config = Config::load().unwrap();
                println!("{:?}", config);
                account_send_message("406b4c9bb2117df0505a58c6c44a99c8817b7639d9c877bdbea5a8e4e0412740".parse()?, "f78e5a39e3d433986c4b8026d0baeb62b7eb845c29bb83a04b79d645ef7efbba".parse()?, "hello".parse()?).await;
            }
            Self::ReceiveMessage { url } => {
                account_read_message(url).await;
            }
        }
        Ok(())
    }
}
