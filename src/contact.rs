use crate::{Arena, Body, BodySet, Handle, Real, Vector3};

/// A contact represents two objects in contact
/// Resolving a contact removes their interpenetration, and applies sufficient
/// impulse to keep them apart. Colliding bodies may also rebound.
pub struct Contact {
    pub body_handle: Handle,

    /// A value of None implies a contact with scenery
    pub other_body_handle: Option<Handle>,

    /// The normal restitution coefficient at the contact
    pub restitution: Real,

    /// The direction of the contact in world coordinates
    pub normal: Vector3,

    // The depth of penetration at thecontact
    pub penetration: Real,
}

impl Contact {
    fn resolve(&self, duration: Real, bodies: &mut BodySet) {
        self.resolve_velocity(duration, bodies);
        self.resolve_interpenetration(duration, bodies);
    }

    fn resolve_velocity(&self, duration: Real, bodies: &mut BodySet) {
        // Find velocity in the direction of the of the contact
        let separating_velocity = self.separating_velocity(bodies);

        if separating_velocity > 0.0 {
            // The contact is either separating or stationary
            // so there is no impulse required
            return;
        }

        let mut new_separating_velocity = -separating_velocity * self.restitution;

        let body = bodies
            .get(self.body_handle)
            .expect("Failed to lookup body!");

        // Check the velocity build-up due to acceleration only
        let acceleration_caused_velocity = body.acceleration;

        let other_body = match self.other_body_handle {
            Some(handle) => bodies.get(handle),
            None => None,
        };

        let acceleration_caused_separation_velocity =
            acceleration_caused_velocity.dot(self.normal) * duration;

        // If we've got a closing velocity due to acceleration build-up
        // remove it from the new separating velocity
        if acceleration_caused_separation_velocity < 0.0 {
            new_separating_velocity += self.restitution * acceleration_caused_separation_velocity;

            if new_separating_velocity < 0.0 {
                new_separating_velocity = 0.0
            }
        }

        let delta_velocity = new_separating_velocity - separating_velocity;

        // We apply the change in velocity to each object in proportion to their inverse mass
        // Those with lower inverse mass (higher actual mass) get less change in velocity
        let mut total_inverse_mass = body.inverse_mass;
        if let Some(other_body) = other_body {
            total_inverse_mass += other_body.inverse_mass;
        }

        if total_inverse_mass <= 0.0 {
            return;
        }

        let impulse = delta_velocity / total_inverse_mass;

        // The amount of impulse per unit of inverse mass
        let impulse_per_inverse_mass = self.normal * impulse;

        // FIXME: Set body velocities
        // set body velocity here
        let velocity = body.velocity + impulse_per_inverse_mass * body.inverse_mass;
        // set other body velocity here
        // let velocity = other_body.velocity + impulse_per_inverse_mass * body.inverse_mass;
    }

    fn separating_velocity(&self, bodies: &mut BodySet) -> Real {
        let body = bodies
            .get(self.body_handle)
            .expect("Failed to lookup body!");

        let other_body = match self.other_body_handle {
            Some(handle) => bodies.get(handle),
            None => None,
        };

        let mut relative_velocity = body.velocity;
        if let Some(other_body) = other_body {
            relative_velocity -= other_body.velocity;
        }

        relative_velocity.dot(self.normal)
    }

    fn resolve_interpenetration(&self, duration: Real, bodies: &mut BodySet) {
        // If we don't have any penetration, skip this step.
        if self.penetration <= 0.0 {
            return;
        }

        // The movement of each object is based on their inverse mass, so
        // total that.
        let body = bodies
            .get(self.body_handle)
            .expect("Failed to lookup body!");
        let other_body = match self.other_body_handle {
            Some(handle) => bodies.get(handle),
            None => None,
        };
        let mut total_inverse_mass = body.inverse_mass;
        if let Some(other_body) = other_body {
            total_inverse_mass += other_body.inverse_mass;
        }

        // If all particles have infinite mass, then we do nothing
        if total_inverse_mass <= 0.0 {
            return;
        }

        // Find the amount of penetration resolution per unit of inverse mass
        let move_per_inverse_mass = self.normal * (self.penetration / total_inverse_mass);

        // Calculate the the movement amounts
        let mut body_0_movement = move_per_inverse_mass * body.inverse_mass;
        let body_1_movement = match other_body {
            Some(other_body) => move_per_inverse_mass * -other_body.inverse_mass,
            None => Vector3::zero(),
        };

        // FIXME: Set body positions
        // Apply the penetration resolution
        // Set body 0 position = body_0.position + body_0_movement;
        // Set body 1 position = body_1.position + body_0_movement;
    }
}
