use ray_tracing_in_a_weekend::{
    camera::Camera,
    materials::{Dielectric, Diffuse, Material, Metal},
    render::Render,
    scene::Scene,
    vector::Color,
};
use std::rc::Rc;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PP: u64 = 80;
    const ZOOM: f64 = 2.0;
    const BOUNCES: u64 = 10;

    let mut scene = Scene::new();
    let mat_ground = get_mat(Materials::DiffuseYellow);
    let mat_center = get_mat(Materials::DielectricClear);
    let mat_left = get_mat(Materials::DielectricClear);
    let mat_right = get_mat(Materials::MetalHighFuzz);

    scene.add_sphere(0.0, 0.0, -1.0, 0.5, mat_center);
    scene.add_sphere(0.0, -100.5, -1.0, 100.0, mat_ground);
    scene.add_sphere(-1.0, 0.0, -1.0, 0.5, mat_left);
    scene.add_sphere(1.0, 0.0, -1.0, 0.5, mat_right);

    let camera = Camera::default(ASPECT_RATIO, ZOOM);

    let mut img = Render::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PP, BOUNCES);
    img.render(&camera, &scene);
    img.save("docs/testimage.png");
}

enum Materials {
    DiffuseYellow,
    DielectricClear,
    DiffusePink,
    MetalLowFuzz,
    MetalHighFuzz,
}

fn get_mat(mat: Materials) -> Rc<dyn Material> {
    match mat {
        Materials::DiffuseYellow => Rc::new(Diffuse::new(Color::new(0.8, 0.8, 0.0))),
        Materials::DiffusePink => Rc::new(Diffuse::new(Color::new(0.7, 0.3, 0.3))),
        Materials::MetalLowFuzz => Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3)),
        Materials::MetalHighFuzz => Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)),
        Materials::DielectricClear => Rc::new(Dielectric::new(1.5)),
    }
}
