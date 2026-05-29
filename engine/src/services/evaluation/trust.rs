use std::collections::HashSet;
use crate::domain::trust::TrustVector;

pub fn compute_trust(text: &str) -> TrustVector {
    let lines: Vec<&str> = text.lines().collect();
    let total_lines = lines.len().max(1) as f32;

    let mut empty_lines = 0.0;
    let mut repeated_lines = 0.0;

    let mut seen: HashSet<&str> = HashSet::new();

    let mut heading_count = 0.0;
    let mut list_count = 0.0;

    for line in &lines {
        let t = line.trim();

        if t.is_empty() {
            empty_lines += 1.0;
            continue;
        }

        if !seen.insert(t) {
            repeated_lines += 1.0;
        }

        if t.starts_with('#') {
            heading_count += 1.0;
        }

        if t.starts_with('-') || t.starts_with('*') {
            list_count += 1.0;
        }

        if t.chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && t.contains('.')
        {
            list_count += 0.5;
        }
    }

    let empty_ratio = empty_lines / total_lines;
    let repeat_ratio = repeated_lines / total_lines;

    // STRUCTURE
    let mut structure = ((heading_count * 0.6 + list_count * 0.4) / total_lines)
        .clamp(0.0, 1.0);

    // penalty for totally flat content
    if heading_count == 0.0 && list_count == 0.0 {
        structure *= 0.4;
    }

    // CLARITY
    let clarity = (1.0 - repeat_ratio - empty_ratio * 0.5)
        .clamp(0.0, 1.0);

    // COMPLETENESS
    let completeness = ((total_lines / 50.0).min(1.0) * 0.5 + structure * 0.5)
        .clamp(0.0, 1.0);

    // CONSISTENCY
    let consistency = ((1.0 - repeat_ratio) * 0.5 + structure * 0.5)
        .clamp(0.0, 1.0);

    TrustVector {
        structure,
        clarity,
        completeness,
        consistency,
    }
}