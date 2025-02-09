use crate::input::{ self, context };
use itertools::Itertools;

pub struct Layer {
    name: Option<String>,
    context_stack: Vec<context::Context>
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
}
