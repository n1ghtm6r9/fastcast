use mimalloc::MiMalloc;

#[global_allocator]
static THE_ALLOCATOR_THE_WHOLE_APP_RUNS_ON: MiMalloc = MiMalloc;

pub mod app;
pub mod capture;
pub mod display;
pub mod hub;
pub mod journal;
pub mod server;
pub mod settings;
pub mod stream;
pub mod window;

#[cfg(test)]
mod fixtures;
