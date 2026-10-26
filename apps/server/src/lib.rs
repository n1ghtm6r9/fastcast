use mimalloc::MiMalloc;

#[global_allocator]
static THE_ALLOCATOR_THE_WHOLE_APP_RUNS_ON: MiMalloc = MiMalloc;

pub mod app;
pub mod capture;
pub mod display;
pub mod hub;
pub mod journal;
pub mod settings;
pub mod stream;

#[cfg(test)]
mod fixtures;
