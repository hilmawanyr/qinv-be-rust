use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::modules::shared::encrypt::{EncryptError, EncryptPort};

pub struct InternalEncryptAdapter {
    duration: u64,
    client_id: Vec<u8>,
    secret: Vec<u8>,
}

impl InternalEncryptAdapter {
    pub fn new(duration: u64, client_id: String, secret: String) -> Result<Self, EncryptError> {
        if duration == 0 || client_id.is_empty() || secret.is_empty() {
            return Err(EncryptError::InvalidConfig);
        }

        Ok(Self {
            duration,
            client_id: client_id.into_bytes(),
            secret: secret.into_bytes(),
        })
    }

    #[inline]
    fn now() -> Result<u64, EncryptError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .map_err(|_| EncryptError::EncryptionFailed)
    }

    /// Encrypt bytes using a repeating key.
    #[inline]
    fn encode(input: &[u8], key: &[u8]) -> Vec<u8> {
        input
            .iter()
            .enumerate()
            .map(|(i, byte)| byte.wrapping_add(key[i % key.len()]))
            .collect()
    }

    /// Decrypt bytes using a repeating key.
    #[inline]
    fn decode(input: &[u8], key: &[u8]) -> Vec<u8> {
        input
            .iter()
            .enumerate()
            .map(|(i, byte)| byte.wrapping_sub(key[i % key.len()]))
            .collect()
    }

    #[inline]
    fn double_encode(&self, input: &[u8]) -> Vec<u8> {
        let first = Self::encode(input, &self.client_id);

        Self::encode(&first, &self.secret)
    }

    #[inline]
    fn double_decode(&self, input: &[u8]) -> Vec<u8> {
        let first = Self::decode(input, &self.secret);

        Self::decode(&first, &self.client_id)
    }

    #[inline]
    fn is_valid_timestamp(&self, timestamp: u64) -> Result<bool, EncryptError> {
        let now = Self::now()?;

        Ok(timestamp.abs_diff(now) <= self.duration)
    }
}

impl EncryptPort for InternalEncryptAdapter {
    fn encrypt(&self, payload: &[u8]) -> Result<String, EncryptError> {
        // Reverse timestamp.
        let timestamp = Self::now()?.to_string().bytes().rev().collect::<Vec<_>>();

        // Build timestamp.payload
        let mut input = Vec::with_capacity(timestamp.len() + 1 + payload.len());

        input.extend_from_slice(&timestamp);
        input.push(b'.');
        input.extend_from_slice(payload);

        // Double encryption.
        let encrypted = self.double_encode(&input);

        // Base64 URL-safe without padding.
        Ok(URL_SAFE_NO_PAD.encode(encrypted))
    }

    fn decrypt(&self, value: &str) -> Result<Option<Vec<u8>>, EncryptError> {
        // Base64 decode.
        let decoded = URL_SAFE_NO_PAD
            .decode(value)
            .map_err(|_| EncryptError::InvalidPayload)?;

        // Double decryption.
        let decrypted = self.double_decode(&decoded);

        // Find timestamp separator.
        let Some(dot) = decrypted.iter().position(|&b| b == b'.') else {
            return Ok(None);
        };

        if dot == 0 {
            return Ok(None);
        }

        // Reverse timestamp and parse.
        let timestamp = decrypted[..dot]
            .iter()
            .rev()
            .try_fold(0u64, |value, &byte| {
                if byte.is_ascii_digit() {
                    value.checked_mul(10)?.checked_add(u64::from(byte - b'0'))
                } else {
                    None
                }
            });

        let Some(timestamp) = timestamp else {
            return Ok(None);
        };

        // Validate expiration.
        if !self.is_valid_timestamp(timestamp)? {
            return Ok(None);
        }

        // Return raw payload bytes.
        Ok(Some(decrypted[dot + 1..].to_vec()))
    }
}
