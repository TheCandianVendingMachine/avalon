use crate::input::{ self, api::axis, api::button, api::cursor };

pub enum Input {
    Button {
        button: button::Button,
        timestamp: input::Timestamp
    },
    Axis {
        axis: axis::Axis,
        timestamp: input::Timestamp
    },
    Cursor {
        cursor: cursor::Cursor,
        timestamp: input::Timestamp
    }
}

