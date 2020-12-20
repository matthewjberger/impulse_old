use crate::{Body, Handle, Real, Vector3};

pub trait ForceGenerator {
    fn apply(&self, duration: Real, body: &mut Body);
}

pub struct ForceRegistration {
    pub generator_handle: Handle,
    pub bodies: Vec<Handle>,
}

#[derive(Default)]
struct Gravity {
    pub force: Vector3,
}

impl ForceGenerator for Gravity {
    fn apply(&self, _: Real, body: &mut Body) {
        if !body.has_finite_mass() {
            return;
        }
        let force = self.force * body.mass();
        body.add_force(&force);
    }
}

#[derive(Default)]
struct Drag {
    pub k1: Real,
    pub k2: Real,
}

impl ForceGenerator for Drag {
    fn apply(&self, _: Real, body: &mut Body) {
        let mut drag_coefficient = body.velocity.magnitude();
        drag_coefficient = self.k1 * drag_coefficient + self.k2 * drag_coefficient.powi(2);
        let force = body.velocity.normalize() * -drag_coefficient;
        body.add_force(&force);
    }
}
