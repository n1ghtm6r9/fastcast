use crate::capture::Picture;

impl Picture {
    pub fn exists(self) -> bool {
        self != Picture::Missing
    }
}
