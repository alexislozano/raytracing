use std::fs;
mod vec3;

fn main() {
    let width = 300;
    let height = 200;
    let max_color = 255;

    let mut pic = format!("P3\n{} {}\n{}\n", width, height, max_color);

    let blue = 0;
    for h in 0..height {
        let green = ((1.0 - h as f64 / height as f64) * max_color as f64) as u8;
        for w in 0..width {
            let red = ((w as f64 / width as f64) * max_color as f64) as u8;
            let pixel = format!("{} {} {}", red, green, blue);
            pic = format!("{}{}\n", pic, pixel);
        }
    }

    match fs::write("output.ppm", pic) {
        Err(_) => eprintln!("Could not generate the picture"),
        Ok(_) => (),
    };
}
