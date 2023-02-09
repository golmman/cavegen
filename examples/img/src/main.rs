use cavegen::model::options::Options;
use image::Rgb;
use image::RgbImage;

fn main() {
    let options = Options {
        width: 1000,
        height: 1000,
    };
    let cave = cavegen::generate(&options);

    let mut img = RgbImage::new(cave.width as u32, cave.height as u32);
    let green = Rgb([0, 255, 0]);

    for i in 0..cave.width * cave.height {
        if cave.data[i] {
            let x = (i % cave.width) as u32;
            let y = (i / cave.width) as u32;
            img.put_pixel(x, y, green);
        }
    }

    img.save("output.png").unwrap();
}
