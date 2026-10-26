use crate::settings::SavedSettings;

#[test]
fn an_unknown_encoder_name_in_the_saved_file_is_refused() {
    let refused = serde_json::from_str::<SavedSettings>(r#"{"video_encoder":"vaapi"}"#)
        .expect_err("an unknown encoder is refused");

    assert!(refused.to_string().contains("vaapi"));
}
