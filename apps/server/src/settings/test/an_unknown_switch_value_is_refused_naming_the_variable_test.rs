use super::environment_with;
use crate::settings::settings_from;

#[test]
fn an_unknown_switch_value_is_refused_naming_the_variable() {
    let refusal = settings_from(environment_with(&[("VIRTUAL_DISPLAY", "maybe")]))
        .expect_err("maybe is neither on nor off");

    assert_eq!(refusal.variable, "VIRTUAL_DISPLAY");
    assert_eq!(refusal.got, "maybe");
    assert_eq!(refusal.expected, "on or off");
}
