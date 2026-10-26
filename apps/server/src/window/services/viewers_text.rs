pub fn viewers_text(viewers: usize) -> String {
    match viewers {
        0 => "ТВ не подключён".to_owned(),
        count => format!("Подключено ТВ: {count}"),
    }
}
