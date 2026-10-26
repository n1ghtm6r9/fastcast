use crate::settings::BackendChoice;

pub fn backend_choice_named(name: &str) -> Option<BackendChoice> {
    match name.trim().to_ascii_lowercase().as_str() {
        "auto" => Some(BackendChoice::Auto),
        "native" => Some(BackendChoice::Native),
        "ffmpeg" => Some(BackendChoice::Ffmpeg),
        _ => None,
    }
}
