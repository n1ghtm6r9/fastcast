use crate::support::TestViewer;
use wire::Welcome;

impl TestViewer {
    pub fn welcome(&self) -> Welcome {
        self.welcome
    }
}
