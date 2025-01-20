use miniserde::{ Serialize, Deserialize };

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Stage {
    Compute,
    Vertex,
    Fragment
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Shader {
    pub stage: Stage,
}

impl std::fmt::Display for Stage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stage::Compute => "Compute".fmt(formatter),
            Stage::Vertex => "Vertex".fmt(formatter),
            Stage::Fragment => "Fragment".fmt(formatter),
        }
    }
}
