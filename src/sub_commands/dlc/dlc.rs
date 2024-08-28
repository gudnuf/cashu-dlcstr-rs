use crate::sub_commands::dlc::contract_messages::GlobalContractOffer;
use crate::sub_commands::dlc::nostr_events::create_offer_event;
use anyhow::Result;
use clap::{Args, Subcommand};
use nostr_sdk::{Client, Keys};

use super::{contract_messages::GlobalAttestation, nostr_events::create_attestation_event};

const RELAYS: [&str; 1] = ["wss://relay.damus.io"];

#[derive(Args)]
pub struct DLCSubCommand {
    #[command(subcommand)]
    pub command: DLCCommands,
}

#[derive(Subcommand)]
pub enum DLCCommands {
    Create {
        key: String, // placeholder for whatever contract metadaa is needed
        arg2: Option<String>,
    },

    Attest {
        key: String,
    }, // Add more subcommands and their arguments as needed
}

pub async fn dlc(sub_command_args: &DLCSubCommand) -> Result<()> {
    //let keys =
    //   Keys::parse("nsec15jldh0htg2qeeqmqd628js8386fu4xwpnuqddacc64gh0ezdum6qaw574p").unwrap();

    match &sub_command_args.command {
        DLCCommands::Create { key, arg2 } => {
            let keys = Keys::parse(key).unwrap();
            let global_contract_offer = GlobalContractOffer::new();

            let event = create_offer_event(
                &global_contract_offer,
                &RELAYS.iter().map(|r| r.to_string()).collect::<Vec<_>>(),
                &keys,
            )?;

            let client = Client::new(&keys);
            for relay in RELAYS.iter() {
                client.add_relay(relay.to_string()).await?;
            }
            client.connect().await;

            // TODO: create a publish_event function ?
            match client.send_event(event).await {
                Ok(event_id) => println!("Event published: {}", event_id.to_string()),
                Err(e) => eprintln!("Error publishing event: {}", e),
            }
        }

        DLCCommands::Attest { key } => {
            let global_attestation = GlobalAttestation::new();

            let keys = Keys::parse(key).unwrap();

            let event = create_attestation_event(
                global_attestation,
                &RELAYS.iter().map(|r| r.to_string()).collect::<Vec<_>>(),
                &keys,
            )?;

            let client = Client::new(&keys);
            for relay in RELAYS.iter() {
                client.add_relay(relay.to_string()).await?;
            }

            client.connect().await;

            match client.send_event(event).await {
                Ok(event_id) => println!("Event published: {}", event_id.to_string()),
                Err(e) => eprintln!("Error publishing event: {}", e),
            }
        }
    }

    Ok(())
}
