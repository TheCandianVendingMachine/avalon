use std::collections::HashMap;
use std::time::Instant;

pub use sdl2::keyboard::{ Scancode, Keycode };

use crate::input::event;

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub pressed_keys: HashMap<Scancode, Instant>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            pressed_keys: HashMap::new()
        }
    }

    pub fn held(&self) -> Vec<event::Keyboard> {
        self.pressed_keys.iter()
            .map(|(key, _)| event::Keyboard::Button {
                state: event::Binary::Hold,
                key: *key
            })
            .collect()
    }

    pub fn press_scancode(&mut self, key: Scancode) -> event::Keyboard {
        let _t1 = Instant::now();
        let t0 = self.pressed_keys.insert(key, Instant::now());
        if let Some(_t0) = t0 {
            event::Keyboard::Button {
                state: event::Binary::Single,
                key
            }
        } else {
            event::Keyboard::Button {
                state: event::Binary::Single,
                key
            }
        }
    }

    pub fn press_keycode(&mut self, key: Keycode) -> event::Keyboard {
        if let Some(scancode) = Scancode::from_keycode(key) {
            self.press_scancode(scancode)
        } else {
            event::Keyboard::Button { state: event::Binary::Single, key: Scancode::Power }
        }
    }

    pub fn release_scancode(&mut self, key: Scancode) -> event::Keyboard {
        self.pressed_keys.remove(&key);
        event::Keyboard::Button {
            state: event::Binary::Release,
            key
        }
    }

    pub fn release_keycode(&mut self, key: Keycode) -> event::Keyboard {
        if let Some(scancode) = Scancode::from_keycode(key) {
            self.release_scancode(scancode)
        } else {
            event::Keyboard::Button {
                state: event::Binary::Release,
                key: Scancode::Power
            }
        }
    }
}
