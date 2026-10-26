use crate::stream::server_reply_json;
use serde_json::Value;

#[test]
fn a_name_with_quotes_and_cyrillic_letters_stays_valid_json() {
    let name = "Мой \"ПК\"";

    let reply: Value =
        serde_json::from_str(&server_reply_json(name, 3278)).expect("the reply is valid JSON");

    assert_eq!(reply["type"], "server");
    assert_eq!(reply["name"], name);
    assert_eq!(reply["port"], 3278);
}
