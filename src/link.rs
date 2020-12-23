use crate::{Body, BodySet, ContactGenerator, Handle, Real};

pub struct Link {
    pub body_handle: Handle,
    pub other_body_handle: Handle,
}

impl Link {
    pub fn length(&self, bodies: &mut BodySet) -> Real {
        let body = bodies.get(self.body_handle).expect("Failed to get body!");
        let other_body = bodies
            .get(self.other_body_handle)
            .expect("Failed to get body!");
        (body.position - other_body.position).magnitude()
    }
}

struct Cable {
    max_length: Real,
    restitution: Real,
    link: Link,
}
