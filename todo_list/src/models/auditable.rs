use chrono::{DateTime, Utc};

pub struct Auditable {
    pub creation_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub last_updated_date: DateTime<Utc>,
}