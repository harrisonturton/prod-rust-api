use rand::distributions::Alphanumeric;
use rand::Rng;

/// Generate a random 11-chararacter user ID.
pub fn generate_user_id() -> String {
    let prefix = "U".to_owned();
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("{}{}", prefix, suffix)
}

// 25 June – 9 July
// Two solid weeks
// Paxton
