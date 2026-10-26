use crate::capture::Picture;

impl Picture {
    pub fn arrived(&mut self) {
        *self = Picture::Waiting;
    }
}
