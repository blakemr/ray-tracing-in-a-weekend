pub mod camera;
pub mod hittable;
pub mod ray;
pub mod render;
pub mod scene;
pub mod shapes;
pub mod utilities;

#[test]
fn make_an_image() -> std::io::Result<()> {
    use camera::Camera;
    use render::Render;
    use scene::Scene;

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PP: u64 = 80;
    const ZOOM: f64 = 2.0;
    const BOUNCES: u64 = 50;

    let mut scene = Scene::new();
    scene.add_sphere(0.0, 0.0, -1.0, 0.5);
    scene.add_sphere(0.0, -100.5, -1.0, 100.0);

    let camera = Camera::default(ASPECT_RATIO, ZOOM);

    let mut img = Render::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PP, BOUNCES);
    img.render(&camera, &scene);
    img.save("docs/testimage.png");

    Ok(())
}
