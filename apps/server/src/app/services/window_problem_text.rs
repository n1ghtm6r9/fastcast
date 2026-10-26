use std::fmt::Display;
use std::path::Path;

pub fn window_problem_text(problem: impl Display, journal_file: &Path) -> String {
    format!(
        "fastcast не смог открыть окно: {problem}\n\nПодробности в журнале: {}",
        journal_file.display()
    )
}
