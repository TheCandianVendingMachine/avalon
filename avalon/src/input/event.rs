use nalgebra_glm::IVec2;
use crate::input::{ controller, keyboard, mouse };

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum Keyboard {
    Button { state: Binary, key: keyboard::Scancode }
}

#[derive(Debug, Copy, Clone)]
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
