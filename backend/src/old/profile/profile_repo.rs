use super::profile_model::User;
use postgres::Client;
use rocket_sync_db_pools::database;

#[database("editor_db")]
pub struct Db(Client);

pub struct ProfileRepo<'a> {
    conn: &'a Db,
}

impl<'a> ProfileRepo<'a> {
    pub fn new(conn: &'a Db) -> ProfileRepo {
        ProfileRepo { conn }
    }

    pub async fn get_user_by_id(&self, id: String) -> Option<User> {
        self.conn
            .run(move |conn| {
                let query = "SELECT id, email, hash FROM users WHERE id = $1 LIMIT 1";
                let rows = conn.query(query, &[&id]).ok()?;
                if rows.is_empty() {
                    return None;
                }
                let user_row = rows.get(0)?;
                let id: String = user_row.try_get(0).ok()?;
                let email: String = user_row.try_get(1).ok()?;
                let hash: String = user_row.try_get(2).ok()?;
                Some(User {
                    id,
                    email,
                    hash,
                })
            })
            .await
    }

    pub async fn get_user_by_email(&self, email: String) -> Option<User> {
        self.conn
            .run(move |conn| {
                let query = "SELECT id, email, hash FROM users WHERE email = $1 LIMIT 1";
                let rows = conn.query(query, &[&email]).ok()?;
                if rows.is_empty() {
                    return None;
                }
                let user_row = rows.get(0)?;
                let id: String = user_row.try_get(0).ok()?;
                let email: String = user_row.try_get(1).ok()?;
                let hash: String = user_row.try_get(2).ok()?;
                Some(User {
                    id,
                    email,
                    hash,
                })
            })
            .await
    }

    pub async fn create_user(&mut self, user: User) -> Option<()> {
        self.conn
            .run(move |conn| {
                println!("About to store user...");
                let query = "INSERT INTO users (id, email, hash) VALUES ($1, $2, $3)";
                let num_rows_modified = conn
                    .execute(query, &[&user.id, &user.email, &user.hash])
                    .ok()?;
                if num_rows_modified == 0 {
                    None
                } else {
                    Some(())
                }
            })
            .await
    }
}
