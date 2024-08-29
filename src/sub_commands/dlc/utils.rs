use dlc_messages::oracle_msgs::OracleAnnouncement;
use lightning::util::ser::Readable;
use nostr::base64;
use nostr::prelude::hex::decode;
use std::io::Cursor;

fn decode_bytes(str: &str) -> Result<Vec<u8>, base64::DecodeError> {
    // match FromHex::from_hex(str) {
    //     Ok(bytes) => Ok(bytes),
    //     Err(_) => Ok(base64::decode(str)?),
    // }
    base64::decode(str)
}

/// Parses a string into an oracle announcement.
pub(crate) fn oracle_announcement_from_str(str: &str) -> OracleAnnouncement {
    let bytes = decode_bytes(str).unwrap();
    let mut cursor = Cursor::new(bytes);

    OracleAnnouncement::read(&mut cursor).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANNOUNCEMENT: &str = "ypyyyX6pdZUM+OovHftxK9StImd8F7nxmr/eTeyR/5koOVVe/EaNw1MAeJm8LKDV1w74Fr+UJ+83bVP3ynNmjwKbtJr9eP5ie2Exmeod7kw4uNsuXcw6tqJF1FXH3fTF/dgiOwAByEOAEd95715DKrSLVdN/7cGtOlSRTQ0/LsW/p3BiVOdlpccA/dgGDAACBDEyMzQENDU2NwR0ZXN0";

    #[test]
    fn test_decode_oracle_announcement() {
        let announcement = oracle_announcement_from_str(ANNOUNCEMENT);
        println!("{:?}", announcement);
    }
}
