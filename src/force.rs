use crate::{Arena, Body, Handle, Real, Vector3};

pub trait ForceGenerator {
    fn apply(&self, duration: Real, body_handle: Handle, bodies: &mut Arena<Body>);
}

pub struct ForceRegistration {
    pub generator_handle: Handle,
    pub bodies: Vec<Handle>,
}

impl ForceRegistration {
    pub fn new(generator_handle: Handle, bodies: Vec<Handle>) -> Self {
        Self {
            generator_handle,
            bodies,
        }
    }
}

pub struct Gravity {
    pub force: Vector3,
}

impl Gravity {
    pub fn new(force: Vector3) -> Self {
        Self { force }
    }

    pub fn earth_gravity() -> Vector3 {
        Vector3::y() * -9.8
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self::new(Self::earth_gravity())
    }
}

impl ForceGenerator for Gravity {
    fn apply(&self, _duration: Real, body_handle: Handle, bodies: &mut Arena<Body>) {
        let body = match bodies.get_mut(body_handle) {
            Some(body) => body,
            None => return,
        };

        if body.has_infinite_mass() {
            return;
        }
        let force = self.force * body.mass();
        body.add_force(&force);
    }
}

#[derive(Default)]
pub struct Drag {
    pub k1: Real,
    pub k2: Real,
}

impl ForceGenerator for Drag {
    fn apply(&self, _duration: Real, body_handle: Handle, bodies: &mut Arena<Body>) {
        let body = match bodies.get_mut(body_handle) {
            Some(body) => body,
            None => return,
        };

        let mut drag_coefficient = body.velocity.magnitude();
        drag_coefficient = self.k1 * drag_coefficient + self.k2 * drag_coefficient.powi(2);
        let force = body.velocity.normalize() * -drag_coefficient;
        body.add_force(&force);
    }
}

pub struct Spring {
    pub end_body_handle: Handle, // FIXME: Replace this with a handle
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for Spring {
    fn apply(&self, _duration: Real, body_handle: Handle, bodies: &mut Arena<Body>) {
        let end_body_position = {
            let end_body = match bodies.get(self.end_body_handle) {
                Some(end_body) => end_body,
                None => return,
            };
            end_body.position
        };

        let body = match bodies.get_mut(body_handle) {
            Some(body) => body,
            None => return,
        };

        let force = body.position - end_body_position;
        let magnitude = (force.magnitude() - self.rest_length).abs() * self.spring_constant;
        let force = force.normalize() * -magnitude;
        body.add_force(&force);
    }
}

pub struct AnchoredSpring {
    pub anchor: Vector3,
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for AnchoredSpring {
    fn apply(&self, _duration: Real, body_handle: Handle, bodies: &mut Arena<Body>) {
        let body = match bodies.get_mut(body_handle) {
            Some(body) => body,
            None => return,
        };
        let force = body.position - self.anchor;
        let magnitude = (force.magnitude() - self.rest_length).abs() * self.spring_constant;
        let force = force.normalize() * -magnitude;
        body.add_force(&force);
    }
}

pub struct Bungee {
    pub end_body_handle: Handle,
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for Bungee {
    fn apply(&self, _duration: Real, body_handle: Handle, bodies: &mut Arena<Body>) {
        let end_body_position = {
            let end_body = match bodies.get(self.end_body_handle) {
                Some(end_body) => end_body,
                None => return,
            };
            end_body.position
        };

        let body = match bodies.get_mut(body_handle) {
            Some(body) => body,
            None => return,
        };

        let force = body.position - end_body_position;
        let magnitude = force.magnitude();
        if magnitude <= self.rest_length {
            return;
        }
        let magnitude = (self.rest_length - magnitude) * self.spring_constant;
        let force = force.normalize() * -magnitude;
        body.add_force(&force);
    }
}

pub struct AnchoredBungee {
    pub anchor: Vector3,
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for AnchoredBungee {
    fn apply(&self, _duration: Real, body_handle: Handle, bodies: &mut Arena<Body>) {
        let body = match bodies.get_mut(body_handle) {
            Some(body) => body,
            None => return,
        };

        let force = body.position - self.anchor;
        let magnitude = force.magnitude();
        if magnitude <= self.rest_length {
            return;
        }
        let magnitude = (self.rest_length - magnitude) * self.spring_constant;
        let force = force.normalize() * -magnitude;
        body.add_force(&force);
    }
}

pub struct Buoyancy {
    pub max_depth: Real,
    pub volume: Real,
    pub water_height: Real,
    pub liquid_density: Real,
}

impl ForceGenerator for Buoyancy {
    fn apply(&self, _duration: Real, body_handle: Handle, bodies: &mut Arena<Body>) {
        let body = match bodies.get_mut(body_handle) {
            Some(body) => body,
            None => return,
        };

        let depth = body.position.y;
        let out_of_water = depth >= self.water_height + self.max_depth;
        if out_of_water {
            return;
        }
        let mut force = Vector3::zero();

        let at_max_depth = depth <= self.water_height - self.max_depth;
        if at_max_depth {
            force.y = self.liquid_density * self.volume;
        } else {
            // partially submerged
            force.y =
                self.liquid_density * self.volume * (depth - self.max_depth - self.water_height)
                    / 2.0
                    * self.max_depth;
        }

        body.add_force(&force);
    }
}
