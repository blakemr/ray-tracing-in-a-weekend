pub mod raytracer;

#[test]
fn test() -> std::io::Result<()> {
    use raytracer::make_image;
    make_image()
}
