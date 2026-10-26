use crate::window::Monitor;

pub fn monitor_text(index: u32, monitors: &[Monitor]) -> String {
    match monitors.iter().find(|monitor| monitor.index == index) {
        Some(monitor) => format!(
            "{index}: {}, {}×{}",
            monitor.name, monitor.width, monitor.height
        ),
        None => format!("{index}: не подключён"),
    }
}
