use crate::settings::computer_name_from;

#[test]
fn the_computer_name_comes_from_computername_then_hostname() {
    let both = |variable: &str| match variable {
        "COMPUTERNAME" => Some(" DESKTOP-1 ".to_owned()),
        "HOSTNAME" => Some("box".to_owned()),
        _ => None,
    };
    let host_only = |variable: &str| (variable == "HOSTNAME").then(|| "box".to_owned());
    let blank = |_: &str| Some("  ".to_owned());

    assert_eq!(computer_name_from(both), "DESKTOP-1");
    assert_eq!(computer_name_from(host_only), "box");
    assert_eq!(computer_name_from(blank), "FastCast");
}
