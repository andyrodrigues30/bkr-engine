#[derive(Debug)]
pub enum ValidationResult {
    Accepted,
    Rejected(String),
}