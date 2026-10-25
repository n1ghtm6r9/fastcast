#[cfg(windows)]
mod added_display;
#[cfg(windows)]
mod added_display_add;
#[cfg(windows)]
mod added_display_drop;
mod display_bounds;
mod display_bounds_right_edge;
mod display_device;
mod display_device_is_parsec;
mod display_error;
mod display_mode;
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
pub use display_bounds::*;
pub use display_device::*;
pub use display_error::*;
pub use display_mode::*;
#[cfg(windows)]
pub use driver_handle::*;
#[cfg(windows)]
pub use keep_alive::*;
pub use virtual_display::*;
