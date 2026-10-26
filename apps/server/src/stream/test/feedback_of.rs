use wire::Feedback;

pub fn feedback_of(highest_frame: u32, keyframe_needed: bool) -> Feedback {
    Feedback {
        highest_frame,
        keyframe_needed,
        ..Feedback::default()
    }
}
