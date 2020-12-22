use crate::{Arena, Body, BodySet, Handle, Real, Vector3};

/// A contact represents two objects in contact
/// Resolving a contact removes their interpenetration, and applies sufficient
/// impulse to keep them apart. Colliding bodies may also rebound.
pub struct Contact {
    pub body_handle: Handle,

    /// A body with infinite mass and a velocity of zero implies a contact with scenery
    pub other_body_handle: Handle,

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

        let impulse_required = separating_velocity > 0.0;
        if !impulse_required {
            // The contact is either separating or stationary
            // so there is no impulse required
            return;
        }

        let (body_acceleration, body_inverse_mass) = {
            let body = bodies
                .get(self.body_handle)
                .expect("Failed to lookup body!");
            (body.acceleration, body.inverse_mass)
        };

        let other_body_inverse_mass = {
            bodies
                .get(self.other_body_handle)
                .expect("Failed to lookup body!")
                .inverse_mass
        };

        let mut new_separating_velocity = -separating_velocity * self.restitution;

        // Check the velocity build-up due to acceleration only
        let acceleration_caused_separation_velocity = body_acceleration.dot(self.normal) * duration;

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
        let total_inverse_mass = body_inverse_mass + other_body_inverse_mass;
        if total_inverse_mass <= 0.0 {
            return;
        }

        let impulse = delta_velocity / total_inverse_mass;

        // The amount of impulse per unit of inverse mass
        let impulse_per_inverse_mass = self.normal * impulse;

        {
            let body = bodies
                .get_mut(self.body_handle)
                .expect("Failed to lookup body!");
            body.velocity += impulse_per_inverse_mass * body.inverse_mass;
        };

        {
            let body = bodies
                .get_mut(self.other_body_handle)
                .expect("Failed to lookup body!");
            body.velocity += impulse_per_inverse_mass * -body.inverse_mass;
        };
    }

    fn separating_velocity(&self, bodies: &mut BodySet) -> Real {
        let body = bodies
            .get(self.body_handle)
            .expect("Failed to lookup body!");

        let other_body = bodies
            .get(self.other_body_handle)
            .expect("Failed to lookup body!");

        (body.velocity - other_body.velocity).dot(self.normal)
    }

    fn resolve_interpenetration(&self, duration: Real, bodies: &mut BodySet) {
        // If we don't have any penetration, skip this step.
        if self.penetration <= 0.0 {
            return;
        }

        // Find the amount of penetration resolution per unit of inverse mass
        let move_per_inverse_mass = {
            // The movement of each object is based on their inverse mass, so
            // total that.
            let body = bodies
                .get(self.body_handle)
                .expect("Failed to lookup body!");
            let other_body = bodies
                .get(self.body_handle)
                .expect("Failed to lookup body!");

            // If all particles have infinite mass, then we do nothing
            let total_inverse_mass = body.inverse_mass + other_body.inverse_mass;
            if total_inverse_mass <= 0.0 {
                return;
            }
            self.normal * (self.penetration / total_inverse_mass)
        };

        // Apply the penetration resolution
        {
            let body = bodies
                .get_mut(self.body_handle)
                .expect("Failed to lookup body!");
            body.position = move_per_inverse_mass * body.inverse_mass;
        };

        {
            let body = bodies
                .get_mut(self.other_body_handle)
                .expect("Failed to lookup body!");
            body.position = move_per_inverse_mass * -body.inverse_mass;
        };
    }
}
