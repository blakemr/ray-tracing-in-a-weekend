#[test]
fn test() -> std::io::Result<()> {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    println!("{:?}", (IMAGE_WIDTH, IMAGE_HEIGHT));

    let mut output: String = String::from("");

    for i in 0..(IMAGE_WIDTH * IMAGE_HEIGHT) {
        let x = i % IMAGE_WIDTH;
        let y = i / IMAGE_WIDTH;

        let r: f64 = x as f64 / IMAGE_WIDTH as f64;
        let g: f64 = y as f64 / IMAGE_HEIGHT as f64;
        let b: f64 = 0.25;

        let ir: i64 = (r * 256.0) as i64;
        let ig: i64 = (g * 256.0) as i64;
        let ib: i64 = (b * 256.0) as i64;

        output.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
    }

    use std::fs::write;
    write("docs/testimage.ppm", output).expect("Unable to write to file.");

    Ok(())
}
