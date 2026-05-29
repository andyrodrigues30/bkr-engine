use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmissionState {
    Submitted,
    Validated,
    SafetyPassed,
    Classified,
    UnderReview,
    Approved,
    ApprovedWithFeedback,
    Rejected,
    Indexed,
}