use uuid::Uuid;

#[derive(Debug)]
pub struct Submission {
    pub id: Uuid,
    pub title: Option<String>,
    pub content: String,
}