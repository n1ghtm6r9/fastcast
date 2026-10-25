use crate::capture::forward_stderr;

#[tokio::test]
async fn forwarding_stderr_ends_when_the_pipe_closes_and_survives_invalid_utf8() {
    let stderr: &[u8] = b"first warning\r\n\xff\xfe broken\n\n   \nlast line without a newline";

    forward_stderr(stderr).await;
}
