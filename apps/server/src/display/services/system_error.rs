use crate::display::DisplayError;
use std::io;

pub fn system_error<E: Into<io::Error>>(call: &'static str) -> impl FnOnce(E) -> DisplayError {
    move |source| DisplayError::System {
        call,
        source: source.into(),
    }
}
