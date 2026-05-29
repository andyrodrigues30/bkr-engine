pub fn content_quality(text: &str) -> f32 {
    let lines: Vec<&str> = text.lines().collect();
    let total_lines = lines.len().max(1) as f32;

    let mut heading_count = 0.0;
    let mut bullet_count = 0.0;
    let mut numbered_count = 0.0;
    let mut empty_lines = 0.0;

    for line in &lines {
        let t = line.trim();

        if t.is_empty() {
            empty_lines += 1.0;
            continue;
        }

        // headings
        if t.starts_with('#') {
            heading_count += 1.0;
        }

        // bullet points
        if t.starts_with('-') || t.starts_with('*') {
            bullet_count += 1.0;
        }

        // numbered lists
        if t.chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && t.contains('.')
        {
            numbered_count += 1.0;
        }
    }

    let heading_ratio = heading_count / total_lines;
    let list_ratio = (bullet_count + numbered_count) / total_lines;
    let empty_ratio = empty_lines / total_lines;

    // stronger structure signal
    let mut structure = (heading_ratio * 0.6 + list_ratio * 0.4).clamp(0.0, 1.0);

    // penalise completely flat text (no structure at all)
    if heading_count == 0.0 && bullet_count == 0.0 && numbered_count == 0.0 {
        structure *= 0.4;
    }

    // penalise empty / poorly formatted text
    let cleanliness = (1.0 - empty_ratio * 0.7).clamp(0.0, 1.0);

    // final score
    (structure * 0.8 + cleanliness * 0.2).clamp(0.0, 1.0)
}