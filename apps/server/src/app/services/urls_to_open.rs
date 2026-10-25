use std::net::{IpAddr, SocketAddr};

pub fn urls_to_open(host: &str, port: u16, lan: Option<IpAddr>) -> Vec<String> {
    let localhost = format!("http://localhost:{port}");

    match host.parse::<IpAddr>() {
        Ok(ip) if ip.is_unspecified() => lan
            .map(|lan_ip| format!("http://{}", SocketAddr::new(lan_ip, port)))
            .into_iter()
            .chain([localhost])
            .collect(),
        Ok(ip) if ip.is_loopback() => vec![localhost],
        Ok(ip) => vec![format!("http://{}", SocketAddr::new(ip, port))],
        Err(_) => vec![format!("http://{host}:{port}")],
    }
}
