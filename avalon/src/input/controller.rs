use nalgebra_glm::Vec2;
use std::collections::HashMap;
use std::time::Instant;

use crate::input::event;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Unknown,
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    LeftTrigger,
    RightTrigger,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Touchpad
}

#[derive(Debug, Copy, Clone)]
pub struct Stick {
    direction: Vec2,
    amount: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Trigger {
    amount: f32,
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub pressed_buttons: HashMap<Button, Instant>,
    pub left_stick: Stick,
    pub right_stick: Stick,
    pub left_trigger: Trigger,
    pub right_trigger: Trigger,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            pressed_buttons: HashMap::new(),
            left_stick: Stick::new(),
            right_stick: Stick::new(),
            left_trigger: Trigger::new(),
            right_trigger: Trigger::new(),
        }
    }

    pub fn held(&self) -> Vec<event::Controller> {
        self.pressed_buttons.iter()
            .map(|(button, _)| event::Controller::Button {
                state: event::Binary::Hold,
                button: *button
            })
            .collect()
    }

    pub fn press(&mut self, button: sdl2::controller::Button) -> event::Controller {
        self.pressed_buttons.insert(button.into(), Instant::now());
        event::Controller::Button {
            state: event::Binary::Single,
            button: button.into()
        }
    }
    pub fn release(&mut self, button: sdl2::controller::Button) -> event::Controller {
        self.pressed_buttons.remove(&button.into());
        event::Controller::Button {
            state: event::Binary::Release,
            button: button.into()
        }
    }

    pub fn axis(&mut self, axis: sdl2::controller::Axis, amount: i16) -> event::Controller {
        let amount = (amount as f32) / (i16::MAX as f32);
        match axis {
            sdl2::controller::Axis::LeftX => {
                self.left_stick.direction.x = amount;
                event::Controller::LeftStick(self.left_stick)
            },
            sdl2::controller::Axis::LeftY => {
                self.left_stick.direction.y = amount;
                event::Controller::LeftStick(self.left_stick)
            },
            sdl2::controller::Axis::RightX => {
                self.right_stick.direction.x = amount;
                event::Controller::RightStick(self.right_stick)
            },
            sdl2::controller::Axis::RightY => {
                self.right_stick.direction.y = amount;
                event::Controller::RightStick(self.right_stick)
            },
            sdl2::controller::Axis::TriggerLeft => {
                self.left_trigger.amount = amount;
                if amount > 0.5 {
                    self.pressed_buttons.insert(Button::LeftTrigger, Instant::now());
                } else {
                    self.pressed_buttons.remove(&Button::LeftTrigger);
                }
                event::Controller::LeftTrigger(self.left_trigger)
            },
            sdl2::controller::Axis::TriggerRight => {
                self.right_trigger.amount = amount;
                if amount > 0.5 {
                    self.pressed_buttons.insert(Button::RightTrigger, Instant::now());
                } else {
                    self.pressed_buttons.remove(&Button::RightTrigger);
                }
                event::Controller::RightTrigger(self.right_trigger)
            },
        }
    }
}

impl From<sdl2::controller::Button> for Button {
    fn from(button: sdl2::controller::Button) -> Button {
        match button {
            sdl2::controller::Button::A => Button::A,
            sdl2::controller::Button::B => Button::B,
            sdl2::controller::Button::X => Button::X,
            sdl2::controller::Button::Y => Button::Y,
            sdl2::controller::Button::Back => Button::Back,
            sdl2::controller::Button::Guide => Button::Guide,
            sdl2::controller::Button::Start => Button::Start,
            sdl2::controller::Button::LeftStick => Button::LeftStick,
            sdl2::controller::Button::RightStick => Button::RightStick,
            sdl2::controller::Button::LeftShoulder => Button::LeftShoulder,
            sdl2::controller::Button::RightShoulder => Button::RightShoulder,
            sdl2::controller::Button::DPadUp => Button::DPadUp,
            sdl2::controller::Button::DPadDown => Button::DPadDown,
            sdl2::controller::Button::DPadLeft => Button::DPadLeft,
            sdl2::controller::Button::DPadRight => Button::DPadRight,
            sdl2::controller::Button::Touchpad => Button::Touchpad,
            _ => Button::Unknown,
        }
    }
}

impl Stick {
    pub(super) fn new() -> Stick {
        Stick {
            direction: Vec2::zeros(),
            amount: 0.0
        }
    }
}

impl Trigger {
    pub(super) fn new() -> Trigger {
        Trigger {
            amount: 0.0
        }
    }
}
