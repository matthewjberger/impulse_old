pub use self::{body::*, contact::*, force::*, link::*, vector::*, world::*};

mod body;
mod contact;
mod force;
mod link;
mod vector;
mod world;

pub type Real = f32;
pub type Arena<T> = generational_arena::Arena<T>;
pub type Handle = generational_arena::Index;
pub type BodySet = Arena<Body>;
pub type ForceGeneratorSet = Arena<Box<dyn ForceGenerator>>;
