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
    fn apply(&self, _duration: Real, body: &mut Body) {
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
    fn apply(&self, _duration: Real, body: &mut Body) {
        let mut drag_coefficient = body.velocity.magnitude();
        drag_coefficient = self.k1 * drag_coefficient + self.k2 * drag_coefficient.powi(2);
        let force = body.velocity.normalize() * -drag_coefficient;
        body.add_force(&force);
    }
}

struct Spring {
    pub end: Body, // FIXME: Replace this with a handle
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for Spring {
    fn apply(&self, _duration: Real, body: &mut Body) {
        let force = body.position - self.end.position;
        let magnitude = (force.magnitude() - self.rest_length).abs() * self.spring_constant;
        let force = force.normalize() * -magnitude;
        body.add_force(&force);
    }
}

struct AnchoredSpring {
    pub anchor: Vector3,
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for AnchoredSpring {
    fn apply(&self, _duration: Real, body: &mut Body) {
        let force = body.position - self.anchor;
        let magnitude = (force.magnitude() - self.rest_length).abs() * self.spring_constant;
        let force = force.normalize() * -magnitude;
        body.add_force(&force);
    }
}

struct Bungee {
    pub end: Body, // FIXME: Replace this with a handle
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for Bungee {
    fn apply(&self, _duration: Real, body: &mut Body) {
        let force = body.position - self.end.position;
        let magnitude = force.magnitude();
        if magnitude <= self.rest_length {
            return;
        }
        let magnitude = (self.rest_length - magnitude) * self.spring_constant;
        let force = force.normalize() * -magnitude;
        body.add_force(&force);
    }
}

struct AnchoredBungee {
    pub anchor: Vector3,
    pub spring_constant: Real,
    pub rest_length: Real,
}

impl ForceGenerator for AnchoredBungee {
    fn apply(&self, _duration: Real, body: &mut Body) {
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
    fn apply(&self, _duration: Real, body: &mut Body) {
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
