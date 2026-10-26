use crate::server::{ServerState, ServerStatus};

pub fn status_headline(status: &ServerStatus) -> String {
    match (&status.state, status.viewers) {
        (ServerState::Starting, _) => "Запускается…".to_owned(),
        (ServerState::Running { .. }, 0) => "Ждёт ТВ".to_owned(),
        (ServerState::Running { .. }, 1) => "Транслирует на ТВ".to_owned(),
        (ServerState::Running { .. }, viewers) => format!("Транслирует на {viewers} ТВ"),
        (ServerState::Failed { .. }, _) => "Не запустился".to_owned(),
        (ServerState::Stopped, _) => "Остановлен".to_owned(),
    }
}
