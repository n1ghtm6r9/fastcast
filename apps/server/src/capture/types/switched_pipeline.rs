use crate::hub::ViewerRequest;

pub enum SwitchedPipeline<N, F> {
    Native { pipeline: N, request: ViewerRequest },
    Ffmpeg(F),
}
