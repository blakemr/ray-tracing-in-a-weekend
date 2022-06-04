use camera::Camera;
use materials::{Dielectric, Diffuse, Metal};
use ray_tracing_in_a_weekend::{utilities::Color, *};
use render::Render;
use scene::Scene;
use std::rc::Rc;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PP: u64 = 80;
    const ZOOM: f64 = 2.0;
    const BOUNCES: u64 = 10;

    let mut scene = Scene::new();
    let mat_ground = Rc::new(Diffuse::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Dielectric::new(1.5));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_diffuse_pink = Rc::new(Diffuse::new(Color::new(0.7, 0.3, 0.3)));
    let mat_metal_low_fuzz = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    scene.add_sphere(0.0, 0.0, -1.0, 0.5, mat_center);
    scene.add_sphere(0.0, -100.5, -1.0, 100.0, mat_ground);
    scene.add_sphere(-1.0, 0.0, -1.0, 0.5, mat_left);
    scene.add_sphere(1.0, 0.0, -1.0, 0.5, mat_right);

    let camera = Camera::default(ASPECT_RATIO, ZOOM);

    let mut img = Render::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PP, BOUNCES);
    img.render(&camera, &scene);
    img.save("docs/testimage.png");
}
