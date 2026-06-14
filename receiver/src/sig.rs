use chrono::Utc;
use ed25519_dalek::{Signature, VerifyingKey};

/// Verify Discord's Ed25519 signature over `timestamp || body`.
pub fn verify_discord_signature(
    public_key_hex: &str,
    signature_hex: &str,
    timestamp: &str,
    body: &[u8],
) -> bool {
    let mut pk_bytes = [0u8; 32];
    if hex::decode_to_slice(public_key_hex, &mut pk_bytes).is_err() {
        return false;
    }
    let mut sig_bytes = [0u8; 64];
    if hex::decode_to_slice(signature_hex, &mut sig_bytes).is_err() {
        return false;
    }
    let verifying_key = match VerifyingKey::from_bytes(&pk_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let signature = Signature::from_bytes(&sig_bytes);
    let mut message = timestamp.as_bytes().to_vec();
    message.extend_from_slice(body);
    verifying_key.verify_strict(&message, &signature).is_ok()
}

/// True if `timestamp` (unix seconds, as a string) is within ±`window_secs`
/// of now. Mirrors Discord's recommended replay tolerance.
pub fn timestamp_within_window(timestamp: &str, window_secs: i64) -> bool {
    match timestamp.parse::<i64>() {
        Ok(ts) => (Utc::now().timestamp() - ts).abs() <= window_secs,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    const SEED: [u8; 32] = [
        0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c,
        0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae,
        0x7f, 0x60,
    ];

    fn sign(body: &[u8], ts: &str) -> (String, String) {
        let key = SigningKey::from_bytes(&SEED);
        let pk = hex::encode(key.verifying_key().to_bytes());
        let mut msg = ts.as_bytes().to_vec();
        msg.extend_from_slice(body);
        (pk, hex::encode(key.sign(&msg).to_bytes()))
    }

    #[test]
    fn accepts_valid() {
        let (pk, sig) = sign(b"{\"x\":1}", "1717286400");
        assert!(verify_discord_signature(
            &pk,
            &sig,
            "1717286400",
            b"{\"x\":1}"
        ));
    }

    #[test]
    fn rejects_tampered_body() {
        let (pk, sig) = sign(b"{\"x\":1}", "1717286400");
        assert!(!verify_discord_signature(
            &pk,
            &sig,
            "1717286400",
            b"{\"x\":2}"
        ));
    }

    #[test]
    fn rejects_malformed_hex() {
        assert!(!verify_discord_signature("nothex", "00", "1", b"b"));
        assert!(!verify_discord_signature("00", "00", "1", b"b"));
    }

    #[test]
    fn replay_window_rejects_far_timestamp() {
        assert!(!timestamp_within_window("100", 300));
        assert!(!timestamp_within_window("not-a-number", 300));
    }
}
