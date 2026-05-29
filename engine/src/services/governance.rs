use crate::domain::submission::Submission;

#[derive(Debug)]
pub enum GovernanceDecision {
    NewGuide,
    Amendment,
    Variant,
    Reject,
}

pub fn classify(submission: &Submission) -> GovernanceDecision {

    let has_title = submission.title.is_some();
    let content_len = submission.content.len();

    // RULE 1: Amendment (no title)
    if !has_title {
        return GovernanceDecision::Amendment;
    }

    // RULE 2: too short
    if content_len < 80 {
        return GovernanceDecision::Reject;
    }

    // RULE 3: variant heuristic
    if let Some(title) = &submission.title {
        if title.to_lowercase().contains("variant") {
            return GovernanceDecision::Variant;
        }
    }

    GovernanceDecision::NewGuide
}