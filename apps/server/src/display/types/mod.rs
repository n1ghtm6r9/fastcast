#[cfg(windows)]
mod added_display;
#[cfg(windows)]
mod added_display_add;
#[cfg(windows)]
mod added_display_drop;
#[cfg(windows)]
mod desktop_takeover;
#[cfg(windows)]
mod desktop_takeover_drop;
mod display_bounds;
mod display_device;
mod display_device_is_parsec;
mod display_error;
mod display_mode;
mod display_placement;
mod display_placement_is_primary;
#[cfg(windows)]
mod driver_handle;
#[cfg(windows)]
mod driver_handle_open;
#[cfg(windows)]
mod driver_handle_request;
#[cfg(windows)]
mod keep_alive;
#[cfg(windows)]
mod keep_alive_drop;
#[cfg(windows)]
mod keep_alive_start;
#[cfg(windows)]
mod saved_display;
mod virtual_display;
mod virtual_display_device_name;
mod virtual_display_drop;
mod virtual_display_mode;
#[cfg(windows)]
mod virtual_display_open;
#[cfg(not(windows))]
mod virtual_display_open_unsupported;
mod virtual_display_output_index;
#[cfg(windows)]
pub use added_display::*;
#[cfg(windows)]
pub use desktop_takeover::*;
pub use display_bounds::*;
pub use display_device::*;
pub use display_error::*;
pub use display_mode::*;
pub use display_placement::*;
#[cfg(windows)]
pub use driver_handle::*;
#[cfg(windows)]
pub use keep_alive::*;
#[cfg(windows)]
pub use saved_display::*;
pub use virtual_display::*;
