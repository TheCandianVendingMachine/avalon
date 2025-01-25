pub mod packed;
pub mod ngon;

use miniserde::{ Deserialize, Serialize };

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Model {}
