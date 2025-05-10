use std::collections::HashSet;
use crate::input::{ self, event };

pub type ActionId = String;

pub enum Key {
    Scancode(input::keyboard::Scancode),
    Keycode(input::keyboard::Keycode),
}

pub enum Keyboard {
    Press(Key),
    Release(Key),
    Hold(Key),
}

pub enum Mouse {
    Press(input::mouse::Button),
    Release(input::mouse::Button),
    Hold(input::mouse::Button),
    Scroll,
    Move,
}

pub enum Controller {
    Press(input::controller::Button),
    Release(input::controller::Button),
    Hold(input::controller::Button),
    LeftStick,
    RightStick,
    LeftTrigger,
    RightTrigger
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    pub name: ActionId,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    pub(crate) required_events: HashSet<event::Event>,
    action: ActionId
}

#[derive(Debug, Clone)]
pub struct Map {
    pub(crate) mappings: Vec<Mapping>,
}

pub struct MappingBuilder {
    map_builder: MapBuilder,
    action: ActionId,
    required_events: HashSet<event::Event>
}

impl MappingBuilder {
    pub fn key(mut self, key: Keyboard) -> MappingBuilder {
        let event = match key {
            Keyboard::Press(key) => event::Keyboard::Button {
                state: event::Binary::Single,
                key: match key {
                    Key::Scancode(scancode) => scancode,
                    Key::Keycode(keycode) => input::keyboard::Scancode::from_keycode(keycode).unwrap(),
                }
            },
            Keyboard::Release(key) => event::Keyboard::Button {
                state: event::Binary::Release,
                key: match key {
                    Key::Scancode(scancode) => scancode,
                    Key::Keycode(keycode) => input::keyboard::Scancode::from_keycode(keycode).unwrap(),
                }
            },
            Keyboard::Hold(key) => event::Keyboard::Button {
                state: event::Binary::Hold,
                key: match key {
                    Key::Scancode(scancode) => scancode,
                    Key::Keycode(keycode) => input::keyboard::Scancode::from_keycode(keycode).unwrap(),
                }
            },
        };
        self.required_events.insert(event.into());
        self
    }

    pub fn mouse(mut self, mouse: Mouse) -> MappingBuilder {
        let event = match mouse {
            Mouse::Press(button) => event::Mouse::Button {
                state: event::Binary::Single,
                button
            },
            Mouse::Release(button) => event::Mouse::Button {
                state: event::Binary::Release,
                button
            },
            Mouse::Hold(button) => event::Mouse::Button {
                state: event::Binary::Hold,
                button
            },
            Mouse::Move => event::Mouse::Move { position: nalgebra_glm::IVec2::zeros(), direction: nalgebra_glm::IVec2::zeros() },
            Mouse::Scroll => event::Mouse::Scroll { scroll: f32::NAN },
        };
        self.required_events.insert(event.into());
        self
    }

    pub fn controller(mut self, controller: Controller) -> MappingBuilder {
        let event = match controller {
            Controller::Press(button) => event::Controller::Button {
                state: event::Binary::Single,
                button
            },
            Controller::Release(button) => event::Controller::Button {
                state: event::Binary::Release,
                button
            },
            Controller::Hold(button) => event::Controller::Button {
                state: event::Binary::Hold,
                button
            },
            Controller::LeftStick => event::Controller::LeftStick(
                input::controller::Stick::new()
            ),
            Controller::RightStick => event::Controller::RightStick(
                input::controller::Stick::new()
            ),
            Controller::LeftTrigger => event::Controller::LeftTrigger(
                input::controller::Trigger::new()
            ),
            Controller::RightTrigger => event::Controller::RightTrigger(
                input::controller::Trigger::new()
            ),
        };
        self.required_events.insert(event.into());
        self
    }

    pub fn finish(mut self) -> MapBuilder {
        self.map_builder.mappings.push(Mapping {
            action: self.action,
            required_events: self.required_events
        });
        self.map_builder
    }
}

pub struct MapBuilder {
    mappings: Vec<Mapping>
}

impl MapBuilder {
    pub fn map(self, action: impl Into<String>) -> MappingBuilder {
        MappingBuilder {
            map_builder: self,
            action: action.into(),
            required_events: HashSet::new()
        }
    }

    pub fn build(self) -> Map {
        Map {
            mappings: self.mappings
        }
    }
}

impl Map {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> MapBuilder {
        MapBuilder {
            mappings: Vec::new()
        }
    }
}

impl From<&Mapping> for Action {
    fn from(mapping: &Mapping) -> Action {
        Action {
            name: mapping.action.clone()
        }
    }
}

impl From<Mapping> for Action {
    fn from(mapping: Mapping) -> Action {
        Action {
            name: mapping.action
        }
    }
}
