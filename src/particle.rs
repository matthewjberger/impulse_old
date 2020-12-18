use crate::{Real, Vector3};

pub struct Particle {
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
    /// The amount of damping applied to linear motion.
    /// Damping is required to remove energy added through
    /// numerical instability in the integrator.
    pub damping: Real,
}
