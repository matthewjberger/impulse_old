use crate::{BodySet, ForceGeneratorSet, ForceRegistration, Real};

#[derive(Default)]
pub struct PhysicsWorld {
    pub bodies: BodySet,
    pub force_generators: ForceGeneratorSet,
    pub registrations: Vec<ForceRegistration>,
}

impl PhysicsWorld {
    pub fn tick(&mut self, duration: Real) {
        for registration in self.registrations.iter() {
            let force_generator = match self.force_generators.get(registration.generator_handle) {
                Some(force_generator) => force_generator,
                None => continue,
            };

            for body_handle in registration.bodies.iter() {
                (*force_generator).apply(duration, *body_handle, &mut self.bodies);
            }
        }

        for (_index, body) in self.bodies.iter_mut() {
            body.integrate(duration);
        }
    }
}
