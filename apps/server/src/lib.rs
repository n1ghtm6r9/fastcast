use mimalloc::MiMalloc;

#[global_allocator]
static THE_ALLOCATOR_THE_WHOLE_APP_RUNS_ON: MiMalloc = MiMalloc;

pub mod app;
pub mod capture;
pub mod discovery;
pub mod display;
pub mod hub;
pub mod settings;
pub mod stream;
pub mod web;

#[cfg(test)]
mod fixtures;
