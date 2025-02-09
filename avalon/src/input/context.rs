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
    name: Option<String>,
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

    pub(super) fn process(&mut self, actions: &[action::Action]) {
        for action in actions {
            if self.allowed_actions.contains(&action.name) {
                // report up
                self.channel.push(Event::new(action.clone()));
            }
        }
        self.dispatcher.tick();
    }

    pub fn context_handler(&mut self) -> Channel<action::Action, &'static str> {
        self.dispatcher.receiver()
    }
}
