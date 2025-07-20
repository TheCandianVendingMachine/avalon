use avalon::ecs::component::{ Tag, Component };
use avalon::ecs::{ Handle, Poolable };
use avalon::transform;
use avalon::input::context;
use nalgebra_glm::Vec3;
use std::time::Instant;

macro_rules! impl_component {
    ($component:tt) => {
        impl Component for $component {
            fn tag() -> impl Tag { Kind::$component }
            fn id(&self) -> u32 { self.id }
        }
        impl Poolable for $component {
            fn with_handle(handle: Handle) -> Self {
                let mut component = Self::default();
                component.id = handle.into();
                component
            }
            fn handle(&self) -> Handle {
                self.id.into()
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
    Transform =         1,
    Collider =          2,
    Particle =          3,
    PlayerController =  4,
    Camera =            5,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveState {
    Idle,
    Walk,
    Sprint,
    Slide,
    JumpStart,
    JumpFree,
    Fall
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerState {
    pub state: MoveState,
    pub enter_time: Instant
}

impl Default for PlayerState {
    fn default() -> PlayerState {
        PlayerState {
            state: MoveState::Idle,
            enter_time: Instant::now()
        }
    }
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
impl_component!(Transform);

#[derive(Default, Debug, Copy, Clone)]
pub struct Collider {
    id: u32,
    pub hull: Hull = Hull::Sphere { radius: 0.0 },
    pub movement: Movement = Movement::Dynamic
}
impl_component!(Collider);

#[derive(Default, Debug, Copy, Clone)]
pub struct Particle {
    id: u32,
    pub velocity: Vec3,
    pub acceleration: Vec3
}
impl_component!(Particle);

#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerController {
    id: u32,
    pub max_speed: f32 = 10.0,
    pub height: f32 = 1.7,
    pub state: PlayerState
}
impl_component!(PlayerController);

#[derive(Default, Debug, Copy, Clone)]
pub struct Camera {
    id: u32,
}
impl_component!(Camera);

impl Tag for Kind {
    fn uid(&self) -> u32 { *self as u32 }
}
