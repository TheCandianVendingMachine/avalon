use std::collections::HashMap;
use std::time::Instant;
use nalgebra_glm::{ IVec2, vec2 };

use crate::input::event;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Button {
    Unknown,
    Left,
    Right,
    Middle,
    X1,
    X2
}

#[derive(Debug, Clone)]
pub struct Mouse {
    pub pressed_buttons: HashMap<Button, Instant>,
    pub position: IVec2,
    pub direction: IVec2,
    pub scroll: f32
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pressed_buttons: HashMap::new(),
            position: IVec2::zeros(),
            direction: IVec2::zeros(),
            scroll: 0.0
        }
    }

    pub fn held(&self) -> Vec<event::Mouse> {
        self.pressed_buttons.iter()
            .map(|(button, _)| event::Mouse::Button {
                state: event::Binary::Hold,
                button: *button
            })
            .collect()
    }

    pub fn press(&mut self, button: sdl2::mouse::MouseButton) -> event::Mouse {
        let _t1 = Instant::now();
        let t0 = self.pressed_buttons.insert(button.into(), Instant::now());
        if let Some(_t0) = t0 {
            event::Mouse::Button {
                state: event::Binary::Single,
                button: button.into()
            }
        } else {
            event::Mouse::Button {
                state: event::Binary::Single,
                button: button.into()
            }
        }
    }
    pub fn release(&mut self, button: sdl2::mouse::MouseButton) -> event::Mouse {
        self.pressed_buttons.remove(&button.into());
        event::Mouse::Button {
            state: event::Binary::Release,
            button: button.into()
        }
    }

    pub fn motion(&mut self, position_x: i32, position_y: i32, direction_x: i32, direction_y: i32) -> event::Mouse {
        self.position = vec2(position_x, position_y);
        self.direction = vec2(direction_x, direction_y);
        event::Mouse::Move {
            position: self.position,
            direction: self.direction
        }
    }

    pub fn scroll(&mut self, scroll: f32, direction: sdl2::mouse::MouseWheelDirection) -> event::Mouse {
        self.scroll += scroll * match direction {
            sdl2::mouse::MouseWheelDirection::Normal => 1.0,
            sdl2::mouse::MouseWheelDirection::Flipped => -1.0,
            sdl2::mouse::MouseWheelDirection::Unknown(_) => 1.0,
        };
        event::Mouse::Scroll {
            scroll: self.scroll
        }
    }
}

impl From<sdl2::mouse::MouseButton> for Button {
    fn from(button: sdl2::mouse::MouseButton) -> Button {
        match button {
            sdl2::mouse::MouseButton::Left => Button::Left,
            sdl2::mouse::MouseButton::Right => Button::Right,
            sdl2::mouse::MouseButton::Middle => Button::Middle,
            sdl2::mouse::MouseButton::X1 => Button::X1,
            sdl2::mouse::MouseButton::X2 => Button::X2,
            _ => Button::Unknown,
        }
    }
}
