use crate::server::ServerState;

pub fn state_text(state: &ServerState) -> String {
    match state {
        ServerState::Starting => "Сервер запускается…".to_owned(),
        ServerState::Running { address } => format!("Сервер работает на {address}"),
        ServerState::Failed { reason } => format!("Сервер не запустился: {reason}"),
        ServerState::Stopped => "Сервер остановлен".to_owned(),
    }
}
