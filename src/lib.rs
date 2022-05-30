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

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PP: u32 = 80;

    let mut scene = Scene::new();
    scene.add_sphere(0.0, 0.0, -1.0, 0.5);
    scene.add_sphere(0.0, -100.5, -1.0, 100.0);

    let camera = Camera::default(ASPECT_RATIO, 2.0);

    let mut img = Render::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PP);
    img.render(&camera, &scene);
    img.save("docs/testimage.png");

    Ok(())
}
