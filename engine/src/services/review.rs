use crate::domain::review::ReviewDecision;

pub fn perform_review(content: &str) -> ReviewDecision {
    classify_review(content)
}

fn classify_review(content: &str) -> ReviewDecision {
    if content.contains("malware") {
        ReviewDecision::Reject
    } else if content.contains("bad") {
        ReviewDecision::Escalate
    } else if content.contains("mistake") {
        ReviewDecision::ApproveWithFeedback
    } else {
        ReviewDecision::Approve
    }
}