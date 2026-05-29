use crate::domain::submission::Submission;
use crate::domain::result::ValidationResult;
use crate::services::governance::{classify, GovernanceDecision};
use crate::services::validation::validate::validate_submission;

use crate::services::evaluation::quality::content_quality;
use crate::services::evaluation::trust::compute_trust;
use crate::services::review::perform_review;

pub fn run_pipeline(
    submission: &Submission,
    existing: &[Submission],
) -> anyhow::Result<Option<GovernanceDecision>> {

    // STEP 1: VALIDATION
    let validation = validate_submission(submission, existing)?;

    match validation {
        ValidationResult::Rejected(reason) => {
            println!("VALIDATION REJECTED: {}", reason);
            return Ok(None);
        }
        ValidationResult::Accepted => {
            println!("VALIDATION: Accepted");
        }
    }

    // STEP 2: QUALITY
    let quality = content_quality(&submission.content);
    println!("QUALITY: {:.2}", quality);

    // STEP 3: TRUST
    let trust = compute_trust(&submission.content);

    println!(
        "TRUST:\n    Structure: {:.2}\n    Clarity: {:.2}\n    Completeness: {:.2}\n    Consistency: {:.2}",
        trust.structure,
        trust.clarity,
        trust.completeness,
        trust.consistency
    );

    // STEP 4: REVIEW
    let review = perform_review(&submission.content);
    println!("REVIEW: {:?}", review);

    // STEP 5: GOVERNANCE
    let decision = classify(submission);

    println!("GOVERNANCE DECISION: {:?}", decision);

    Ok(Some(decision))
}