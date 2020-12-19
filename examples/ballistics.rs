use impulse::{Particle, Real};
use kiss3d::{camera::ArcBall, light::Light, text::Font, window::Window};
use na::{Point2, Point3, Translation3, UnitQuaternion, Vector3};
use nalgebra as na;

fn main() {
    let mut camera = ArcBall::new(Point3::new(10.0f32, 0.0, 10.0), Point3::origin());

    let mut window = Window::new("Ballistics");
    window.set_light(Light::StickToCamera);
    let font = Font::default();

    let mut ground = window.add_quad(10.0, 10.0, 1, 1);
    ground.set_local_translation(Translation3::new(0.0, -2.0, 0.0));
    ground.set_local_rotation(UnitQuaternion::from_axis_angle(
        &Vector3::x_axis(),
        90_f32.to_radians(),
    ));

    let step: Real = 0.001;
    let mut particle = Particle::default();
    particle.acceleration = impulse::Vector3::new(0.0, -9.8, 0.0);
    particle.inverse_mass = 4_f32.recip();
    let mut sphere = window.add_sphere(0.5);

    while !window.should_close() {
        // for event in window.events().iter() {
        //     match event.value {
        //         WindowEvent::Key(key, Action::Release, _) => {
        //         }
        //         _ => {}
        //     }
        // }

        window.draw_text(
            "Impulse Ballistics Demo",
            &Point2::origin(),
            36.0,
            &font,
            &Point3::new(0.0, 1.0, 1.0),
        );

        window.render_with_camera(&mut camera);

        particle.integrate(step);
        sphere.set_local_translation(Translation3::new(
            particle.position.x,
            particle.position.y,
            particle.position.z,
        ));
    }
}
