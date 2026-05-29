use anyhow::Result;
use crate::domain::submission::Submission;
use crate::services::validation::duplicate::is_exact_duplicate;
use crate::domain::result::ValidationResult;

pub fn validate_submission(
    submission: &Submission,
    existing: &[Submission],
) -> Result<ValidationResult> {

    // RULE 1: minimum content length
    if submission.content.trim().len() < 50 {
        return Ok(ValidationResult::Rejected(
            "Content too short".to_string()
        ));
    }

    // RULE 2: duplicate detection
    for existing_item in existing {
        if is_exact_duplicate(&submission.content, &existing_item.content) {
            return Ok(ValidationResult::Rejected(
                "Duplicate content".to_string()
            ));
        }
    }

    Ok(ValidationResult::Accepted)
}