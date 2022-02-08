use crate::base::time::Timestamp;

#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: String,
    pub token: String,
    pub created_at: Timestamp,
}
