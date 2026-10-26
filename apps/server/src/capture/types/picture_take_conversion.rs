use crate::capture::Picture;

impl Picture {
    pub fn take_conversion(&mut self) -> bool {
        let waiting = *self == Picture::Waiting;

        if waiting {
            *self = Picture::Converted;
        }

        waiting
    }
}
