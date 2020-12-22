use crate::{Real, Vector3};

#[derive(Debug, Default, Copy, Clone)]
pub struct Body {
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
    /// The amount of damping applied to linear motion.
    /// Damping is required to remove energy added through
    /// numerical instability in the integrator.
    ///
    /// The damping parameter controls how much velocity is left after the
    /// update. If the damping is zero then the velocity will be reduced to nothing, meaning
    /// that the object couldn’t sustain any motion without a force and would look odd to
    /// the player. A value of 1 means that the object keeps all its velocity (equivalent to no
    /// damping). If you don’t want the object to look like it is experiencing drag, but still
    /// want to use damping to avoid numerical problems, then values slightly less than 1 are
    /// optimal. A value of 0.999 might be perfect, for example.
    pub damping: Real,

    /// Holds the inverse of the mass of the body.
    ///
    /// It is more useful to hold the inverse mass because
    /// integration is simpler, and because in real-time
    /// simulation it is more useful to have objects with
    /// infinite mass (immovable) than zero mass
    /// (completely unstable in numerical simulation).
    pub inverse_mass: Real,

    // Holds the accumulated force to be applied at the next
    // simulation iteration only. This value is zeroed at each
    // integration step.
    pub force_accumulator: Vector3,
}

impl Body {
    pub fn mass(&self) -> Real {
        self.inverse_mass.recip()
    }

    pub fn has_infinite_mass(&self) -> bool {
        self.inverse_mass == 0.0
    }

    pub fn add_force(&mut self, force: &Vector3) {
        self.force_accumulator += force;
    }

    /// Integrates the body forward in time by the given amount.
    /// This function uses a Newton-Euler integration method, which is a
    /// linear approximation to the correct integral. For this reason it
    /// may be inaccurate in some cases.
    pub fn integrate(&mut self, duration: Real) {
        if self.inverse_mass <= 0.0 {
            return;
        }

        // FIXME: Return a real error here instead of panicking
        assert!(duration > 0.0);

        // Update linear position
        self.position += self.velocity * duration;

        // Work out the acceleration from the force
        let mut acceleration = self.acceleration;
        acceleration += self.force_accumulator * self.inverse_mass;

        let drag = duration.powf(self.damping);

        // Update linear velocity from the acceleration
        self.velocity += acceleration * duration * drag;

        // Clear any accumulated forces
        self.force_accumulator = Vector3::zero();
    }
}
