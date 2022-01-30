use rand::distributions::Alphanumeric;
use rand::Rng;

/// Generate a random 11-chararacter ID.
pub fn generate_id(prefix: char) -> String {
    let prefix = String::from(prefix);
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("{}{}", prefix, suffix)
}
