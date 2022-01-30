use ring::pbkdf2::{Algorithm, PBKDF2_HMAC_SHA256};
use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest::SHA256_OUTPUT_LEN, pbkdf2};
use std::num::NonZeroU32;

static ALGORITHM: Algorithm = PBKDF2_HMAC_SHA256;
// Cannot use the safer `NonZeroU32::new()` because it returns an `Option`.
// Minimum PBKDF2 iteration count of 1000 is recommended by RFC 2898.
// See https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-132.pdf.
const ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10_000) };
const HASH_LENGTH: usize = SHA256_OUTPUT_LEN;
const HASH_LENGTH_CHARS: usize = 44;

// Base64-encoded hash and salt.
#[derive(PartialEq, Clone, Debug)]
pub struct Hash(String, String);

impl Hash {
    #[allow(clippy::self_named_constructors)]
    pub fn hash(value: &str) -> Option<Hash> {
        let bytes = value.as_bytes();
        let mut salt = [0u8; HASH_LENGTH];
        let mut hash = [0u8; HASH_LENGTH];
        SystemRandom::new().fill(&mut salt).ok()?;
        pbkdf2::derive(ALGORITHM, ITERATIONS, &salt, bytes, &mut hash);
        let hash = base64::encode(hash);
        let salt = base64::encode(salt);
        Some(Hash(hash, salt))
    }

    pub fn deserialize(value: &str) -> Option<Hash> {
        // There is trailing whitespace in the database
        let value = value.trim();
        // Salt is same length as hash
        if value.trim().len() != HASH_LENGTH_CHARS * 2 {
            return None;
        }
        let hash = String::from(&value[..HASH_LENGTH_CHARS]);
        let salt = String::from(&value[HASH_LENGTH_CHARS..]);
        Some(Hash(hash, salt))
    }

    pub fn serialize(&self) -> String {
        let Hash(hash, salt) = self;
        format!("{}{}", hash, salt)
    }

    pub fn verify(hash: &Hash, value: &str) -> bool {
        let Hash(salt, hash) = hash;
        let hash = match base64::decode(hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };
        let salt = match base64::decode(salt) {
            Ok(salt) => salt,
            Err(_) => return false,
        };
        let result = pbkdf2::verify(ALGORITHM, ITERATIONS, &hash, value.as_bytes(), &salt);
        result.is_ok()
    }
}

pub fn generate_token() -> Option<String> {
    let mut token = [0u8; 256];
    SystemRandom::new().fill(&mut token).ok()?;
    Some(base64::encode(token))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_to_and_from_base64_string() {
        let hash_a = Hash::hash("password").unwrap();
        let serialized = hash_a.serialize();
        let hash_b = Hash::deserialize(&serialized).unwrap();
        assert_eq!(hash_a, hash_b)
    }

    #[test]
    fn test_verify_accepts_valid_value() {
        let hash = Hash::hash("password").unwrap();
        let is_ok = Hash::verify(&hash, "password");
        assert!(is_ok);
    }

    #[test]
    fn test_verify_rejects_invalid_value() {
        let hash = Hash::hash("password").unwrap();
        let is_ok = Hash::verify(&hash, "bad password");
        assert!(!is_ok);
    }

    #[test]
    fn test_deserialize_trims_input() {
        let raw_hash = "B2t559DUa2f5Z/CVhoEoPgcaZGCXP7IQXJUG4GUbceI=qCUHj/l014ZVwEm1OJYjMxa93Y5hkfB4WPTgakV/5fg=                                        ";
        let _ = Hash::deserialize(raw_hash).unwrap();
    }
}
