use todo_list::models::auditable::Auditable;

#[test]
fn test_auditable_creation() {
    let auditable = Auditable::new();
    assert!(auditable.creation_date <= chrono::Utc::now());
    assert_eq!(auditable.completion_date, None);
}