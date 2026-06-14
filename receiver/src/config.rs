/// Receiver configuration from the environment. Fail-closed: a release build
/// MUST have a public key (mirrors the backend's prior posture).
#[derive(Clone)]
pub struct ReceiverConfig {
    pub public_key: String,
    pub backend_internal_url: String,
    pub internal_token: String,
    pub bind_addr: String,
}

impl ReceiverConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let public_key = std::env::var("DISCORD_PUBLIC_KEY")
            .map_err(|_| anyhow::anyhow!("DISCORD_PUBLIC_KEY is required"))?;
        validate_public_key_hex(&public_key)?;
        Ok(Self {
            public_key,
            backend_internal_url: std::env::var("BACKEND_INTERNAL_URL")
                .unwrap_or_else(|_| "http://backend:3001".to_string()),
            internal_token: std::env::var("INTERNAL_TOKEN")
                .map_err(|_| anyhow::anyhow!("INTERNAL_TOKEN is required"))?,
            bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3002".to_string()),
        })
    }
}

/// Validate that a Discord public key is 64 hex chars decoding to exactly 32
/// bytes. Fail-closed: a bad key must stop startup, never disable verification.
fn validate_public_key_hex(public_key: &str) -> anyhow::Result<()> {
    let mut buf = [0u8; 32];
    hex::decode_to_slice(public_key, &mut buf)
        .map_err(|_| anyhow::anyhow!("DISCORD_PUBLIC_KEY must be 64 hex chars (32 bytes)"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_32_byte_hex() {
        // 64 hex chars = 32 bytes.
        assert!(validate_public_key_hex(&"ab".repeat(32)).is_ok());
    }

    #[test]
    fn rejects_malformed_or_wrong_length() {
        assert!(validate_public_key_hex("nothex").is_err());
        assert!(validate_public_key_hex(&"ab".repeat(16)).is_err()); // 16 bytes, too short
        assert!(validate_public_key_hex(&"ab".repeat(33)).is_err()); // 33 bytes, too long
        assert!(validate_public_key_hex("").is_err());
    }
}
