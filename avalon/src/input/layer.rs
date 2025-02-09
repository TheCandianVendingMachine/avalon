use crate::input::{ self, action, context };
use crate::event::Channel;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Layer {
    pub(super) name: String,
    pub(super) context_stack: Vec<context::Context>
}

pub struct ContextBuilder<'layer> {
    layer: &'layer mut Layer,
    allowed_actions: HashSet<action::ActionId>,
    name: Option<String>,
    priority: context::Priority,
    blocking: context::Block
}

impl<'layer> ContextBuilder<'layer> {
    pub fn name(mut self, name: impl Into<String>) -> ContextBuilder<'layer> {
        self.name = Some(name.into());
        self
    }

    pub fn block(mut self, block: context::Block) -> ContextBuilder<'layer> {
        self.blocking = block;
        self
    }

    pub fn priority(mut self, priority: context::Priority) -> ContextBuilder<'layer> {
        self.priority = priority;
        self
    }

    pub fn action(mut self, action: impl Into<action::ActionId>) -> ContextBuilder<'layer> {
        self.allowed_actions.insert(action.into());
        self
    }

    pub fn build(self) -> Channel<action::Action, &'static str> {
        let mut context = context::Context::new(
            self.priority,
            self.blocking,
            self.allowed_actions
        );
        context.name = self.name;
        self.layer.context_stack.push(context);
        self.layer.context_stack.last_mut().unwrap().context_handler()
    }
}

impl Layer {
    pub(super) fn process_actions(&mut self, actions: Vec<input::action::Action>) {
        let mut only_critical = false;
        for context in self.context_stack.iter_mut()
            .rev()
            .take_while_inclusive(|context| if let context::Block::All = context.block { false } else { true }) {
            if only_critical {
                if let context::Priority::Low = context.priority {
                    continue;
                }
            }

            // send slice of actions to context
            context.process(&actions);

            if let context::Block::NonCritical = context.block {
                only_critical = true;
            }
        }
    }

    pub fn context_handler<'layer>(&'layer mut self) -> ContextBuilder<'layer> {
        ContextBuilder {
            layer: self,
            allowed_actions: HashSet::new(),
            name: None,
            priority: context::Priority::Low,
            blocking: context::Block::None
        }
    }
}
