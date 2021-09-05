pub mod util;

use util::color::Color;

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for row in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", row);
        for column in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                column as f64 / (IMAGE_WIDTH - 1) as f64,
                row as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            pixel_color.write_color();
        }
    }
    eprintln!("\nDone!");
}
