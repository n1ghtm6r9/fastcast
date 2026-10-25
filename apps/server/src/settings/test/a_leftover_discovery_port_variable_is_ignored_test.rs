use super::environment_with;
use crate::settings::settings_from;

#[test]
fn a_leftover_discovery_port_variable_is_ignored() {
    let settings = settings_from(environment_with(&[("DISCOVERY_PORT", "abc")]));

    assert!(settings.is_ok());
}
