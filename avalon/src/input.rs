use std::collections::{ HashSet, HashMap };

use crate::engine;
use crate::event as engine_event;

pub mod action;
pub mod context;
pub mod controller;
pub mod event;
pub mod keyboard;
pub mod layer;
pub mod mouse;

pub type Id = u64;
pub type Timestamp = u64;

struct Controller {
    device: sdl2::controller::GameController,
    controller: controller::Controller,
}

impl Controller {
    fn new(device: sdl2::controller::GameController) -> Controller {
        Controller {
            device,
            controller: controller::Controller::new()
        }
    }
}

pub struct Engine {
    event_channel: engine_event::Channel<sdl2::event::Event, ()>,
    action_map: action::Map,
    events: Vec<event::Event>,
    controller_subsystem: sdl2::GameControllerSubsystem,
    controllers: HashMap<Id, Controller>,
    mouse: mouse::Mouse,
    keyboard: keyboard::Keyboard,
    timestamp: Timestamp,
    last_controller_timestamp: Timestamp,
    last_kbm_timestamp: Timestamp,
    layers: Vec<layer::Layer>,
    pop_layer: bool
}

impl Engine {
    pub fn new(engine: &mut engine::Engine, action_map: action::Map) -> Engine {
        let controller_subsystem = engine.sdl.game_controller().unwrap();
        controller_subsystem.set_event_state(true);

        let mut controllers = HashMap::new();
        for idx in 0..controller_subsystem.num_joysticks().unwrap() {
            if controller_subsystem.is_game_controller(idx) {
                if let Ok(device) = controller_subsystem.open(idx) {
                    controllers.insert(idx.into(), Controller::new(device));
                }
            }
        }

        Engine {
            event_channel: engine.event_listener(),
            action_map,
            controller_subsystem,
            events: Vec::new(),
            controllers,
            mouse: mouse::Mouse::new(),
            keyboard: keyboard::Keyboard::new(),
            timestamp: 0,
            last_controller_timestamp: 0,
            last_kbm_timestamp: 0,
            layers: Vec::new(),
            pop_layer: false,
        }
    }

    fn controller_from_id(&mut self, id: u32) -> &mut Controller {
        self.controllers.get_mut(&id.into()).unwrap()
    }

    pub fn poll(&mut self) {
        self.timestamp += 1;
        self.mouse.direction = nalgebra_glm::IVec2::zeros();
        self.mouse.scroll = 0.0;
        while let Some(event) = self.event_channel.pop() {
            match event.id {
                sdl2::event::Event::KeyDown { keycode, scancode, .. } => {
                    if let Some(keycode) = keycode {
                        self.events.push(self.keyboard.press_keycode(keycode).into());
                    }
                    if let Some(scancode) = scancode {
                        self.events.push(self.keyboard.press_scancode(scancode).into());
                    }
                    self.last_kbm_timestamp = self.timestamp;
                },
                sdl2::event::Event::KeyUp { keycode, scancode, .. } => {
                    if let Some(keycode) = keycode {
                        self.events.push(self.keyboard.release_keycode(keycode).into());
                    }
                    if let Some(scancode) = scancode {
                        self.events.push(self.keyboard.release_scancode(scancode).into());
                    }
                },
                sdl2::event::Event::MouseWheel { direction, precise_y, .. } => {
                    self.events.push(self.mouse.scroll(precise_y, direction).into());
                },
                sdl2::event::Event::MouseMotion { x, y, xrel, yrel, .. } => {
                    self.events.push(self.mouse.motion(x, y, xrel, yrel).into());
                },
                sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                    self.events.push(self.mouse.press(mouse_btn).into());
                    self.last_kbm_timestamp = self.timestamp;
                },
                sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
                    self.events.push(self.mouse.release(mouse_btn).into());
                },
                sdl2::event::Event::ControllerButtonDown { which, button, .. } => {
                    let event = self.controller_from_id(which).controller.press(button);
                    self.events.push(event.into());
                    self.last_controller_timestamp = self.timestamp;
                },
                sdl2::event::Event::ControllerButtonUp { which, button, .. } => {
                    let event = self.controller_from_id(which).controller.release(button);
                    self.events.push(event.into());
                },
                sdl2::event::Event::ControllerAxisMotion { which, axis, value, .. } => {
                    let event = self.controller_from_id(which).controller.axis(axis, value);
                    self.events.push(event.into());
                    self.last_controller_timestamp = self.timestamp;
                },
                sdl2::event::Event::ControllerDeviceAdded { which, .. } => {
                    if self.controller_subsystem.is_game_controller(which) {
                        if let Ok(device) = self.controller_subsystem.open(which) {
                            self.controllers.insert(which.into(), Controller::new(device));
                        }
                    }
                },
                sdl2::event::Event::ControllerDeviceRemoved { which, .. } => {
                    self.controllers.remove(&which.into());
                },
                _ => {}
            }
        }
    }

    pub fn dispatch(&mut self) {
        if self.pop_layer {
            self.layers.pop();
            self.pop_layer = false;
        }

        for (_, controller) in self.controllers.iter() {
            self.events.extend(controller.controller.held().iter().map(|e| *e).map(Into::<event::Event>::into));
        }
        self.events.extend(self.keyboard.held().iter().map(|e| *e).map(Into::<event::Event>::into));
        self.events.extend(self.mouse.held().iter().map(|e| *e).map(Into::<event::Event>::into));

        if self.last_controller_timestamp > self.last_kbm_timestamp {
            // use controller
            self.events.retain(|e| if let event::Event::Controller(_) = e { true } else { false });
        } else {
            // use keyboard
            self.events.retain(|e| if let event::Event::Controller(_) = e { false } else { true });
        }

        let events: HashSet<event::Event> = HashSet::from_iter(self.events.iter().map(|e| *e));

        let mut actions: Vec<(action::Action, Vec<&event::Event>)> = Vec::new();
        for action in self.action_map.mappings.iter() {
            if !action.required_events.is_empty() && action.required_events.is_subset(&events) {
                let triggered_events: Vec<&event::Event> = events.intersection(&action.required_events).collect();
                actions.push((action.into(), triggered_events));
            }
        }

        if let Some(layer) = self.layers.last_mut() {
            layer.process_actions(actions);
        }

        self.events.clear();
    }

    pub fn active_layer(&self) -> Option<&layer::Layer> {
        self.layers.last()
    }

    pub fn active_layer_mut(&mut self) -> Option<&mut layer::Layer> {
        self.layers.last_mut()
    }

    pub fn push_layer(&mut self, name: impl Into<String>) {
        self.layers.push(layer::Layer {
            name: name.into(),
            context_stack: Vec::new()
        });
    }

    pub fn pop_layer(&mut self) {
        self.pop_layer = true;
    }
}
