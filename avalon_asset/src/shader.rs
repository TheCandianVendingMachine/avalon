#[derive(Debug, Copy, Clone)]
pub enum Stage {
    Compute,
    Vertex,
    Fragment
}

#[derive(Debug, Copy, Clone)]
pub struct Shader {
    stage: Stage,
}
