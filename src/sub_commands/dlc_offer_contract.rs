use std::str::FromStr;

use anyhow::Result;
use cdk::secp256k1::{schnorr::Signature, XOnlyPublicKey};
use dlc_messages::{
    contract_msgs::{
        ContractDescriptor, ContractInfoInner, ContractOutcome, EnumeratedContractDescriptor,
    },
    oracle_msgs::{
        EnumEventDescriptor, EventDescriptor, OracleAnnouncement, OracleEvent, OracleInfo,
        SingleOracleInfo,
    },
};
use lightning::util::ser::Writeable;
use nostr::TryIntoUrl;
use nostr_sdk::{
    base64, Client, Event, EventBuilder, Keys, Kind, Tag, TagKind, Timestamp, UncheckedUrl,
};

fn create_offer_event(
    relays: Vec<UncheckedUrl>,
    keys: &Keys,
) -> Result<Event, nostr_sdk::event::builder::Error> {
    let contract_descriptor = EnumeratedContractDescriptor {
        payouts: vec![ContractOutcome {
            outcome: String::from("dummy string"),
            offer_payout: 5,
        }],
    };

    let oracle_info = SingleOracleInfo {
        oracle_announcement: OracleAnnouncement {
            announcement_signature: Signature::from_slice(&[0; 64]).unwrap(),
            oracle_public_key: XOnlyPublicKey::from_str(
                &"cdb88b7c62462984a7f1cfcfa5d49ad1a46fb6c3bdc8e9c6fc81e14a5f68f227",
            )
            .expect("Invalid oracle public key"),
            oracle_event: OracleEvent {
                event_descriptor: EventDescriptor::EnumEvent(EnumEventDescriptor {
                    outcomes: Vec::new(),
                }),
                event_id: String::from("this should be the event id"),
                event_maturity_epoch: 0,
                oracle_nonces: Vec::new(),
            },
        },
    };

    let contract_info = ContractInfoInner {
        contract_descriptor: ContractDescriptor::EnumeratedContractDescriptor(contract_descriptor),
        oracle_info: OracleInfo::Single(oracle_info),
    };

    // TOOD: nip04 encrypt the contenct
    // TODO: figure out what the "offer ID" is for the d tag

    EventBuilder::new(
        Kind::Custom(30088),
        base64::encode(contract_info.encode()),
        vec![
            Tag::from_standardized(nostr_sdk::TagStandard::Relays(relays)),
            Tag::from_standardized(nostr_sdk::TagStandard::Expiration(Timestamp::now() + 60)),
            Tag::identifier("offers ID"),
        ],
    )
    .to_event(keys)
}

pub async fn create_offer() -> Result<()> {
    let my_keys =
        Keys::parse("nsec15jldh0htg2qeeqmqd628js8386fu4xwpnuqddacc64gh0ezdum6qaw574p").unwrap();
    let relays = vec!["wss://relay.damus.io".parse().unwrap()];

    let event = create_offer_event(relays.clone(), &my_keys).unwrap();

    let client = Client::new(&my_keys);
    for relay in relays {
        client
            .add_relay(relay.to_string().try_into_url().unwrap())
            .await?;
    }
    client.connect().await;

    match client.send_event(event).await {
        Ok(event_id) => println!("Event published: {}", event_id.to_string()),
        Err(e) => eprintln!("Error publishing event: {}", e),
    }

    Ok(())
}
