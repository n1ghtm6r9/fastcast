use crate::hub::Hub;

impl Hub {
    pub fn report(&self, text: String) {
        self.reports.send_replace(Some(text));
    }
}
