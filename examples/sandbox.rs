use impulse::{Body, ForceRegistration, Gravity, PhysicsWorld};
use kiss3d::{camera::ArcBall, light::Light, window::Window};
use na::{Point3, Translation3, UnitQuaternion, Vector3};
use nalgebra as na;

fn main() {
    let mut physics_world = PhysicsWorld::default();

    // Register forces
    let gravity = physics_world
        .force_generators
        .insert(Box::new(Gravity::default()));

    // Register bodies
    let ball = physics_world.bodies.insert(Body {
        inverse_mass: 2.0_f32.recip(),
        damping: 0.99,
        ..Default::default()
    });

    // Link forces to bodies
    physics_world
        .registrations
        .push(ForceRegistration::new(gravity, vec![ball]));

    // Setup scene
    let mut camera = ArcBall::new(Point3::new(10.0, 0.0, 6.0), Point3::origin());
    let mut window = Window::new("Impulse Physics Engine - Sandbox");
    window.set_light(Light::StickToCamera);

    // Add a ground plane for reference
    let mut ground = window.add_quad(10.0, 10.0, 1, 1);
    ground.set_local_translation(Translation3::new(0.0, -2.0, 0.0));
    ground.set_local_rotation(UnitQuaternion::from_axis_angle(
        &Vector3::x_axis(),
        90_f32.to_radians(),
    ));

    let mut spheres = Vec::new();

    while window.render_with_camera(&mut camera) {
        // Fake the last frame's duration
        let last_frame_duration = 20.0_f32.recip();
        physics_world.tick(last_frame_duration);

        for (index, (_handle, body)) in physics_world.bodies.iter().enumerate() {
            let sphere = match spheres.get_mut(index) {
                Some(sphere) => sphere,
                None => {
                    let mut sphere = window.add_sphere(0.5);
                    sphere.set_color(0.0, 1.0, 1.0);
                    spheres.push(sphere);
                    &mut spheres[index]
                }
            };

            sphere.set_local_translation(Translation3::new(
                body.position.x,
                body.position.y,
                body.position.z,
            ));
        }
    }
}
