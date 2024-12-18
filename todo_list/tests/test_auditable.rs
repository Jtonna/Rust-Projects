use std::io::stdout;
use chrono::Utc;
use todo_list::models::auditable::Auditable;

// Test creating Auditable
#[test]
fn test_auditable_constructor() {
    let creation_date = Utc::now();
    let completion_date = None;
    let last_updated_date = Utc::now();

    let auditable = Auditable::with_values(creation_date, completion_date, last_updated_date);

}