use nalgebra_glm::IVec2;
use crate::input::{ controller, keyboard, mouse };

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binary {
    Single,
    Double,
    Hold,
    Release
}

#[derive(Debug, Copy, Clone)]
pub enum Controller {
    Button { state: Binary, button: controller::Button },
    LeftStick(controller::Stick),
    RightStick(controller::Stick),
    LeftTrigger(controller::Trigger),
    RightTrigger(controller::Trigger),
}

#[derive(Debug, Copy, Clone)]
pub enum Mouse {
    Button { state: Binary, button: mouse::Button },
    Move { position: IVec2, direction: IVec2 },
    Scroll { scroll: f32 },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Keyboard {
    Button { state: Binary, key: keyboard::Scancode }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    Keyboard(Keyboard),
    Mouse(Mouse),
    Controller(Controller)
}

impl From<Keyboard> for Event {
    fn from(keyboard: Keyboard) -> Event {
        Event::Keyboard(keyboard)
    }
}

impl From<Mouse> for Event {
    fn from(mouse: Mouse) -> Event {
        Event::Mouse(mouse)
    }
}

impl From<Controller> for Event {
    fn from(controller: Controller) -> Event {
        Event::Controller(controller)
    }
}

impl PartialEq for Controller {
    fn eq(&self, rhs: &Controller) -> bool {
        match self {
            Controller::LeftStick(_) => matches!(rhs, Controller::LeftStick(_)),
            Controller::RightStick(_) => matches!(rhs, Controller::RightStick(_)),
            Controller::LeftTrigger(_) => matches!(rhs, Controller::LeftTrigger(_)),
            Controller::RightTrigger(_) => matches!(rhs, Controller::RightTrigger(_)),
            Controller::Button { state, button } => {
                let lhs_state = state;
                let lhs_button = button;
                if let Controller::Button { state, button } = rhs {
                    lhs_state.eq(state) && lhs_button.eq(button)
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Controller {}
impl std::hash::Hash for Controller {
    fn hash<H: std::hash::Hasher>(&self, hash_state: &mut H) {
        match self {
            Controller::Button { state, button } => {
                state.hash(hash_state);
                button.hash(hash_state);
            },
            Controller::LeftTrigger { .. } => 1.hash(hash_state),
            Controller::RightTrigger { .. } => 2.hash(hash_state),
            Controller::LeftStick { .. } => 3.hash(hash_state),
            Controller::RightStick { .. } => 4.hash(hash_state),
        }
    }
}


impl PartialEq for Mouse {
    fn eq(&self, rhs: &Mouse) -> bool {
        match self {
            Mouse::Scroll { .. }  => matches!(rhs, Mouse::Scroll{..}),
            Mouse::Move { .. } => matches!(rhs, Mouse::Move{..}),
            Mouse::Button { state, button } => {
                let lhs_state = state;
                let lhs_button = button;
                if let Mouse::Button { state, button } = rhs {
                    lhs_state.eq(state) && lhs_button.eq(button)
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Mouse {}
impl std::hash::Hash for Mouse {
    fn hash<H: std::hash::Hasher>(&self, hash_state: &mut H) {
        match self {
            Mouse::Button { state, button } => {
                state.hash(hash_state);
                button.hash(hash_state);
            },
            Mouse::Move { .. } => 1.hash(hash_state),
            Mouse::Scroll { .. } => 2.hash(hash_state),
        }
    }
}
