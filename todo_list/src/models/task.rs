use super::auditable::Auditable;

pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub audit: Auditable,
}