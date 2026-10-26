use crate::window::Monitor;

pub fn two_monitors() -> Vec<Monitor> {
    vec![
        Monitor {
            index: 0,
            name: "DISPLAY1".to_owned(),
            width: 2560,
            height: 1440,
        },
        Monitor {
            index: 1,
            name: "DISPLAY2".to_owned(),
            width: 1920,
            height: 1080,
        },
    ]
}
