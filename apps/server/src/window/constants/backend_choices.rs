use crate::settings::BackendChoice;

pub const BACKEND_CHOICES: [(BackendChoice, &str); 3] = [
    (BackendChoice::Auto, "Авто"),
    (BackendChoice::Native, "Только свой"),
    (BackendChoice::Ffmpeg, "Только ffmpeg"),
];
