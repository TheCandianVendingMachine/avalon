use avalon::ecs::component::{ Tag, Component };
use avalon::transform;
use avalon::input::context;
use nalgebra_glm::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
    Transform =         1,
    Collider =          2,
    Particle =          3,
    PlayerController =  4
}

#[derive(Debug, Copy, Clone)]
pub enum Hull {
    Sphere { radius: f32 },
    Box { min: Vec3, max: Vec3 }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Movement {
    Static,
    Dynamic
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    id: u32,
    transform: transform::Transform
}

#[derive(Debug, Copy, Clone)]
pub struct Collider {
    id: u32,
    hull: Hull,
    movement: Movement
}

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    id: u32,
    velocity: Vec3,
    acceleration: Vec3
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerController {
    id: u32,
}

impl Tag for Kind {
    fn uid(&self) -> u32 { *self as u32 }
}

macro_rules! impl_component {
    ($component:tt) => {
        impl Component for $component {
            fn tag() -> impl Tag { Kind::$component }
            fn id(&self) -> u32 { self.id }
        }
    }
}

impl_component!(Transform);
impl_component!(Collider);
impl_component!(Particle);
impl_component!(PlayerController);
