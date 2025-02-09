use crate::input::context;

pub struct Layer {
    name: Option<String>,
    context_stack: Vec<context::Context>
}
