use avalon::ecs::component::{ Tag, Component };
use avalon::ecs::Poolable;
use avalon::transform;
use avalon::input::context;
use nalgebra_glm::Vec3;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
    Transform =         1,
    Collider =          2,
    Particle =          3,
    PlayerController =  4,
    Camera =            5,
}

#[derive(Debug, Copy, Clone)]
pub enum MoveState {
    Idle,
    Walk,
    Sprint,
    Slide,
    Jump,
    Fall
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerState {
    state: MoveState = MoveState::Idle,
    enter_time: Instant
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

#[derive(Default, Debug, Copy, Clone)]
pub struct Transform {
    id: u32,
    pub transform: transform::Transform
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Collider {
    id: u32,
    pub hull: Hull = Hull::Sphere { radius: 0.0 },
    pub movement: Movement = Movement::Dynamic
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Particle {
    id: u32,
    pub velocity: Vec3,
    pub acceleration: Vec3
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerController {
    id: u32,
    pub max_speed: f32,
    pub state: PlayerState
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Camera {
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
        /*impl Poolable for $component {
            fn with_handle(handle: Handle) -> Self {
                Self::default()
            }
            fn handle(&self) -> Handle;
        }*/
    }
}

impl_component!(Transform);
impl_component!(Collider);
impl_component!(Particle);
impl_component!(PlayerController);
impl_component!(Camera);
