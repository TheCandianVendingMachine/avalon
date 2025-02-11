use crate::input::event;
use crate::input::action;
use crate::event::{ Dispatcher, Channel, Event };
use std::collections::HashSet;

pub enum Priority {
    Low,
    High
}

pub enum Block {
    None,
    NonCritical,
    All
}

pub struct Context {
    pub(super) name: Option<String>,
    allowed_actions: HashSet<action::ActionId>,
    dispatcher: Dispatcher<action::Action, &'static str>,
    channel: Channel<action::Action, &'static str>,
    pub(super) block: Block,
    pub(super) priority: Priority,
}

impl Context {
    pub fn new(priority: Priority, block: Block, allowed_actions: HashSet<action::ActionId>) -> Context {
        let mut dispatcher = Dispatcher::new();
        let channel = dispatcher.producer();
        Context {
            name: None,
            allowed_actions,
            dispatcher,
            channel,
            block,
            priority
        }
    }

    pub(super) fn process(&mut self, actions: &[(action::Action, Vec<&event::Event>)]) {
        for (action, events) in actions {
            if self.allowed_actions.contains(&action.name) {
                // report up
                let mut action = Event::new(action.clone());
                for event in events {
                    match event {
                        event::Event::Mouse(mouse_event) => {
                            match mouse_event {
                                event::Mouse::Move { position, direction } => {
                                    action.data.store("cursor_x", position.x).unwrap();
                                    action.data.store("cursor_y", position.y).unwrap();

                                    let magnitude = direction.cast::<f32>().magnitude();
                                    let normal_direction: nalgebra_glm::Vec2 = if magnitude == 0.0 {
                                        nalgebra_glm::vec2(0.0, 0.0)
                                    } else {
                                        direction.cast() / magnitude
                                    };
                                    action.data.store("axis_x", normal_direction.x).unwrap();
                                    action.data.store("axis_y", normal_direction.y).unwrap();
                                    action.data.store("axis_magnitude", magnitude).unwrap();
                                },
                                event::Mouse::Button { .. } => {

                                },
                                event::Mouse::Scroll { scroll } => {
                                    action.data.store("scroll", *scroll).unwrap();
                                },
                            }

                        },
                        event::Event::Keyboard(keyboard_event) => {
                            match keyboard_event {
                                event::Keyboard::Button { .. } => {},
                            }
                        },
                        event::Event::Controller(controller_event) => {
                            match controller_event {
                                event::Controller::Button { .. } => {},
                                event::Controller::LeftStick(stick) => {
                                    let magnitude = stick.direction.cast::<f32>().magnitude();
                                    let normal_direction: nalgebra_glm::Vec2 = if magnitude == 0.0 {
                                        nalgebra_glm::vec2(0.0, 0.0)
                                    } else {
                                        stick.direction.cast() / magnitude
                                    };
                                    action.data.store("axis_x", normal_direction.x).unwrap();
                                    action.data.store("axis_y", normal_direction.y).unwrap();
                                    action.data.store("axis_magnitude", magnitude).unwrap();
                                },
                                event::Controller::RightStick(stick) => {
                                    let magnitude = stick.direction.cast::<f32>().magnitude();
                                    let normal_direction: nalgebra_glm::Vec2 = if magnitude == 0.0 {
                                        nalgebra_glm::vec2(0.0, 0.0)
                                    } else {
                                        stick.direction.cast() / magnitude
                                    };
                                    action.data.store("axis_x", normal_direction.x).unwrap();
                                    action.data.store("axis_y", normal_direction.y).unwrap();
                                    action.data.store("axis_magnitude", magnitude).unwrap();
                                },
                                event::Controller::LeftTrigger(trigger) => {
                                    action.data.store("axis_pressure", trigger.amount).unwrap();
                                },
                                event::Controller::RightTrigger(trigger) => {
                                    action.data.store("axis_pressure", trigger.amount).unwrap();
                                },
                            }
                        },
                    }
                }
                self.channel.push(action);
            }
        }
        self.dispatcher.tick();
    }

    pub fn context_handler(&mut self) -> Channel<action::Action, &'static str> {
        self.dispatcher.receiver()
    }
}
