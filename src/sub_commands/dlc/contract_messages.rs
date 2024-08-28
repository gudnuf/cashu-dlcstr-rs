use std::str::FromStr;

use cdk::secp256k1::{schnorr::Signature, XOnlyPublicKey};
use dlc_messages::{
    contract_msgs::{
        ContractDescriptor, ContractInfoInner, ContractOutcome, EnumeratedContractDescriptor,
    },
    oracle_msgs::{
        EnumEventDescriptor, EventDescriptor, OracleAnnouncement, OracleAttestation, OracleEvent,
        OracleInfo, SingleOracleInfo,
    },
};
use lightning::util::ser::Writeable;

pub struct GlobalContractOffer {
    info: ContractInfoInner,
}

pub struct GlobalAttestation {
    info: OracleAttestation,
}

impl GlobalContractOffer {
    pub fn new() -> Self {
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
            contract_descriptor: ContractDescriptor::EnumeratedContractDescriptor(
                contract_descriptor,
            ),
            oracle_info: OracleInfo::Single(oracle_info),
        };

        GlobalContractOffer {
            info: contract_info,
        }
    }

    pub fn encode_info(&self) -> Vec<u8> {
        self.info.encode()
    }
}

impl GlobalAttestation {
    pub fn new() -> Self {
        let attestation = OracleAttestation {
            oracle_public_key: XOnlyPublicKey::from_str(
                &"cdb88b7c62462984a7f1cfcfa5d49ad1a46fb6c3bdc8e9c6fc81e14a5f68f227",
            )
            .expect("invalid oracle public key"),
            signatures: vec![Signature::from_slice(&[0; 64]).unwrap()],
            outcomes: vec![String::from("outcome 1")],
        };

        GlobalAttestation { info: attestation }
    }

    pub fn encode_info(&self) -> Vec<u8> {
        self.info.encode()
    }
}
