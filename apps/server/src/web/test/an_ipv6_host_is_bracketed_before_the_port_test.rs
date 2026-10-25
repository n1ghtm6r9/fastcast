use crate::web::host_with_port;

#[test]
fn an_ipv6_host_is_bracketed_before_the_port() {
    assert_eq!(host_with_port("::1", 3278), "[::1]:3278");
    assert_eq!(host_with_port("[::1]", 3278), "[::1]:3278");
}
