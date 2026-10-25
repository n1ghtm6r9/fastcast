use crate::display::{AddedDisplay, SavedDisplay};

pub struct DesktopTakeover {
    pub(crate) added: Option<AddedDisplay>,
    pub(crate) saved: Vec<SavedDisplay>,
}
