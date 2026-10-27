use crate::display::{DisplayError, DisplaySource, active_display_paths, source_device_name};

pub fn display_source_named(device_name: &str) -> Result<Option<DisplaySource>, DisplayError> {
    for path in active_display_paths()? {
        let source = DisplaySource {
            adapter: path.sourceInfo.adapterId,
            id: path.sourceInfo.id,
        };

        if source_device_name(source)?.eq_ignore_ascii_case(device_name) {
            return Ok(Some(source));
        }
    }

    Ok(None)
}
