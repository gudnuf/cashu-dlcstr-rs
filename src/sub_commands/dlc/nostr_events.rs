use crate::sub_commands::dlc::contract_messages::GlobalContractOffer;
use nostr_sdk::event::builder::Error;
use nostr_sdk::{base64, Event, EventBuilder, Keys, Kind, Tag, TagStandard, Timestamp};

use super::contract_messages::GlobalAttestation;

pub fn create_offer_event(
    global_contract_offer: &GlobalContractOffer,
    relays: &[String],
    keys: &Keys,
) -> Result<Event, Error> {
    let relays = relays.iter().map(|relay| relay.into()).collect::<Vec<_>>();

    // TOOD: nip04 encrypt the contenct
    // TODO: figure out what the "offer ID" is for the d tag

    EventBuilder::new(
        Kind::Custom(30088),
        base64::encode(global_contract_offer.encode_info()),
        vec![
            Tag::from_standardized(TagStandard::Relays(relays)),
            Tag::from_standardized(TagStandard::Expiration(Timestamp::now() + 60)), // TODO: what does this mean?
            Tag::identifier("offers ID"),                                           // TODO
        ],
    )
    .to_event(keys)
}

pub fn create_attestation_event(
    global_attestation: GlobalAttestation,
    relays: &[String],
    keys: &Keys,
) -> Result<Event, Error> {
    let relays = relays.iter().map(|relay| relay.into()).collect::<Vec<_>>();

    EventBuilder::new(
        Kind::Custom(30089),
        base64::encode(global_attestation.encode_info()),
        vec![
            Tag::from_standardized(TagStandard::Relays(relays)),
            Tag::from_standardized(TagStandard::Expiration(Timestamp::now() + 60)), // TODO: what does this mean?
            Tag::identifier("offers ID"),                                           // TODO
        ],
    )
    .to_event(keys)
}
