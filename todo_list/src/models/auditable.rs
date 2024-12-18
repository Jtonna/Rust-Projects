use chrono::{DateTime, Utc};
use std::option::Option;


pub struct Auditable {
    pub creation_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub last_updated_date: DateTime<Utc>,
}

impl Auditable {

    // Constructor
    pub fn with_values(creation_date: DateTime<Utc>, completion_date: Option<DateTime<Utc>>, last_updated_date: DateTime<Utc>) -> Self {
        let readable_date = last_updated_date.to_rfc3339();
        Auditable {
            creation_date: creation_date,
            completion_date: completion_date,
            last_updated_date: last_updated_date
        }
    }
    pub fn new() -> Self {
        let now = Utc::now();
        Auditable {
            creation_date: now,
            completion_date: None,
            last_updated_date: now
        }
    }

    pub fn update_last_updated_date(&mut self) {
        self.last_updated_date = Utc::now();
    }

    pub fn mark_completed(&mut self) {
        self.completion_date = Some(Utc::now());
        self.update_last_updated_date();
    }

    pub fn mark_incomplete(&mut self) {
        self.completion_date = None;
        self.update_last_updated_date();
    }
}