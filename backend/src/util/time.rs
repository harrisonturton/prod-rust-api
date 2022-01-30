// The `chrono` crate is feature-complete, but unmaintained. There is a good
// chance it will need to be replaced in the future. This module provides a
// facade over it's interface so it will be easier to swap out later.
use chrono::{DateTime, Utc};

pub type Timestamp = DateTime<Utc>;

pub fn now() -> Timestamp {
    Utc::now()
}
