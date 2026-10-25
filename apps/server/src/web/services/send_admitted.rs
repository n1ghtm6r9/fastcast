use crate::hub::Frame;
use crate::web::{Admission, ViewerCursor, ViewerSink, send_config, send_frame};

pub async fn send_admitted(
    sink: &ViewerSink,
    cursor: &mut ViewerCursor,
    frame: &Frame,
) -> Result<(), axum::Error> {
    match cursor.admit(frame) {
        Admission::Skip => Ok(()),
        Admission::Send => send_frame(sink, frame).await,
        Admission::SendWithConfig(config) => {
            send_config(sink, &config).await?;
            send_frame(sink, frame).await
        }
    }
}
