use crate::{Arena, Body, BodySet, Handle, Real, Vector3};

/// The contact resolution routine for contacts. One
/// resolver instance can be shared for the whole simulation.
#[derive(Default)]
pub struct ContactResolver {
    pub iterations: u32,
    pub iterations_used: u32,
}

impl ContactResolver {
    /// Resolves a set of particle contacts for both penetration
    /// and velocity.
    ///
    /// Contacts that cannot interact with each other should be
    /// passed to separate calls to resolveContacts, as the
    /// resolution algorithm takes much longer for lots of contacts
    /// than it does for the same number of contacts in small sets.
    pub fn resolve_contacts(&mut self, contacts: &[Contact], duration: Real, bodies: &mut BodySet) {
        let number_of_contacts = contacts.len();
        while self.iterations_used < self.iterations {
            // Find the contact with the largest closing velocity
            let (max_index, max_separating_velocity) = contacts
                .iter()
                .map(|contact| contact.separating_velocity(bodies))
                .enumerate()
                .fold((0, 0.0), |max, (index, velocity)| {
                    if velocity > max.1 {
                        (index, velocity)
                    } else {
                        max
                    }
                });

            if max_index == number_of_contacts {
                break;
            }

            contacts[max_index].resolve(bodies, duration);

            self.iterations_used += 1;
        }
    }
}

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
    pub fn resolve(&self, bodies: &mut BodySet, duration: Real) {
        self.resolve_velocity(bodies, duration);
        self.resolve_interpenetration(bodies, duration);
    }

    fn resolve_velocity(&self, bodies: &mut BodySet, duration: Real) {
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

    pub fn separating_velocity(&self, bodies: &mut BodySet) -> Real {
        let body = bodies
            .get(self.body_handle)
            .expect("Failed to lookup body!");

        let other_body = bodies
            .get(self.other_body_handle)
            .expect("Failed to lookup body!");

        (body.velocity - other_body.velocity).dot(self.normal)
    }

    fn resolve_interpenetration(&self, bodies: &mut BodySet, duration: Real) {
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
            body.position += move_per_inverse_mass * body.inverse_mass;
        };

        {
            let body = bodies
                .get_mut(self.other_body_handle)
                .expect("Failed to lookup body!");
            body.position += move_per_inverse_mass * -body.inverse_mass;
        };
    }
}
