use crate::capture::keyframe_interval_in_frames;
use crate::fixtures::hub_with_config;
use crate::settings::constants::{CAPTURE_FPS_SETTING, KEYFRAME_INTERVAL_SECONDS_SETTING};
use crate::web::DISPLAY_RATE_RANGE;

#[test]
fn the_longest_allowed_keyframe_interval_at_the_highest_allowed_rate_is_kept_as_one_gop() {
    let highest_rate = CAPTURE_FPS_SETTING.max.max(*DISPLAY_RATE_RANGE.end());
    let interval =
        keyframe_interval_in_frames(highest_rate, KEYFRAME_INTERVAL_SECONDS_SETTING.max) as usize;
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    for index in 1..interval {
        hub.publish(false, index as u32, &[2]);
    }

    assert_eq!(hub.join().gop.len(), interval);
}
