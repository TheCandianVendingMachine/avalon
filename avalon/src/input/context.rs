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
    block: Block,
    priority: Priority,
}

