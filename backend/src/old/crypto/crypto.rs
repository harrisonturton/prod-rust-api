use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

static PBKDF2_ALGORITHM: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
// Cannot use the safer `NonZeroU32::new()` because it returns an `Option`.
// Minimum PBKDF2 iteration count of 1000 is recommended by RFC 2898 (2000).
// See also https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-132.pdf.
const PBKDF2_ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10_000) };
const PBKDF2_HASH_LENGTH: usize = digest::SHA256_OUTPUT_LEN;
const SALT_LENGTH: usize = digest::SHA256_OUTPUT_LEN;

const PBKDF2_HASH_LENGTH_CHARS: usize = 44;
const SALT_LENGTH_CHARS: usize = 44;

pub const HASH_LENGTH_CHARS: usize = PBKDF2_HASH_LENGTH_CHARS + SALT_LENGTH_CHARS;

pub fn create_token() -> Option<String> {
    let mut token = [0u8; 256];
    SystemRandom::new().fill(&mut token).ok()?;
    Some(base64::encode(token))
}

#[derive(PartialEq, Clone, Debug)]
pub struct Hash {
    pub hash: [u8; PBKDF2_HASH_LENGTH],
    pub salt: [u8; SALT_LENGTH],
}

impl Hash {
    pub fn from_base64_string(value: String) -> Option<Hash> {
       if value.len() != HASH_LENGTH_CHARS {
           return None;
       }
       let base64_hash = &value[..PBKDF2_HASH_LENGTH_CHARS];
       let base64_salt = &value[PBKDF2_HASH_LENGTH_CHARS..];
       let hash: [u8; PBKDF2_HASH_LENGTH] = base64::decode(base64_hash).ok()?.try_into().ok()?;
       let salt: [u8; SALT_LENGTH] = base64::decode(base64_salt).ok()?.try_into().ok()?;
       Some(Hash { hash, salt })
    }

    pub fn to_base64_string(&self) -> String {
        let hash = base64::encode(self.hash);
        let salt = base64::encode(self.salt);
        format!("{}{}", hash, salt)
    }
}

pub fn hash_password(password: &str) -> Option<Hash> {
    let mut salt = [0u8; SALT_LENGTH];
    SystemRandom::new().fill(&mut salt).ok()?;
    let mut hash = [0u8; PBKDF2_HASH_LENGTH];
    pbkdf2::derive(
        PBKDF2_ALGORITHM,
        PBKDF2_ITERATIONS,
        &salt,
        password.as_bytes(),
        &mut hash,
    );
    Some(Hash { hash, salt })
}

pub fn check_password(attempted_password: &String, hash: &Hash) -> bool {
    let result = pbkdf2::verify(
        PBKDF2_ALGORITHM,
        PBKDF2_ITERATIONS,
        &hash.salt,
        attempted_password.as_bytes(),
        &hash.hash,
    );
    result.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_to_and_from_base64_string_works() {
        let password = String::from("password");
        let original_hash = hash_password(&password).unwrap();
        let base64_string = original_hash.to_base64_string();

        let derived_hash = Hash::from_base64_string(base64_string).unwrap();
        assert_eq!(derived_hash, original_hash);
    }

    #[test]
    fn test_check_password_passes_for_correct_password() {
        let password = String::from("password");
        let hash = hash_password(&password).unwrap();

        let attempt = String::from("password");
        let is_ok = check_password(&attempt, &hash);
        assert!(is_ok);
    }

    #[test]
    fn test_check_password_rejects_incorrect_password() {
        let password = String::from("password");
        let hash = hash_password(&password).unwrap();

        let attempt = String::from("bad password");
        let is_ok = check_password(&attempt, &hash);
        assert!(!is_ok);
    }

    #[test]
    fn test_hash_base64_string_length_is_88() {
        let password = String::from("password");
        let hash = hash_password(&password).unwrap();
        let hash_string = hash.to_base64_string();
        assert_eq!(hash_string.len(), 88);
    }
}