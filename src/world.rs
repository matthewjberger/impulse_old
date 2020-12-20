use crate::{Arena, Body, ForceGenerator, ForceRegistration, Real};

#[derive(Default)]
pub struct PhysicsWorld {
    pub bodies: Arena<Body>,
    pub force_generators: Arena<Box<dyn ForceGenerator>>,
    pub registrations: Vec<ForceRegistration>,
}

impl PhysicsWorld {
    pub fn tick(&mut self, duration: Real) {
        for registration in self.registrations.iter() {
            let force_generator = match self.force_generators.get(registration.generator_handle) {
                Some(force_generator) => force_generator,
                None => continue,
            };

            for body in registration.bodies.iter() {
                match self.bodies.get_mut(*body) {
                    Some(body) => {
                        (*force_generator).apply(duration, body);
                    }
                    None => continue,
                }
            }
        }

        for (_index, body) in self.bodies.iter_mut() {
            body.integrate(duration);
        }
    }
}
