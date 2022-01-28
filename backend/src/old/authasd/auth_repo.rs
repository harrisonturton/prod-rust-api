use postgres::Client;
use rocket_sync_db_pools::database;
use crate::crypto::{Hash, HASH_LENGTH_CHARS};

#[database("editor_db")]
pub struct Db(Client);

pub struct AuthRepo<'a> {
    conn: &'a Db,
}

impl<'a> AuthRepo<'a> {
    pub fn new(conn: &'a Db) -> AuthRepo {
        AuthRepo { conn }
    }

    pub async fn get_password_hash_by_email(&self, email: String) -> Option<Hash> {
        self.conn.run(move |conn| {
            let query = "SELECT hash FROM users WHERE email = $1 LIMIT 1";
            let rows = conn.query(query, &[&email]).ok()?;
            if rows.is_empty() {
                println!("No hash found for email {}", email);
                return None;
            }
            let first_row = rows.get(0)?;
            // Postgres column isn't exactly sized to the hash length 
            let padded_base64_string: String = first_row.try_get(0).ok()?;
            let base64_string = padded_base64_string[..HASH_LENGTH_CHARS].to_string();
            Some(Hash::from_base64_string(base64_string))
        }).await?
    }

    pub async fn create_session(&self, user_id: String, hash: String) -> Option<()> {
        self.conn.run(move |conn| {
            let query = "INSERT INTO session (user_id, hash) VALUES ($1, $2)";
            let rows_affected = conn.execute(query, &[&user_id, &hash]).ok()?;
            if rows_affected == 0 {
                return None;
            }
            Some(())
        }).await
    }
}
