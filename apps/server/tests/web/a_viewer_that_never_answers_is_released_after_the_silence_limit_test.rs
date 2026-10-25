use crate::support::{
    connect_viewer, hub_with_nvenc_config, quick_timings, start_server_with_timings,
    wait_for_viewers,
};

#[tokio::test]
async fn a_viewer_that_never_answers_is_released_after_the_silence_limit() {
    let hub = hub_with_nvenc_config();
    let server = start_server_with_timings(hub.clone(), quick_timings()).await;
    let _viewer = connect_viewer(server.address).await;

    wait_for_viewers(&hub, 1).await;
    wait_for_viewers(&hub, 0).await;
}
