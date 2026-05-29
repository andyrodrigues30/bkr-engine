mod domain;
mod services;
mod workflow;

use uuid::Uuid;
use walkdir::WalkDir;
use std::fs;

use domain::submission::Submission;
use workflow::pipeline::run_pipeline;

fn extract_title_and_content(raw: &str) -> (Option<String>, String) {
    let mut title: Option<String> = None;
    let mut content_lines: Vec<&str> = vec![];

    for line in raw.lines() {
        let trimmed = line.trim();

        // first "# " becomes title
        if title.is_none() && trimmed.starts_with("# ") {
            title = Some(trimmed.trim_start_matches("# ").to_string());
            continue;
        }

        content_lines.push(line);
    }

    (title, content_lines.join("\n"))
}

fn load_submissions(path: &str) -> Vec<Submission> {
    let mut submissions = vec![];

    if !std::path::Path::new(path).exists() {
        panic!("Directory does not exist: {}", path);
    }

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) // IMPORTANT: avoid directories
    {
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {

            let raw = fs::read_to_string(path)
                .expect("Failed to read markdown file");

            let (title, content) = extract_title_and_content(&raw);

            submissions.push(Submission {
                id: Uuid::new_v4(),
                title,
                content,
            });
        }
    }

    submissions
}

fn main() {
    // 1. Load existing knowledge base
    let existing = load_submissions("./test_data/existing");

    println!("Loaded {} existing guides", existing.len());

    // 2. Load new submissions to evaluate
    let new_submissions = load_submissions("./test_data/new");

    println!("Loaded {} new submissions", new_submissions.len());

    // 3. Process each new submission against existing ones
    for submission in &new_submissions {
        println!("\n==============================");
        println!("SUBMISSION ID: {:?}", submission.id);
        println!(
            "TITLE: {}",
            submission.title.as_deref().unwrap_or("Untitled")
        );
        
        if let Err(e) = run_pipeline(submission, &existing) {
            println!("PIPELINE ERROR: {}", e);
        }
    }
}