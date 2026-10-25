use crate::display::{
    LAYOUT_RESTORE_TIMEOUT, POLL_INTERVAL, SavedDisplay, apply_staged_display_settings,
    current_placements, layout_is_back, poll_until, stage_display_settings,
};
use std::convert::Infallible;
use tracing::{info, warn};
use windows::Win32::Graphics::Gdi::{CDS_SET_PRIMARY, CDS_TYPE};

pub fn restore_displays(saved: &[SavedDisplay]) {
    let placements: Vec<_> = saved
        .iter()
        .map(|monitor| monitor.placement.clone())
        .collect();
    let came_back = poll_until(LAYOUT_RESTORE_TIMEOUT, POLL_INTERVAL, || {
        Ok::<_, Infallible>(layout_is_back(&placements, &current_placements()).then_some(()))
    }) == Ok(Some(()));

    for monitor in saved {
        let flags = if monitor.placement.is_primary() {
            CDS_SET_PRIMARY
        } else {
            CDS_TYPE(0)
        };

        if let Err(code) = stage_display_settings(&monitor.placement.name, &monitor.settings, flags)
        {
            warn!(
                "could not turn {} back on (display change code {})",
                monitor.placement.name, code.0
            );
        }
    }

    if came_back {
        info!("the monitors are back on");
        return;
    }

    match apply_staged_display_settings() {
        Ok(()) => info!("turned the monitors back on"),
        Err(code) => warn!(
            "could not turn the monitors back on (display change code {})",
            code.0
        ),
    }
}
