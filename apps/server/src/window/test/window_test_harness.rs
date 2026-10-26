use crate::window::ServerWindow;
use egui_kittest::Harness;

pub type WindowTestHarness = Harness<'static, ServerWindow<'static>>;
